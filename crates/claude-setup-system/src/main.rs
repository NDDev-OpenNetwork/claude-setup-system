//! The Claude Code setup system.
//!
//! One binary, two surfaces. The ai-stp provider commands are what the consumer
//! invokes; the human commands are what the owner types. Both reach the target
//! through [`setup_core`], and neither has a shortcut around it.
//!
//! This harness owns the configuration only. Claude Code installs itself, so
//! declaring a software lifecycle here would let a consumer call an operation
//! that cannot be honoured.
//!
//! # Reading the exit code
//!
//! `0` — the command answered. For a wire command the answer is one JSON object
//! on stdout, including a refusal that names a reason: a refusal is an answer,
//! and the consumer parses it.
//!
//! `1` — the invocation itself was not usable, or this build declared something
//! the contract does not permit. Nothing was written.

mod expiry;
mod harness;
mod wire;

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    match arguments.first().map(String::as_str) {
        Some("--version") => {
            println!("{} {}", harness::PROVIDER_ID, env!("CARGO_PKG_VERSION"));
            return ExitCode::SUCCESS;
        }
        Some("--help") | None => {
            print_help();
            return ExitCode::SUCCESS;
        }
        _ => {}
    }

    match provider_v3::argv::parse(arguments) {
        Ok(invocation) => match wire::dispatch(invocation) {
            Ok(answer) => {
                println!("{answer}");
                ExitCode::SUCCESS
            }
            Err(error) => {
                if let Some(reason) = error.reason() {
                    // A refusal is an answer: it goes to stdout as JSON so the
                    // consumer matches on the reason rather than parsing a message.
                    println!(
                        "{}",
                        serde_json::json!({
                            "state": "refused",
                            "rejected": true,
                            "reason": reason.as_str(),
                            "detail": error.detail(),
                        })
                    );
                    ExitCode::SUCCESS
                } else {
                    // A defect in this build's own declaration is not the
                    // consumer's to act on, so it is reported as a failure
                    // rather than dressed up as a contract reason.
                    eprintln!("{}: {error}", harness::PROVIDER_ID);
                    ExitCode::FAILURE
                }
            }
        },
        Err(error) => {
            eprintln!("{}: {error}", harness::PROVIDER_ID);
            ExitCode::FAILURE
        }
    }
}

fn print_help() {
    println!("{} {}", harness::PROVIDER_ID, env!("CARGO_PKG_VERSION"));
    println!();
    println!(
        "Configures {} in a caller-named target directory.",
        harness::PRODUCT
    );
    println!(
        "Documented configuration home: {} ({})",
        harness::DOCUMENTED_CONFIG_HOME,
        harness::CONFIG_HOME_ENV
    );
    println!();
    println!("Provider commands (ai-stp protocol v3):");
    println!("  provider-info");
    println!("  status            --target <dir> --json");
    println!("  validate-bundle   --target <dir> --json --bundle <path> ...");
    println!("  plan-operation    --target <dir> --json --operation <op> ...");
    println!("  apply-operation   --target <dir> --json --plan <path> --plan-digest <d> ...");
    println!("  recover-operation --target <dir> --json");
    println!();
    println!("backup, restore and remove are applied. install and replace are");
    println!("planned and then refuse: this build carries no bundle reader yet.");
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use std::fs;
    use std::path::{Path, PathBuf};

    use provider_v3::argv;

    use super::*;

    /// A per-test directory holding the target and anything written beside it.
    ///
    /// The plan artifact must live *outside* the target: writing it inside would
    /// change the target's identity between plan and apply, and the apply would
    /// then correctly refuse its own plan as stale. It must also be unique per
    /// test, because these run in parallel and a shared path made one test read
    /// another's plan.
    fn scratch(name: &str) -> PathBuf {
        let base = std::env::temp_dir().join(format!("claude-setup-system-{name}"));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(base.join("target")).unwrap();
        fs::canonicalize(&base).unwrap()
    }

    fn seeded(name: &str) -> PathBuf {
        let target = scratch(name).join("target");
        fs::write(target.join("CLAUDE.md"), "# first\n").unwrap();
        fs::write(target.join("settings.json"), "{\"model\":\"first\"}").unwrap();
        fs::create_dir_all(target.join("skills")).unwrap();
        fs::write(target.join("skills").join("a.md"), "skill one").unwrap();
        // A sibling overlay this provider does not own.
        fs::write(target.join("unrelated.txt"), "keep me").unwrap();
        target
    }

    fn run(tokens: Vec<String>) -> serde_json::Value {
        wire::dispatch(argv::parse(tokens).unwrap()).unwrap()
    }

    fn wire_args(command: &str, target: &Path, extra: &[&str]) -> Vec<String> {
        let mut tokens = vec![
            command.to_owned(),
            "--target".to_owned(),
            target.to_string_lossy().into_owned(),
            "--json".to_owned(),
        ];
        tokens.extend(extra.iter().map(|s| (*s).to_owned()));
        tokens
    }

    const RELEASE: &str = "sha256:3333333333333333333333333333333333333333333333333333333333333333";

    fn far_future() -> String {
        "2099-01-01T00:00:00.000Z".to_owned()
    }

    fn plan_then_apply(target: &Path, operation: &str, extra: &[&str]) -> serde_json::Value {
        let expiry = far_future();
        let mut arguments = vec![
            "--operation",
            operation,
            "--provider-release-digest",
            RELEASE,
            "--operation-id",
            "operation_01TEST",
            "--expires-at",
            &expiry,
        ];
        arguments.extend_from_slice(extra);
        let planned = run(wire_args("plan-operation", target, &arguments));
        assert_eq!(planned["state"], "planned", "plan refused: {planned}");

        let plan_path = target.join("..").join("plan.json");
        let bytes = setup_core::canonical::to_canonical_bytes(&planned["plan"]).unwrap();
        fs::write(&plan_path, bytes).unwrap();

        run(wire_args(
            "apply-operation",
            target,
            &[
                "--plan",
                &plan_path.to_string_lossy(),
                "--plan-digest",
                planned["plan_digest"].as_str().unwrap(),
                "--provider-release-digest",
                RELEASE,
            ],
        ))
    }

    #[test]
    fn provider_info_answers_without_a_target() {
        let answer = wire::dispatch(argv::parse(["provider-info"]).unwrap()).unwrap();
        assert_eq!(answer["provider_id"], harness::PROVIDER_ID);
        assert_eq!(answer["protocol_version"], 3);
        assert!(
            answer["projection_profile"]["digest"]
                .as_str()
                .unwrap()
                .starts_with("sha256:")
        );
    }

    #[test]
    fn status_reports_an_untouched_target_without_changing_it() {
        let target = seeded("status");
        let before = fs::read_to_string(target.join("CLAUDE.md")).unwrap();

        let answer = run(wire_args("status", &target, &[]));
        assert_eq!(answer["state"], "verified");
        assert_eq!(answer["provider_state"]["present"], false);
        assert!(answer["journal"].is_null());
        assert_eq!(
            fs::read_to_string(target.join("CLAUDE.md")).unwrap(),
            before
        );
    }

    #[test]
    fn a_backup_captures_the_target_and_leaves_it_alone() {
        let target = seeded("backup");
        let applied = plan_then_apply(&target, "backup", &[]);

        assert_eq!(applied["state"], "verified");
        assert_eq!(
            applied["expected_target_digest"],
            applied["target_identity_digest"]
        );
        assert_eq!(
            fs::read_to_string(target.join("CLAUDE.md")).unwrap(),
            "# first\n"
        );

        let status = run(wire_args("status", &target, &[]));
        assert_eq!(status["backups"].as_array().unwrap().len(), 1);
        assert_eq!(status["provider_state"]["drift_state"], "clean");
    }

    #[test]
    fn restore_returns_the_captured_state_and_keeps_unowned_files() {
        let target = seeded("restore");
        plan_then_apply(&target, "backup", &[]);

        // Change everything this provider owns, plus something it does not.
        fs::write(target.join("CLAUDE.md"), "# second\n").unwrap();
        fs::write(target.join("skills").join("b.md"), "skill two").unwrap();
        fs::write(target.join("unrelated.txt"), "still mine").unwrap();

        let applied = plan_then_apply(&target, "restore", &[]);
        assert_eq!(applied["state"], "verified");

        assert_eq!(
            fs::read_to_string(target.join("CLAUDE.md")).unwrap(),
            "# first\n"
        );
        assert!(!target.join("skills").join("b.md").exists());
        // The sibling overlay is not this provider's to revert.
        assert_eq!(
            fs::read_to_string(target.join("unrelated.txt")).unwrap(),
            "still mine"
        );
    }

    #[test]
    fn restore_can_name_an_older_backup_than_the_last_one() {
        let target = seeded("restore-chosen");
        plan_then_apply(&target, "backup", &[]);
        let first = run(wire_args("status", &target, &[]))["backups"][0]["backup_ref"]
            .as_str()
            .unwrap()
            .to_owned();

        fs::write(target.join("CLAUDE.md"), "# second\n").unwrap();
        plan_then_apply(&target, "backup", &[]);
        fs::write(target.join("CLAUDE.md"), "# third\n").unwrap();

        // Without a reference this would restore the second capture.
        let applied = plan_then_apply(&target, "restore", &["--backup-ref", &first]);
        assert_eq!(applied["state"], "verified");
        assert_eq!(
            fs::read_to_string(target.join("CLAUDE.md")).unwrap(),
            "# first\n"
        );
    }

    #[test]
    fn remove_withdraws_only_what_this_provider_owns() {
        let target = seeded("remove");
        let applied = plan_then_apply(&target, "remove", &[]);
        assert_eq!(applied["state"], "verified");

        assert!(!target.join("CLAUDE.md").exists());
        assert!(!target.join("settings.json").exists());
        assert!(!target.join("skills").exists());
        assert_eq!(
            fs::read_to_string(target.join("unrelated.txt")).unwrap(),
            "keep me"
        );
    }

    #[test]
    fn an_expired_plan_has_no_effect() {
        let target = seeded("expired");
        let planned = run(wire_args(
            "plan-operation",
            &target,
            &[
                "--operation",
                "backup",
                "--provider-release-digest",
                RELEASE,
                "--operation-id",
                "operation_01TEST",
                "--expires-at",
                "2000-01-01T00:00:00.000Z",
            ],
        ));
        let plan_path = target.join("..").join("expired-plan.json");
        fs::write(
            &plan_path,
            setup_core::canonical::to_canonical_bytes(&planned["plan"]).unwrap(),
        )
        .unwrap();

        let error = wire::dispatch(
            argv::parse(wire_args(
                "apply-operation",
                &target,
                &[
                    "--plan",
                    &plan_path.to_string_lossy(),
                    "--plan-digest",
                    planned["plan_digest"].as_str().unwrap(),
                    "--provider-release-digest",
                    RELEASE,
                ],
            ))
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(error.reason(), Some(provider_v3::WireReason::Stale));
        assert_eq!(
            run(wire_args("status", &target, &[]))["backups"]
                .as_array()
                .unwrap()
                .len(),
            0
        );
    }

    #[test]
    fn a_plan_digest_that_does_not_bind_the_artifact_has_no_effect() {
        let target = seeded("wrong-digest");
        let planned = run(wire_args(
            "plan-operation",
            &target,
            &[
                "--operation",
                "backup",
                "--provider-release-digest",
                RELEASE,
                "--operation-id",
                "operation_01TEST",
                "--expires-at",
                &far_future(),
            ],
        ));
        let plan_path = target.join("..").join("mismatched-plan.json");
        fs::write(
            &plan_path,
            setup_core::canonical::to_canonical_bytes(&planned["plan"]).unwrap(),
        )
        .unwrap();

        let error = wire::dispatch(
            argv::parse(wire_args(
                "apply-operation",
                &target,
                &[
                    "--plan",
                    &plan_path.to_string_lossy(),
                    "--plan-digest",
                    "sha256:0000000000000000000000000000000000000000000000000000000000000000",
                    "--provider-release-digest",
                    RELEASE,
                ],
            ))
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            error.reason(),
            Some(provider_v3::WireReason::DigestMismatch)
        );
        assert_eq!(
            run(wire_args("status", &target, &[]))["backups"]
                .as_array()
                .unwrap()
                .len(),
            0
        );
    }

    #[test]
    fn planning_is_refused_while_a_journal_is_published() {
        let target = seeded("journaled");
        let control = target.join(harness::CONTROL_DIRECTORY);
        fs::create_dir_all(&control).unwrap();
        setup_core::journal::Journal {
            schema_version: setup_core::journal::JOURNAL_SCHEMA,
            phase: setup_core::journal::Phase::Prepared,
            operation_id: "operation_01STUCK".to_owned(),
            operation: "backup".to_owned(),
            plan_digest: RELEASE.to_owned(),
            target_precondition_digest: RELEASE.to_owned(),
            backup_ref: None,
        }
        .publish_prepared(&control)
        .unwrap();

        let error = wire::dispatch(
            argv::parse(wire_args(
                "plan-operation",
                &target,
                &[
                    "--operation",
                    "backup",
                    "--provider-release-digest",
                    RELEASE,
                    "--operation-id",
                    "operation_01TEST",
                    "--expires-at",
                    &far_future(),
                ],
            ))
            .unwrap(),
        )
        .unwrap_err();
        assert_eq!(
            error.reason(),
            Some(provider_v3::WireReason::RecoveryRequired)
        );
    }

    #[test]
    fn recovery_from_prepared_returns_the_exact_pre_operation_target() {
        let target = seeded("recover");
        plan_then_apply(&target, "backup", &[]);
        let reference = run(wire_args("status", &target, &[]))["backups"][0]["backup_ref"]
            .as_str()
            .unwrap()
            .to_owned();

        // Simulate an interruption after the capture and part-way through a write.
        let control = target.join(harness::CONTROL_DIRECTORY);
        fs::write(target.join("CLAUDE.md"), "# half written\n").unwrap();
        setup_core::journal::Journal {
            schema_version: setup_core::journal::JOURNAL_SCHEMA,
            phase: setup_core::journal::Phase::Prepared,
            operation_id: "operation_01INTERRUPTED".to_owned(),
            operation: "restore".to_owned(),
            plan_digest: RELEASE.to_owned(),
            target_precondition_digest: RELEASE.to_owned(),
            backup_ref: Some(reference.clone()),
        }
        .publish_prepared(&control)
        .unwrap();

        let recovered = run(wire_args("recover-operation", &target, &[]));
        assert_eq!(recovered["recovered"], true);
        assert_eq!(recovered["phase"], "prepared");
        assert_eq!(
            fs::read_to_string(target.join("CLAUDE.md")).unwrap(),
            "# first\n"
        );

        // With the journal cleared, planning works again.
        let planned = run(wire_args(
            "plan-operation",
            &target,
            &[
                "--operation",
                "backup",
                "--provider-release-digest",
                RELEASE,
                "--operation-id",
                "operation_01AFTER",
                "--expires-at",
                &far_future(),
            ],
        ));
        assert_eq!(planned["state"], "planned");
    }

    #[test]
    fn recovery_with_no_journal_says_so_rather_than_inventing_work() {
        let target = seeded("recover-clean");
        let answer = run(wire_args("recover-operation", &target, &[]));
        assert_eq!(answer["recovered"], false);
    }

    #[test]
    fn a_restore_plan_names_the_target_it_will_produce() {
        let target = seeded("restore-plan-shape");
        plan_then_apply(&target, "backup", &[]);
        let planned = run(wire_args(
            "plan-operation",
            &target,
            &[
                "--operation",
                "restore",
                "--provider-release-digest",
                RELEASE,
                "--operation-id",
                "operation_01TEST",
                "--expires-at",
                &far_future(),
            ],
        ));
        assert!(
            planned["plan"]["restore_target_digest"]
                .as_str()
                .unwrap()
                .starts_with("sha256:")
        );
        assert!(planned["plan"]["backup_ref"].is_string());
    }

    #[test]
    fn install_and_replace_are_declared_but_refuse_with_the_real_limitation() {
        let target = seeded("not-yet");
        for operation in ["install", "replace"] {
            let error = wire::dispatch(
                argv::parse(wire_args(
                    "plan-operation",
                    &target,
                    &[
                        "--operation",
                        operation,
                        "--provider-release-digest",
                        RELEASE,
                        "--operation-id",
                        "operation_01TEST",
                        "--expires-at",
                        &far_future(),
                    ],
                ))
                .unwrap(),
            )
            .unwrap_err();
            assert!(
                error.detail().contains("needs a reader for"),
                "{operation}: {error}"
            );
        }
        // They stay declared, because the contract requires all five.
        let info = harness::provider_info().unwrap();
        assert!(info.declares(provider_v3::Operation::Install));
        assert!(info.declares(provider_v3::Operation::Replace));
    }

    #[test]
    fn launch_is_refused_because_this_provider_does_not_start_the_product() {
        let target = seeded("launch");
        let error =
            wire::dispatch(argv::parse(wire_args("launch", &target, &[])).unwrap()).unwrap_err();
        assert_eq!(
            error.reason(),
            Some(provider_v3::WireReason::UnsupportedOperation)
        );
    }
}
