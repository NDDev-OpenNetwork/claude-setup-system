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
//! This provider owns the configuration *and* the program: `src/software.rs`
//! carries the artifacts Anthropic publishes, and the software operations take
//! a `--prefix` for the program distinct from the `--target` that holds its
//! configuration.
//!
//! This line used to say *"Claude Code installs itself, so this provider owns
//! the configuration only"*. That was the owner's original assignment rather
//! than what the build does, and `7d156c2` made it false without editing it
//! here.

use std::process::ExitCode;

mod software;

use harness_runtime::{Harness, LaunchBinding};
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
    // Measured 2026-08-28 by making the product write: `mcp add --scope user`
    // under this variable wrote `<target>/.claude.json`, so the home follows it
    // rather than one file inside it.
    launch_binding: LaunchBinding::Complete {
        how: "measured by making the product write its own configuration into the target",
    },
    // Measured 2026-08-30 in the pinned 2.1.251 artifact: `DISABLE_UPDATES` nine
    // times, beside `DISABLE_AUTOUPDATER` and `DISABLE_UPGRADE_COMMAND`. The
    // vendor documents the first as the one that stops manual updates too, which
    // is the one that matters: an autoupdater switched off still leaves `claude
    // update` able to replace the bytes this provider pinned.
    updates_off_env: "DISABLE_UPDATES",
    // One home, one variable: nothing here is conditional.
    config_home_note: "",
    control_directory: ".claude-setup-system",
    state_file: "NDDEV-CLAUDE-PROVIDER.json",
    predecessor_state_file: "NDDEV-CLAUDE-SETUP.json",
    profile_id: "claude/native-and-marketplace/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    // Each entry is a surface `references/claude-baseline.json` sources. The
    // two that were here and are not documented are in that file's `declined`
    // list with the page that decided it: `.mcp.json`, because Claude Code keeps
    // MCP servers in `~/.claude.json` and nothing reads a `.mcp.json` under this
    // home; and `hooks`, because hooks are a key of `settings.json` and
    // `~/.claude/hooks/` is only where the scripts a hook command names
    // conventionally sit.
    //
    // `rules` is the one that was missing: user-level rules apply to every
    // project on the machine, and a setup could not carry one.
    native_namespaces: &[
        "CLAUDE.md",
        "settings.json",
        "skills",
        "agents",
        "commands",
        "rules",
        // Added 2026-08-28. The vendor calls a script saved here *just for
        // you*, and it runs as a `/<name>` command. Owned so a backup captures
        // it; routing nothing, because `command` already routes to `commands`
        // and one kind on two surfaces makes a consumer's route ambiguous.
        "workflows",
    ],
    // The product's own. `plugins/` is the Claude CLI's registry and cache, not
    // ours to rewrite even though a setup can register a marketplace that fills
    // it; `projects/` is session history; `.credentials.json` is exactly what a
    // backup must never copy.
    never_touch: &[".credentials.json", "projects", "plugins", ".claude.json"],
    // No near neighbour measured for this product. A marker listed here is a
    // refusal waiting to happen, so nothing is listed without evidence.
    foreign_homes: &[],
    permission_profiles: &["default"],
    // `Mcp` and `Hook` were declared and are not: neither has a surface this
    // provider can own, so both were promises of a rollback nothing could
    // keep.
    //
    // **`Plugin` joins them, and it is the same defect one step further along.**
    // A plugin reaches Claude Code through `settings.json` -- `enabledPlugins`
    // and `extraKnownMarketplaces` -- rather than through a directory, so the
    // kind was routed to a file. Unlike `Mcp` and `Hook` it *had* a surface,
    // which is what made it look sound.
    //
    // What it lacked was the semantics. Measured: `write_bundle_files` calls
    // `remove_managed` and then writes the bundle's bytes verbatim. There is no
    // merge, no key handling, no kind-specific behaviour anywhere in this
    // build. So a `plugin` component carrying `settings.json` would replace the
    // whole file -- and, because the removal runs first, delete `CLAUDE.md`,
    // `skills`, `agents`, `commands` and `rules` on the way in.
    //
    // It is also the only kind across all seven that routes to a file rather
    // than a directory, which is the shape that should have prompted the
    // question. The consumer stopped at exactly this row rather than guess the
    // semantics, and was right to. One unreachable capability, correctly
    // reported, is a smaller lie than a route that compiles into the wrong
    // thing. Declaring it again means building settings-merge first.
    // `Plugin` is here because of a mechanism that needs no plugins directory.
    //
    // The product's own reference: *"Any folder under a skills directory that
    // contains a `.claude-plugin/plugin.json` manifest is loaded as a plugin
    // named `<name>@skills-dir` on the next session, with no marketplace and no
    // install step."* At personal scope that skills directory is
    // `~/.claude/skills/`, which this provider already owns, and the reference's
    // own table distinguishes the two by manifest alone: `foo/SKILL.md` with no
    // manifest is a skill named `foo`, `foo/.claude-plugin/plugin.json` is a
    // plugin `foo@skills-dir`.
    //
    // So `skills` routes two kinds, and that is the product's design rather
    // than a compromise here. `plugins/` stays declined: it holds the cache a
    // marketplace install copies into, which is product state.
    //
    // The reason this was blank until 2026-08-29 is worth keeping. The record
    // said a plugin *projects through settings.json*, which was true of
    // enabling one and was never the whole question -- and the skills-directory
    // mechanism needs neither settings key nor marketplace. A negative taken
    // from the pages that happened to be read, exactly like `command` and
    // `instruction` on the harness two doors down.
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Command,
        ComponentKind::Setting,
        ComponentKind::Plugin,
    ],
    projection_kinds: &[
        ProjectionKind::NativeFiles,
        ProjectionKind::Marketplace,
        ProjectionKind::Plugin,
    ],
    // One scope. Claude Code keeps a project copy of every surface, but under `.claude/`
    // in the workspace rather than under this target -- a different root, not a
    // second scope of this one.
    //
    // Empty rather than absent: a harness that owns one target says so.
    scoped_projections: &[],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    // Generated by `build.rs` from this harness's `setups/` directory, so the
    // binary carries the catalog it is named after instead of hoping to find
    // one on a disk it was never shipped to.
    embedded_setups: include!(concat!(env!("OUT_DIR"), "/embedded_setups.rs")),
    software: Some(software::SOFTWARE),
};

fn main() -> ExitCode {
    harness_runtime::run(&CLAUDE, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    /// The directory name this harness's setups live under in the workspace.
    const TOOL: &str = "claude";
    /// The declaration under test, named once so the shared test below reads
    /// the same in all seven crates.
    const HARNESS: Harness = CLAUDE;

    /// `build.rs` put the whole catalog in, under the paths it will be read by.
    ///
    /// This does **not** test for staleness, and an earlier version of this
    /// comment claimed it did. It cannot: `build.rs` declares
    /// `rerun-if-changed` on the catalog directory, so editing a setup rebuilds
    /// the table before this runs, and the test would be comparing the tree
    /// with itself. Observed — a deliberately edited setup left it green.
    ///
    /// What it does test is the build script, against a walk written
    /// independently of it: every file present, none invented, bytes exact, and
    /// paths relative and slash-separated. That last one is the one that would
    /// really break — `join("/")` is the only reason these keys are usable on
    /// Windows, and a path built with the platform separator would still look
    /// perfectly correct in the generated source.
    /// The bytes this harness ships, pinned so they cannot change unseen.
    ///
    /// A setup's `definition_digest` is what makes two setups the same setup,
    /// and it appears in `list`, in a plan and in provider state -- and until
    /// this, nothing compared it to anything. A stray character in a setup file
    /// changed what the estate installs and every test stayed green.
    ///
    /// One aggregate rather than one per setup, because the claim is about the
    /// catalogue: sorted definition digests, joined by a newline, hashed. A
    /// deliberate change to a setup updates the line in the baseline, which is
    /// the point -- the peer calls this a golden and it earns itself the first
    /// time a row moves without anyone meaning it to.
    ///
    /// **And it is the three-OS check nothing else makes.** The setups are
    /// embedded with `include_bytes!`, so whatever the checkout holds is what
    /// ships; `.gitattributes` pins `eol=lf` to keep a Windows checkout from
    /// rewriting them, and this is the assertion that would notice if it ever
    /// stopped working. The matrix runs it on all three systems, so a digest
    /// that differed by platform could not stay hidden.
    #[test]
    fn the_catalogue_this_harness_ships_is_the_one_the_baseline_records() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let mut digests: Vec<String> = catalog
            .list()
            .unwrap()
            .iter()
            // **Both digests, because one of them holds nothing a person
            // reads.** `definition_digest` is the payload tree; the manifest --
            // `id`, `sources`, `description` -- was covered by no digest in this
            // estate, and those three are what a consumer renders on the surface
            // that precedes an install. A description was rewritten and the
            // whole gate stayed clean, which is how this was found.
            .map(|setup| format!("{}\n{}", setup.definition_digest, setup.manifest_digest))
            .collect();
        digests.sort();
        let joined = digests.join("\n");
        let aggregate = harness_runtime::digest_of_bytes(&joined);
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let recorded = baseline["setup_catalogue_digest"].as_str().unwrap_or("");
        assert_eq!(
            aggregate, recorded,
            "the setups this binary ships are not the ones {TOOL}-baseline.json \
             records; if the change was meant, put this digest there"
        );
    }

    #[test]
    fn the_catalog_this_binary_carries_is_the_one_in_the_tree() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        // The workspace holds one directory per harness; a rendered public tree
        // ships one harness and holds it flat. Same two candidates `build.rs`
        // chooses between, asked the same way.
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };

        // Only the setup directories, which is what the reader lists and what
        // `build.rs` embeds. A rendered public tree also carries a
        // `setups/README.md` at the catalog root, which belongs to no setup.
        let mut on_disk = Vec::new();
        let mut stack: Vec<std::path::PathBuf> = std::fs::read_dir(&root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.join("setup.json").is_file())
            .collect();
        while let Some(directory) = stack.pop() {
            for entry in std::fs::read_dir(&directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    on_disk.push(path);
                }
            }
        }

        assert_eq!(
            HARNESS.embedded_setups.len(),
            on_disk.len(),
            "the binary carries {} files and the tree holds {}",
            HARNESS.embedded_setups.len(),
            on_disk.len()
        );

        for (relative, bytes) in HARNESS.embedded_setups {
            assert!(
                !relative.contains('\\') && !relative.starts_with('/'),
                "{relative:?} is not a relative slash path; a key built with the \
                 platform separator reads correctly on Unix and finds nothing on Windows"
            );
            let path = root.join(relative);
            let found = std::fs::read(&path)
                .unwrap_or_else(|e| panic!("{relative} is compiled in but not in the tree: {e}"));
            assert_eq!(
                &found, bytes,
                "{relative} differs between the binary and the tree"
            );
        }
    }

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
    /// Everything this harness claims to own, against the vendor page that
    /// decided it.
    ///
    /// What this replaced only checked that the baseline parsed. The block it
    /// reads now is hand-authored beside the rest of the baseline, and this is
    /// what keeps that block from being decoration: a namespace no vendor
    /// document names, or a declared kind no owned surface routes, is red here.
    ///
    /// Both directions, because the defect it was written for ran both ways --
    /// `~/.cursor/rules` was owned and does not exist, `~/.pi/agent/prompts`
    /// exists and was not owned. Conformance caught neither: its
    /// `declared_native_route_is_compilable` case asks for **one** route, not
    /// every one.
    #[test]
    fn every_surface_this_harness_owns_is_one_the_vendor_documents() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references")
            .join(format!("{TOOL}-baseline.json"));
        let baseline: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let problems = harness_runtime::surfaces::disagreements(&HARNESS, &baseline);
        assert!(
            problems.is_empty(),
            "the declaration and {TOOL}-baseline.json disagree:\n  {}",
            problems.join("\n  ")
        );
    }
    /// A setup that writes a configuration file says where its format came from.
    ///
    /// The release before this one made the *surfaces* sourced: a path this
    /// provider owns cites the page that documents it. This is the same rule
    /// one level down, and it was written because two of the seven failed it.
    ///
    /// opencode's baseline set `"permission": "ask"` where the product
    /// documents an object of tool names, and antigravity's set
    /// `toolPermissions` where the product reads `toolPermission` with four
    /// values, none of them the one written. Both were valid JSON in the right
    /// file at the right path. Both installed, verified and restored cleanly.
    /// Neither changed anything about the product — a target that looks
    /// configured and is not, which is the failure this estate refuses one
    /// level up and had been shipping one level down.
    /// Two files in one setup that a case-insensitive filesystem would merge.
    ///
    /// macOS and Windows fold case, so such a pair is one file there and two on
    /// Linux -- the setup would install different content depending on the
    /// machine, and its catalogue digest would differ per platform. The bundle
    /// reader has refused this for an arriving bundle since 0.0.11; this is the
    /// same rule applied to what this repository authors.
    /// Every component entry point describes itself.
    ///
    /// A `SKILL.md` or an agent whose frontmatter lost its `description` still
    /// installs, verifies and restores cleanly -- and the product names it after
    /// its directory and gives the model nothing to choose on. Documents under
    /// `references/` and files under `commands/` are exempt, because the
    /// products measured do not read frontmatter from either.
    /// Supporting documents are reachable from an entry point.
    ///
    /// A `references/` folder whose skill has no `SKILL.md` is prose nothing
    /// routes to. A generator in this repository produced exactly that, and
    /// every other guard passed it: the files are documents, so `unsourced`
    /// exempts them, and there is no `SKILL.md`, so `undescribed` has nothing
    /// to check.
    /// Nothing shipped sends a reader to a file this setup does not carry.
    ///
    /// A routing table naming `references/surfaces.md` in a setup that ships no
    /// such file sends the reader nowhere -- and the reader is a model, which
    /// will not say so. The generator here did exactly that: it pointed every
    /// harness's agent at that path, and codex ships no skill at all.
    #[test]
    fn nothing_shipped_names_a_document_it_does_not_carry() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::dangling_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn every_reference_folder_has_an_entry_point() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unreachable_references(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    /// Nothing inside a skill is a file no reader is sent to.
    ///
    /// Two findings in one hour were of exactly this shape and every guard in
    /// this estate was silent on both: an executable validator shipped into
    /// people's homes that nothing named, and eleven authoring pages written
    /// into four harnesses and routed to from none. The estate asked whether a
    /// *named* file exists and never whether an *existing* file is named.
    #[test]
    fn nothing_inside_a_skill_is_stranded() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let found = harness_runtime::catalog::stranded(
            &harness_runtime::Catalog::at(&root).list().unwrap(),
        );
        assert!(found.problems.is_empty(), "{}", found.problems.join("\n  "));
        // claude carries 9 file(s) inside its skill. Stated so that a layout change emptying the skill fails here rather than passing a guard with nothing left to walk.
        assert_eq!(
            found.entry_points, 9,
            "the stranded-file guard walked {} files inside skills, not 9",
            found.entry_points
        );
    }

    #[test]
    fn every_component_entry_point_describes_itself() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let examined = harness_runtime::catalog::undescribed(&catalog.list().unwrap());
        assert!(
            examined.problems.is_empty(),
            "{}",
            examined.problems.join("\n  ")
        );
        // claude ships 2 entry point(s) across its four postures. Stated so that a layout change removing them fails here rather than passing a guard with nothing left to check.
        assert_eq!(
            examined.entry_points, 2,
            "the description guard examined {} entry points, not 2",
            examined.entry_points
        );
    }

    #[test]
    fn no_two_files_in_a_setup_differ_only_in_case() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::colliding(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }

    #[test]
    fn a_setup_that_writes_configuration_says_where_its_format_came_from() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::unsourced(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Three postures, on every one of the seven.
    ///
    /// `baseline` is a working floor, `minimal` is the product's own defaults,
    /// and `full-auto` asks nothing and sandboxes nothing. A caller who learns
    /// them on one product knows them on all seven, which is the whole reason
    /// the names are the estate's rather than each harness's.
    ///
    /// The second half of the check is the one worth having: two setups with
    /// the same bytes mean one of them is a posture in name only, and it would
    /// still read as offered in `list`.
    #[test]
    fn the_three_postures_are_offered_and_are_actually_different() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems = harness_runtime::catalog::asymmetric(&catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
    /// Nothing this setup ships tells a reader to run something that is not here.
    ///
    /// A setup carries documents an agent reads and acts on -- a skill, a rule,
    /// a command file -- and nothing was checking them. One shipped
    /// `software-status --target <dir> --json` and `list --json` for six
    /// releases; the binary refuses both, and says so in those words.
    ///
    /// Two refusals: a name belonging to the frozen estate, and any line naming
    /// this provider followed by a verb `into_command` does not accept. English
    /// is not judged -- `install` in a sentence is a word, and only
    /// `<provider> install` is an instruction.
    #[test]
    fn nothing_this_harness_ships_names_a_command_it_refuses() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let root = manifest.join("../../setups").join(TOOL);
        let root = if root.is_dir() {
            root
        } else {
            manifest.join("../../setups")
        };
        let catalog = harness_runtime::Catalog::at(&root);
        let problems =
            harness_runtime::catalog::misdirecting(HARNESS.provider_id, &catalog.list().unwrap());
        assert!(problems.is_empty(), "{}", problems.join("\n  "));
    }
}
