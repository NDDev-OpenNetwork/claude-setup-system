//! The Claude Code setup system.
//!
//! One binary, two surfaces. The ai-stp provider commands are what the consumer
//! invokes; the human commands are what the owner types. Both reach the target
//! through `setup-core`, and neither has a shortcut around it.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with every other setup system, so a change to
//! behaviour lands once and a change to Claude Code's surface lands here.
//!
//! Claude Code installs itself, so this provider owns the configuration only.

use std::process::ExitCode;

mod software;

use harness_runtime::Harness;
use provider_v3::{ComponentKind, ProjectionKind};

/// Everything specific to Claude Code, verified against `claude-baseline.json`.
pub const CLAUDE: Harness = Harness {
    // The consumer's closed harness enum spells this `claude-code`, and that
    // enum is embedded in published corpus artifacts. Ours was not released, so
    // ours is the one that moves.
    harness_id: "claude-code",
    provider_id: "claude-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "Claude Code",
    vendor: "Anthropic",
    documented_config_home: "~/.claude",
    config_home_env: "CLAUDE_CONFIG_DIR",
    control_directory: ".claude-setup-system",
    state_file: "NDDEV-CLAUDE-PROVIDER.json",
    predecessor_state_file: "NDDEV-CLAUDE-SETUP.json",
    profile_id: "claude/native-and-marketplace/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    native_namespaces: &[
        "CLAUDE.md",
        "settings.json",
        "skills",
        "agents",
        "commands",
        "hooks",
        ".mcp.json",
    ],
    // The product's own. `plugins/` is the Claude CLI's registry and cache, not
    // ours to rewrite even though a setup can register a marketplace that fills
    // it; `projects/` is session history; `.credentials.json` is exactly what a
    // backup must never copy.
    never_touch: &[".credentials.json", "projects", "plugins"],
    permission_profiles: &["default"],
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Command,
        ComponentKind::Hook,
        ComponentKind::Mcp,
        ComponentKind::Plugin,
        ComponentKind::Setting,
    ],
    projection_kinds: &[
        ProjectionKind::NativeFiles,
        ProjectionKind::Marketplace,
        ProjectionKind::Plugin,
    ],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    software: Some(software::SOFTWARE),
};

fn main() -> ExitCode {
    harness_runtime::run(&CLAUDE, std::env::args().skip(1).collect())
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
        assert_eq!(baseline["config_dir"], CLAUDE.documented_config_home);
        assert_eq!(baseline["config_dir_env"], CLAUDE.config_home_env);
    }

    #[test]
    fn the_settings_file_the_baseline_names_is_one_we_own() {
        assert!(
            CLAUDE
                .native_namespaces
                .contains(&baseline()["settings_file"].as_str().unwrap())
        );
    }

    #[test]
    fn everything_the_baseline_marks_never_touch_is_disclaimed_here() {
        for entry in baseline()["never_touch"].as_array().unwrap() {
            let Some(name) = entry.as_str() else { continue };
            // `~/.claude.json` sits outside the target and cannot be a top-level
            // entry of it; the rest must be disclaimed by name.
            if name.starts_with('~') {
                continue;
            }
            assert!(
                !CLAUDE.native_namespaces.contains(&name),
                "{name} is marked never_touch by the baseline but claimed as ours"
            );
            assert!(
                CLAUDE.never_touch.contains(&name),
                "{name} is marked never_touch by the baseline but not disclaimed here"
            );
        }
    }

    #[test]
    fn the_cli_owned_plugin_registry_is_not_ours_to_rewrite() {
        // The baseline lists these under `cli_owned`. A setup may register a
        // marketplace that causes the CLI to fill them; writing them directly
        // would be writing the product's own bookkeeping.
        for entry in baseline()["cli_owned"].as_array().unwrap() {
            let name = entry.as_str().unwrap();
            let top = name.split('/').next().unwrap();
            assert!(
                !CLAUDE.native_namespaces.contains(&top),
                "{top} is CLI-owned but claimed"
            );
        }
    }

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = CLAUDE.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in CLAUDE.never_touch {
            assert!(
                !CLAUDE.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }
}
