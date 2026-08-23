//! The provider commands, answered through the kernel and nothing else.
//!
//! Each handler is small on purpose. The decisions that matter — what a target
//! is, when a lock is held, what a journal means, which backup is the last one —
//! belong to [`setup_core`], and repeating any of them here would create a
//! second answer that could disagree with the first.
//!
//! # What this build applies, and what it refuses
//!
//! `backup`, `restore` and `remove` need no bundle: they read the target, a
//! backup slot, or the provider's own state. They are implemented.
//!
//! `install` and `replace` materialize bundle contents, which needs a reader for
//! the bundle format. Until that exists they refuse with a detail saying so.
//! They stay *declared* because the contract requires all five core operations
//! in `provider-info`, and a refusal that names the real limitation is honest in
//! a way that a narrowed declaration could not be.

use std::fs;
use std::path::Path;
use std::time::SystemTime;

use provider_v3::argv::{Bundle, Invocation, PlanRequest};
use provider_v3::plan::{PlanArtifact, PlanInputs};
use provider_v3::{Error, Operation, Result, WireReason};
use setup_core::backup::{BackupRef, Pool, SLOT_SCHEMA, SlotRecord};
use setup_core::journal::{JOURNAL_SCHEMA, Journal, Phase};
use setup_core::stamp::{DriftState, ProviderState, STATE_SCHEMA, StateReading};
use setup_core::target::Target;
use setup_core::{digest, lock};

use crate::expiry;
use crate::harness;

/// Answer one parsed invocation.
///
/// # Errors
///
/// Every failure is a typed refusal the caller prints as a reason plus a detail.
pub fn dispatch(invocation: Invocation) -> Result<serde_json::Value> {
    match invocation {
        Invocation::ProviderInfo => {
            let info = harness::provider_info()?;
            serde_json::to_value(info).map_err(|source| {
                Error::declaration(format!("provider-info cannot be encoded: {source}"))
            })
        }
        Invocation::Status { target } => status(&target),
        Invocation::ValidateBundle { bundle, .. } => validate_bundle(&bundle),
        Invocation::PlanOperation { target, request } => plan(&target, &request),
        Invocation::ApplyOperation {
            target,
            plan_path,
            plan_digest,
            ..
        } => apply(&target, &plan_path, &plan_digest),
        Invocation::RecoverOperation { target } => recover(&target),
        Invocation::Launch { .. } => Err(Error::refuse(
            WireReason::UnsupportedOperation,
            "this provider owns the configuration only and does not start the product",
        )),
    }
}

fn open(target: &Path) -> Result<(Target, std::path::PathBuf, Pool)> {
    let resolved = Target::resolve(target, harness::CONTROL_DIRECTORY)?;
    let control = resolved.ensure_control_directory()?;
    let pool = Pool::open(&control, harness::BACKUP_SLOTS)?;
    Ok((resolved, control, pool))
}

/// Report the target without changing it, including a schema this build cannot write.
fn status(target: &Path) -> Result<serde_json::Value> {
    let (resolved, control, pool) = open(target)?;
    let identity = resolved.identity_digest_excluding(&harness::not_our_identity())?;
    let journal = Journal::read(&control).ok().flatten();

    let state = match ProviderState::read(resolved.root(), harness::STATE_FILE)? {
        StateReading::Absent => serde_json::json!({ "present": false }),
        StateReading::ForeignSchema { found_schema } => serde_json::json!({
            "present": true,
            "readable": false,
            "found_schema": found_schema,
            "detail": "a schema this build does not write; status never migrates it",
        }),
        StateReading::Current(current) => {
            let drift = if current.target_identity_digest == identity {
                DriftState::Clean
            } else {
                DriftState::LocalDrift
            };
            serde_json::json!({
                "present": true,
                "readable": true,
                "setup_stable_id": current.setup_stable_id,
                "setup_version": current.setup_version,
                "operation_id": current.operation_id,
                "backup_ref": current.backup_ref,
                "recorded_identity": current.target_identity_digest,
                "drift_state": drift,
            })
        }
    };

    Ok(serde_json::json!({
        "state": "verified",
        "protocol_version": provider_v3::PROTOCOL_VERSION,
        "provider_id": harness::PROVIDER_ID,
        "harness_id": harness::HARNESS_ID,
        "canonical_target": resolved.root().to_string_lossy(),
        "target_identity_digest": identity,
        "provider_state": state,
        "journal": journal.map(|entry| serde_json::json!({
            "phase": entry.phase.as_str(),
            "operation": entry.operation,
            "operation_id": entry.operation_id,
        })),
        "backups": pool.list()?.iter().map(|record| serde_json::json!({
            "backup_ref": record.backup_ref.as_str(),
            "operation": record.operation,
            "setup_id": record.setup_id,
        })).collect::<Vec<_>>(),
    }))
}

/// Check a bundle's identity against the bytes the caller pointed at.
fn validate_bundle(bundle: &Bundle) -> Result<serde_json::Value> {
    let binding = &bundle.binding;
    if binding.bundle_format != harness::BUNDLE_FORMAT {
        return Ok(provider_v3::plan::bundle_rejected(
            binding,
            WireReason::UnsupportedBundleFormat,
        ));
    }

    let Ok(metadata) = fs::symlink_metadata(&bundle.path) else {
        return Ok(provider_v3::plan::bundle_rejected(
            binding,
            WireReason::DigestMismatch,
        ));
    };
    if metadata.is_symlink() {
        return Ok(provider_v3::plan::bundle_rejected(
            binding,
            WireReason::LinkNotAllowed,
        ));
    }
    if !metadata.is_file() {
        return Ok(provider_v3::plan::bundle_rejected(
            binding,
            WireReason::SpecialFileNotAllowed,
        ));
    }
    if metadata.len() != binding.bundle_size {
        return Ok(provider_v3::plan::bundle_rejected(
            binding,
            WireReason::DigestMismatch,
        ));
    }
    if metadata.len() > harness::MAX_BYTES {
        return Ok(provider_v3::plan::bundle_rejected(
            binding,
            WireReason::LimitExceeded,
        ));
    }
    if digest::of_file(&bundle.path)? != binding.artifact_digest {
        return Ok(provider_v3::plan::bundle_rejected(
            binding,
            WireReason::DigestMismatch,
        ));
    }
    Ok(provider_v3::plan::bundle_accepted(binding))
}

/// Produce a plan without touching the target.
fn plan(target: &Path, request: &PlanRequest) -> Result<serde_json::Value> {
    let (resolved, control, pool) = open(target)?;
    setup_core::journal::require_clean_for_planning(
        &control,
        &control.join("transaction"),
        &pool.partial_slots()?,
    )?;

    let identity = resolved.identity_digest_excluding(&harness::not_our_identity())?;
    let profile = harness::projection_profile()?;
    let build_digest = harness::build_digest()?;

    let (effects, backup_ref, restore_target_digest) = match request.operation {
        Operation::Backup => (
            vec![format!(
                "capture {} into a new backup slot",
                resolved.root().display()
            )],
            None,
            None,
        ),
        Operation::Restore => {
            let record = chosen_backup(&pool, request.backup_ref.as_deref())?;
            let payload = pool.payload_of(&record.backup_ref)?;
            (
                vec![
                    format!("capture the current target before restoring"),
                    format!("restore the target from {}", record.backup_ref.as_str()),
                ],
                Some(record.backup_ref.as_str().to_owned()),
                Some(digest::of_tree(&payload)?),
            )
        }
        Operation::Remove => (
            vec![
                "capture the current target before removing".to_owned(),
                "withdraw every file this provider owns".to_owned(),
            ],
            None,
            None,
        ),
        Operation::Install | Operation::Replace => {
            return Err(Error::refuse(
                WireReason::ProviderUnavailable,
                format!(
                    "{} needs a reader for {}, which this build does not carry yet",
                    request.operation,
                    harness::BUNDLE_FORMAT
                ),
            ));
        }
        other => {
            return Err(Error::refuse(
                WireReason::UnsupportedOperation,
                format!("{other} is not declared by this provider"),
            ));
        }
    };

    PlanArtifact::new(PlanInputs {
        provider_id: harness::PROVIDER_ID,
        provider_version: env!("CARGO_PKG_VERSION"),
        provider_build_digest: &build_digest,
        provider_release_digest: &request.provider_release_digest,
        operation_id: &request.operation_id,
        operation: request.operation,
        canonical_target: &resolved.root().to_string_lossy(),
        expected_target_digest: &identity,
        projection_profile_digest: &profile.digest,
        bundle: request.bundle.as_ref().map(|bundle| bundle.binding.clone()),
        backup_ref,
        restore_target_digest,
        permission_profile: request.permission_profile.clone(),
        expires_at: &request.expires_at,
        effects,
    })?
    .into_response()
}

/// The backup a restore names, or the newest when it names none.
fn chosen_backup(pool: &Pool, requested: Option<&str>) -> Result<SlotRecord> {
    match requested {
        Some(text) => {
            let reference = BackupRef::parse(text)?;
            pool.list()?
                .into_iter()
                .find(|record| record.backup_ref == reference)
                .ok_or_else(|| {
                    Error::refuse(
                        WireReason::ProviderUnavailable,
                        format!("{text} is not a completed backup of this target"),
                    )
                })
        }
        None => pool.latest()?.ok_or_else(|| {
            Error::refuse(
                WireReason::ProviderUnavailable,
                "this target has no backup to restore",
            )
        }),
    }
}

/// Apply one exact plan under the target lock.
fn apply(target: &Path, plan_path: &Path, plan_digest: &str) -> Result<serde_json::Value> {
    let artifact = load_plan(plan_path, plan_digest)?;
    let operation = operation_of(&artifact)?;
    let expires_at = string_field(&artifact, "expires_at")?;
    if expiry::has_expired(&expires_at, SystemTime::now()) {
        return Err(Error::refuse(
            WireReason::Stale,
            "this plan expired before it was applied; no effect was made",
        ));
    }

    let (resolved, control, pool) = open(target)?;
    let mut guard = setup_core::lock::TargetLock::acquire(&control)?;
    guard.annotate(&format!(
        "{} {}",
        harness::PROVIDER_ID,
        string_field(&artifact, "operation_id")?
    ))?;

    // Re-check after the lock: everything observed before it could have moved.
    let identity = resolved.identity_digest_excluding(&harness::not_our_identity())?;
    if identity != string_field(&artifact, "expected_target_digest")? {
        return Err(Error::refuse(
            WireReason::Stale,
            "the target changed after the lock was taken; no effect was made",
        ));
    }
    setup_core::journal::require_clean_for_planning(
        &control,
        &control.join("transaction"),
        &pool.partial_slots()?,
    )?;

    let operation_id = string_field(&artifact, "operation_id")?;
    let captured = pool.capture(resolved.root(), &excluded(), |backup_ref| SlotRecord {
        schema_version: SLOT_SCHEMA,
        backup_ref,
        operation: operation.as_str().to_owned(),
        operation_id: operation_id.clone(),
        target_identity_digest: identity.clone(),
        setup_id: None,
    })?;

    let journal = Journal {
        schema_version: JOURNAL_SCHEMA,
        phase: Phase::Prepared,
        operation_id: operation_id.clone(),
        operation: operation.as_str().to_owned(),
        plan_digest: plan_digest.to_owned(),
        target_precondition_digest: identity.clone(),
        backup_ref: Some(captured.backup_ref.as_str().to_owned()),
    }
    .publish_prepared(&control)?;

    let effect = match operation {
        // The capture above *is* the effect. Nothing else is written.
        Operation::Backup => Ok(()),
        Operation::Restore => restore_from(&resolved, &pool, &artifact),
        Operation::Remove => remove_managed(&resolved),
        other => Err(Error::refuse(
            WireReason::ProviderUnavailable,
            format!("{other} is planned but not applied by this build"),
        )),
    };

    // On failure the journal stays in `prepared`, which is what makes the
    // interruption legible: recovery restores the captured pre-operation target.
    effect?;

    let after = resolved.identity_digest_excluding(&harness::not_our_identity())?;
    write_state(&resolved, &artifact, &identity, &after, &captured)?;
    journal.promote_to_committed(&control)?;
    Journal::clear(&control)?;

    Ok(serde_json::json!({
        "state": "verified",
        "plan_digest": plan_digest,
        "expected_target_digest": identity,
        "target_identity_digest": after,
        "backup_ref": captured.backup_ref.as_str(),
    }))
}

/// Resolve an interrupted operation from its journal.
fn recover(target: &Path) -> Result<serde_json::Value> {
    let (resolved, control, pool) = open(target)?;
    let _guard = setup_core::lock::TargetLock::acquire(&control)?;

    let Some(journal) = Journal::read(&control)? else {
        return Ok(serde_json::json!({
            "state": "verified",
            "recovered": false,
            "detail": "no journal is published; there is nothing to resolve",
        }));
    };

    match journal.phase {
        Phase::Prepared => {
            // The effect may be partial. Return the exact pre-operation target.
            let Some(reference) = journal.backup_ref.as_deref() else {
                return Err(Error::refuse(
                    WireReason::RecoveryRequired,
                    "the journal names no backup, so the pre-operation target cannot be restored",
                ));
            };
            let backup_ref = BackupRef::parse(reference)?;
            let payload = pool.payload_of(&backup_ref)?;
            replace_managed_from(&resolved, &payload)?;
            Journal::clear(&control)?;
            Ok(serde_json::json!({
                "state": "verified",
                "recovered": true,
                "phase": Phase::Prepared.as_str(),
                "restored_from": reference,
                "target_identity_digest": resolved.identity_digest_excluding(&harness::not_our_identity())?,
            }))
        }
        Phase::Committed => {
            // The effect is complete. Verify and clear the tails only.
            Journal::clear(&control)?;
            Ok(serde_json::json!({
                "state": "verified",
                "recovered": true,
                "phase": Phase::Committed.as_str(),
                "target_identity_digest": resolved.identity_digest_excluding(&harness::not_our_identity())?,
            }))
        }
    }
}

fn excluded() -> Vec<&'static str> {
    let mut names = vec![harness::CONTROL_DIRECTORY];
    names.extend_from_slice(harness::NEVER_TOUCH);
    names
}

fn restore_from(target: &Target, pool: &Pool, artifact: &serde_json::Value) -> Result<()> {
    let reference = artifact
        .get("backup_ref")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            Error::refuse(WireReason::ProviderUnavailable, "the plan names no backup")
        })?;
    let payload = pool.payload_of(&BackupRef::parse(reference)?)?;
    replace_managed_from(target, &payload)
}

/// Replace this provider's namespaces from a captured tree.
///
/// Only the namespaces this provider owns are removed and rewritten. A sibling
/// overlay the product or the owner put in the target survives, because a
/// restore that also reverted files this provider never wrote would be undoing
/// someone else's work.
fn replace_managed_from(target: &Target, payload: &Path) -> Result<()> {
    for namespace in harness::NATIVE_NAMESPACES {
        let destination = target.root().join(namespace);
        remove_path(&destination)?;
        let source = payload.join(namespace);
        if !source.exists() {
            continue;
        }
        if source.is_dir() {
            setup_core::backup::copy_tree(&source, &destination, &[])?;
        } else {
            let bytes = fs::read(&source).map_err(|error| {
                setup_core::Error::new(
                    setup_core::ReasonCode::StateUnavailable,
                    format!("cannot read {}", source.display()),
                )
                .with_source(error)
            })?;
            lock::atomic_write(&destination, &bytes)?;
        }
    }
    Ok(())
}

fn remove_managed(target: &Target) -> Result<()> {
    for namespace in harness::NATIVE_NAMESPACES {
        remove_path(&target.root().join(namespace))?;
    }
    Ok(())
}

fn remove_path(path: &Path) -> Result<()> {
    let Ok(metadata) = fs::symlink_metadata(path) else {
        return Ok(());
    };
    let outcome = if metadata.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    };
    outcome.map_err(|error| {
        Error::from(
            setup_core::Error::new(
                setup_core::ReasonCode::StateUnavailable,
                format!("cannot remove {}", path.display()),
            )
            .with_source(error),
        )
    })
}

fn write_state(
    target: &Target,
    artifact: &serde_json::Value,
    before: &str,
    after: &str,
    captured: &SlotRecord,
) -> Result<()> {
    let previous = match ProviderState::read(target.root(), harness::STATE_FILE)? {
        StateReading::Current(current) => Some(current.target_identity_digest),
        _ => None,
    };
    ProviderState {
        state_schema: STATE_SCHEMA,
        protocol_version: provider_v3::PROTOCOL_VERSION,
        provider_id: harness::PROVIDER_ID.to_owned(),
        provider_version: env!("CARGO_PKG_VERSION").to_owned(),
        provider_build_digest: harness::build_digest()?,
        provider_release_digest: artifact
            .get("provider_release_digest")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        harness_id: harness::HARNESS_ID.to_owned(),
        canonical_target: target.root().to_string_lossy().into_owned(),
        target_identity_digest: after.to_owned(),
        setup_stable_id: None,
        setup_version: None,
        setup_version_passport_digest: None,
        setup_definition_digest: None,
        component_refs: Vec::new(),
        bundle_format: None,
        bundle_digest: None,
        artifact_digest: None,
        projection_profile_digest: Some(harness::projection_profile()?.digest),
        provider_plan_digest: artifact
            .get("plan_digest")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        operation_id: string_field(artifact, "operation_id")?,
        target_precondition_digest: before.to_owned(),
        native_ownership: harness::NATIVE_NAMESPACES
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        backup_ref: Some(captured.backup_ref.as_str().to_owned()),
        previous_verified_identity: previous,
        drift_state: DriftState::Clean,
    }
    .write(target.root(), harness::STATE_FILE)
    .map_err(Error::from)
}

fn load_plan(path: &Path, expected_digest: &str) -> Result<serde_json::Value> {
    let bytes = fs::read(path).map_err(|error| {
        Error::refuse(
            WireReason::ProviderUnavailable,
            format!(
                "the approved plan at {} cannot be read: {error}",
                path.display()
            ),
        )
    })?;
    let artifact: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| {
        Error::refuse(
            WireReason::ProviderUnavailable,
            format!("the approved plan is not JSON: {error}"),
        )
    })?;
    let actual = digest::of_domain_canonical_json(provider_v3::PLAN_DOMAIN, &artifact)?;
    if actual != expected_digest {
        return Err(Error::refuse(
            WireReason::DigestMismatch,
            "the approved plan artifact has another digest; no effect was made",
        ));
    }
    Ok(artifact)
}

fn operation_of(artifact: &serde_json::Value) -> Result<Operation> {
    let name = string_field(artifact, "operation")?;
    Operation::parse(&name).ok_or_else(|| {
        Error::refuse(
            WireReason::UnsupportedOperation,
            format!("{name:?} is not an operation this protocol defines"),
        )
    })
}

fn string_field(artifact: &serde_json::Value, name: &str) -> Result<String> {
    artifact
        .get(name)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| {
            Error::refuse(
                WireReason::ProviderUnavailable,
                format!("the plan artifact has no {name}"),
            )
        })
}
