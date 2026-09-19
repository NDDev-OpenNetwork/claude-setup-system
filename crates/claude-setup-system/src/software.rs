//! Claude Code's own program, as measured rather than as described.
//!
//! Generated from the `software_artifacts` block of
//! `references/claude-baseline.json`. Every member path below was read out
//! of the archive it names, not assumed: codex's carries the target triple and
//! so genuinely differs per platform.
//!
//! Where a `previous_software_artifacts` block is present, it is transcribed
//! too. It is not a second choice: the outgoing current pin is stored there on
//! a bump, so the pair is always two consecutive real releases and there is
//! still exactly one value to keep fresh.
//!
//! Do not edit. The test at the bottom re-reads that baseline and compares it
//! field by field, so an edit here fails rather than silently installing bytes
//! nobody measured.

use harness_runtime::{Artifact, Delivery, Previous, Shape, Software};

/// The artifacts claude is published as.
pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-arm64/-/claude-code-linux-arm64-2.1.278.tgz",
        bytes: 104_532_073,
        sha256: "sha256:ec54bd853c9cb71cb03874d694a1a38638d350828f0d84e0492fe1734a172302",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.278.tgz",
        bytes: 104_055_827,
        sha256: "sha256:d1fb51ab0a0234d1bd7f418ee9d6b6b124c2412b2ddaf3dfc3256bad8063f1c7",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.278.tgz",
        bytes: 94_329_072,
        sha256: "sha256:934a258c59e90ed6ba768d0d46502d3163607dec949845650af80e5f1228330c",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-x64/-/claude-code-darwin-x64-2.1.278.tgz",
        bytes: 98_435_684,
        sha256: "sha256:36b343bc10f55627d773946ef521b020535d68c5b075bee77ed1091252f2f590",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-arm64/-/claude-code-win32-arm64-2.1.278.tgz",
        bytes: 103_919_037,
        sha256: "sha256:1fffc48348f421a0e5ce922e78140426d21eeb1fa536f07942f10064be55d90b",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-x64/-/claude-code-win32-x64-2.1.278.tgz",
        bytes: 106_956_684,
        sha256: "sha256:0fd2a29ad0752ab130f3e0bb0a34afd817a8cfef39e3f2d3adc60b5d54a22f71",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
];

/// The artifacts 2.1.273 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-arm64/-/claude-code-linux-arm64-2.1.273.tgz",
        bytes: 102_117_697,
        sha256: "sha256:2030a0b196d9fa8a265bb08afae81fbfe39e67d824b3ee0b2d22bf940ed6f537",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.273.tgz",
        bytes: 101_647_074,
        sha256: "sha256:ab7679e0dc5b38cd07d9d63e3a1db1178bae722474270469795b733ecada103c",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.273.tgz",
        bytes: 91_885_277,
        sha256: "sha256:9472eb2d68b614dfddb6b3683a98d16be79dc34b66268282b0b6ce8ef4a3e04b",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-x64/-/claude-code-darwin-x64-2.1.273.tgz",
        bytes: 95_952_306,
        sha256: "sha256:a72dd49c8f4094ef71ddd34a9c1b767845db2087d6cf77772634a3e3d1d85a22",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-arm64/-/claude-code-win32-arm64-2.1.273.tgz",
        bytes: 101_517_506,
        sha256: "sha256:a102c527abf2b232fc12c078fc63b332bdb4b1d9b914a7f506ff2a5348322583",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-x64/-/claude-code-win32-x64-2.1.273.tgz",
        bytes: 104_549_735,
        sha256: "sha256:5d5e3b799a9a29d5eafc41fa674d079b1caf2aa8449d005a1369a7c1aad5dc98",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
];

/// Claude Code's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "2.1.278",
    command: "claude",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "2.1.273",
        artifacts: PREVIOUS_ARTIFACTS,
    }),
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    // Named rather than glob-imported: a product delivered by a package manager
    // has no `Artifact` in scope, and the test is the same text for all seven.
    use harness_runtime::{Delivery, Shape};

    use super::SOFTWARE;

    fn measured() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/claude-baseline.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn every_artifact_compiled_in_is_the_one_the_baseline_measured() {
        let block = &measured()["software_artifacts"];
        assert_eq!(block["version"], SOFTWARE.version);
        assert_eq!(block["command"], SOFTWARE.command);

        let Delivery::Artifacts(compiled) = SOFTWARE.delivery else {
            // A product delivered by a package manager has no artifacts, and
            // the baseline must agree that it has none.
            assert_eq!(block["shape"], "manager");
            assert!(block["platforms"].as_object().unwrap().is_empty());
            return;
        };
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            compiled.len(),
            published.len(),
            "the table and the baseline disagree on how many platforms exist"
        );
        for artifact in compiled {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
            let member = entry.get("member").and_then(serde_json::Value::as_str);
            assert_eq!(
                member.unwrap_or(""),
                artifact.member,
                "{} names a different member",
                artifact.platform
            );
            assert_eq!(
                artifact.shape == Shape::Raw,
                member.is_none(),
                "{} disagrees about whether the bytes are the program",
                artifact.platform
            );
        }
    }

    /// The second pin is the baseline's, or it is absent in both places.
    ///
    /// Asserted from either side rather than only where it exists: a harness
    /// that has never been bumped must compile in `None`, and a build that
    /// dropped the block while the baseline still carried it would otherwise
    /// pass by having nothing to compare.
    #[test]
    fn the_version_this_build_can_move_between_is_the_one_measured_before_it() {
        let baseline = measured();
        let recorded = baseline.get("previous_software_artifacts");
        let Some(earlier) = SOFTWARE.previous else {
            assert!(
                recorded.is_none(),
                "the baseline records a previous release and this build names none"
            );
            return;
        };
        let block = recorded.unwrap_or_else(|| {
            panic!("this build names a previous release the baseline does not record")
        });
        assert_eq!(block["version"], earlier.version);
        assert_ne!(
            earlier.version, SOFTWARE.version,
            "a second pin equal to the first is one version wearing two names"
        );
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            earlier.artifacts.len(),
            published.len(),
            "the previous table and the baseline disagree on how many platforms exist"
        );
        for artifact in earlier.artifacts {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
        }
    }

    #[test]
    fn a_platform_the_vendor_does_not_publish_is_listed_rather_than_missing() {
        let block = &measured()["software_artifacts"];
        let unpublished: Vec<&str> = block
            .get("unpublished")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(unpublished, SOFTWARE.unsupported);
    }

    #[test]
    fn no_release_calls_a_platform_both_published_and_unpublished() {
        let baseline = measured();
        for name in ["software_artifacts", "previous_software_artifacts"] {
            let Some(block) = baseline.get(name) else {
                continue;
            };
            let published = block["platforms"].as_object().unwrap();
            let unpublished = block
                .get("unpublished")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str);
            for platform in unpublished {
                assert!(
                    !published.contains_key(platform),
                    "{name}: {platform} is both published and unpublished"
                );
            }
        }
    }
}
