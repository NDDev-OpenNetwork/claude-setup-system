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
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-arm64/-/claude-code-linux-arm64-2.1.283.tgz",
        bytes: 108_771_563,
        sha256: "sha256:e5ea2a2b09ad70e447d985bdcd1fb49ac60043cd4841ddc434cd69c5c8770bf7",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.283.tgz",
        bytes: 108_529_866,
        sha256: "sha256:d14ec0fca400151092c926928fea7e8e38231f7a90e6e1f8eeb2a55c24676afb",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.283.tgz",
        bytes: 98_721_064,
        sha256: "sha256:ce1799101976be0c35e10f2d6fe959369b3be6227645332671475cafd6f45591",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-x64/-/claude-code-darwin-x64-2.1.283.tgz",
        bytes: 102_777_696,
        sha256: "sha256:950368a19840116e3eddb0cbbfea108e8fc3e7b7ca5f36590e7a11984127a826",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-arm64/-/claude-code-win32-arm64-2.1.283.tgz",
        bytes: 107_274_873,
        sha256: "sha256:8fcd85c3356ba33981d2a32e87ce01ce8bcf4dc3a0aa67cb055ccd9900569e60",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-x64/-/claude-code-win32-x64-2.1.283.tgz",
        bytes: 111_392_373,
        sha256: "sha256:9a24a7de00831c064fef530d049271b9631340cfd51826c64d1e294482e6fc59",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
];

/// The artifacts 2.1.282 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-arm64/-/claude-code-linux-arm64-2.1.282.tgz",
        bytes: 107_607_069,
        sha256: "sha256:3274e35357e8ea4f6676aa6770daa6c1281a29d52d7503775fee19c575ed0fd8",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.282.tgz",
        bytes: 107_356_656,
        sha256: "sha256:1499a6947b466c2048a678b3291adf1e3f9cc3e3abd8b822f9e03ac85a2bb44b",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.282.tgz",
        bytes: 97_521_611,
        sha256: "sha256:738ce2deba0060eef3cdc55b7aaa6c4c45ecaacdd371bc81ac9088d638204efe",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-x64/-/claude-code-darwin-x64-2.1.282.tgz",
        bytes: 101_563_766,
        sha256: "sha256:4a0fef1d551fa533e848a29df9ea497871903649b5ecc9670067413cc3a20386",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-arm64/-/claude-code-win32-arm64-2.1.282.tgz",
        bytes: 106_114_350,
        sha256: "sha256:425e82a178b9124e140ed7eccb565381c18a9ca1caba29d4c2668c2bc15be580",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-x64/-/claude-code-win32-x64-2.1.282.tgz",
        bytes: 110_236_851,
        sha256: "sha256:34d84a836edd9cf96dbd3b20206bcc61227f48cb81f1b3688dac9e2c0dbc7623",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
];

/// Claude Code's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "2.1.283",
    command: "claude",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "2.1.282",
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
