//! What this provider owns in a Claude Code target, and what it must not touch.
//!
//! Every value here comes from `references/claude-baseline.json`, which records
//! what was verified against the product's own documentation. The baseline is
//! the authority; a test binds these constants back to it, so a baseline that
//! moves and a build that does not is a failure rather than a quiet lie.

use provider_v3::{
    Command, ComponentKind, Declaration, Operation, ProjectionKind, ProjectionProfile, ProviderInfo,
};
use setup_core::digest;

/// The harness this provider configures.
pub const HARNESS_ID: &str = "claude";

/// The provider identity on the wire.
pub const PROVIDER_ID: &str = "claude-setup-system";

/// The product being configured.
pub const PRODUCT: &str = "Claude Code";

/// The documented configuration home. Documentation, never a fallback.
pub const DOCUMENTED_CONFIG_HOME: &str = "~/.claude";

/// The environment variable that names the configuration home.
pub const CONFIG_HOME_ENV: &str = "CLAUDE_CONFIG_DIR";

/// The provider-owned control directory inside a target.
pub const CONTROL_DIRECTORY: &str = ".claude-setup-system";

/// The provider-owned state file inside a target.
pub const STATE_FILE: &str = "NDDEV-CLAUDE-PROVIDER.json";

/// How many backup slots a target keeps.
pub const BACKUP_SLOTS: usize = 10;

/// The projection profile identity a compiler builds against.
pub const PROFILE_ID: &str = "claude/native-and-marketplace/1";

/// The largest file count a bundle may carry.
pub const MAX_FILES: u64 = 8192;

/// The largest byte count a bundle may carry.
pub const MAX_BYTES: u64 = 64 * 1024 * 1024;

/// The bundle format this provider reads.
pub const BUNDLE_FORMAT: &str = "ai-stp-bundle/1";

/// The native identifier namespaces this provider owns inside a target.
///
/// Everything outside this list is a sibling overlay preserved verbatim. The
/// product's own credentials, its project history and its plugin cache are not
/// here, and a mutation that touched them would be writing state the product
/// owns rather than state this provider manages.
pub const NATIVE_NAMESPACES: &[&str] = &[
    "CLAUDE.md",
    "settings.json",
    "skills",
    "agents",
    "commands",
    "hooks",
    ".mcp.json",
];

/// Paths this provider never reads and never writes.
///
/// A backup that captured credentials would put them in a slot on disk, which
/// is why the exclusion is enforced at capture rather than only at write.
pub const NEVER_TOUCH: &[&str] = &[".credentials.json", "projects", "plugins"];

/// Top-level entries that are not part of this target's identity.
///
/// The control directory and the state file are this provider's own bookkeeping;
/// counting them would make an applied operation leave the target different from
/// the identity it just recorded. The never-touch paths are the product's own —
/// it rewrites credentials and session history constantly, and letting that
/// traffic move the identity would make a plan go stale for a change no effect
/// of ours would have overwritten.
#[must_use]
pub fn not_our_identity() -> Vec<&'static str> {
    let mut names = vec![STATE_FILE];
    names.extend_from_slice(NEVER_TOUCH);
    names
}

/// The operations this build declares.
///
/// Claude Code is installed by its own installer, so this provider owns the
/// configuration only. Declaring `software_install` without owning it would let
/// a consumer call an operation that cannot be honoured.
pub const OPERATIONS: &[Operation] = Operation::CORE;

/// The commands this build implements.
///
/// `launch` is absent for the same reason: this provider does not start the
/// product, and a parser that answered the command would be answering for
/// something it does not do.
pub const COMMANDS: &[Command] = Command::CORE;

/// The operating systems this build runs on.
pub const SUPPORTED_OS: &[&str] = &["linux", "macos", "windows"];

/// The architectures this build runs on.
pub const SUPPORTED_ARCH: &[&str] = &["x86_64", "arm64"];

/// The permission profiles this provider can apply.
pub const PERMISSION_PROFILES: &[&str] = &["default"];

/// The exact provider-kit revision this build was compiled against.
///
/// Embedded at compile time so the running binary reports the kit it was built
/// with, not whatever kit happens to sit beside it now.
pub const KIT_IDENTITY: &str = include_str!("../../../provider-kit/v3/KIT-IDENTITY.json");

/// A digest of this build's own manifest.
///
/// The contract is explicit that the release digest must not come from
/// `provider-info` — an artifact hashing itself proves nothing. This is a
/// different value: an independent statement of what this build *is*, which the
/// consumer records beside the release digest it verified separately.
///
/// # Errors
///
/// Propagates a canonicalization refusal.
pub fn build_digest() -> provider_v3::Result<String> {
    let kit: serde_json::Value = serde_json::from_str(KIT_IDENTITY).map_err(|source| {
        provider_v3::Error::declaration(format!(
            "the vendored kit identity is unreadable: {source}"
        ))
    })?;
    let manifest = serde_json::json!({
        "provider_id": PROVIDER_ID,
        "provider_version": env!("CARGO_PKG_VERSION"),
        "protocol_version": provider_v3::PROTOCOL_VERSION,
        "harness_id": HARNESS_ID,
        "kit_aggregate_digest": kit["aggregate_digest"],
    });
    digest::of_canonical_json(&manifest)
        .map_err(|source| provider_v3::Error::declaration(source.detail()))
}

/// The projection profile this build declares.
///
/// # Errors
///
/// Propagates a declaration refusal.
pub fn projection_profile() -> provider_v3::Result<ProjectionProfile> {
    ProjectionProfile::new(
        PROFILE_ID,
        &[
            ComponentKind::Instruction,
            ComponentKind::Skill,
            ComponentKind::Agent,
            ComponentKind::Command,
            ComponentKind::Hook,
            ComponentKind::Mcp,
            ComponentKind::Plugin,
            ComponentKind::Setting,
        ],
        &[
            ProjectionKind::NativeFiles,
            ProjectionKind::Marketplace,
            ProjectionKind::Plugin,
        ],
        NATIVE_NAMESPACES,
        &[BUNDLE_FORMAT],
        MAX_FILES,
        MAX_BYTES,
    )
}

/// The complete `provider-info` answer for this build.
///
/// # Errors
///
/// Propagates a declaration refusal.
pub fn provider_info() -> provider_v3::Result<ProviderInfo> {
    let build_digest = build_digest()?;
    ProviderInfo::declare(Declaration {
        provider_id: PROVIDER_ID,
        harness_id: HARNESS_ID,
        provider_version: env!("CARGO_PKG_VERSION"),
        provider_build_digest: &build_digest,
        commands: COMMANDS,
        operations: OPERATIONS,
        supported_os: SUPPORTED_OS,
        supported_arch: SUPPORTED_ARCH,
        permission_profiles: PERMISSION_PROFILES,
        projection_profile: projection_profile()?,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    fn baseline() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/claude-baseline.json");
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
    }

    #[test]
    fn the_config_home_and_environment_variable_match_the_verified_baseline() {
        let baseline = baseline();
        assert_eq!(baseline["config_dir"], DOCUMENTED_CONFIG_HOME);
        assert_eq!(baseline["config_dir_env"], CONFIG_HOME_ENV);
    }

    #[test]
    fn nothing_the_baseline_marks_never_touch_is_a_namespace_we_claim() {
        let baseline = baseline();
        for entry in baseline["never_touch"].as_array().unwrap() {
            let name = entry.as_str().unwrap();
            assert!(
                !NATIVE_NAMESPACES.contains(&name),
                "{name} is marked never_touch but claimed as ours"
            );
        }
    }

    #[test]
    fn the_settings_file_we_manage_is_the_one_the_baseline_names() {
        assert!(NATIVE_NAMESPACES.contains(&baseline()["settings_file"].as_str().unwrap()));
    }

    #[test]
    fn the_build_digest_is_reproducible_and_binds_the_vendored_kit() {
        let once = build_digest().unwrap();
        assert_eq!(once, build_digest().unwrap());
        assert!(once.starts_with("sha256:"));

        let kit: serde_json::Value = serde_json::from_str(KIT_IDENTITY).unwrap();
        assert!(
            kit["aggregate_digest"]
                .as_str()
                .unwrap()
                .starts_with("sha256:")
        );
        assert_eq!(kit["protocol_version"], provider_v3::PROTOCOL_VERSION);
    }

    #[test]
    fn the_answer_declares_only_what_this_build_actually_does() {
        let info = provider_info().unwrap();
        assert_eq!(info.provider_id, PROVIDER_ID);
        assert_eq!(info.harness_id, HARNESS_ID);
        assert_eq!(info.protocol_version, 3);

        // No software lifecycle and no launch: this provider owns configuration.
        assert!(!info.declares(Operation::SoftwareInstall));
        assert!(!info.declares(Operation::SoftwareUpdate));
        assert!(!info.declares(Operation::SoftwareRemove));
        assert!(!info.declares(Operation::Launch));
        assert!(!info.supported_commands.iter().any(|c| c == "launch"));

        for operation in Operation::CORE {
            assert!(info.declares(*operation), "{operation} must be declared");
        }
    }

    #[test]
    fn this_build_declares_the_host_it_is_running_on() {
        assert!(provider_info().unwrap().supports_this_host());
    }

    #[test]
    fn the_profile_digest_changes_if_the_declared_surface_changes() {
        let declared = projection_profile().unwrap();
        let narrower = ProjectionProfile::new(
            PROFILE_ID,
            &[ComponentKind::Instruction],
            &[ProjectionKind::NativeFiles],
            NATIVE_NAMESPACES,
            &[BUNDLE_FORMAT],
            MAX_FILES,
            MAX_BYTES,
        )
        .unwrap();
        assert_ne!(declared.digest, narrower.digest);
    }
}
