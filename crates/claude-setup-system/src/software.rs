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
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-arm64/-/claude-code-linux-arm64-2.1.296.tgz",
        bytes: 115_122_331,
        sha256: "sha256:56abbe1cfc85e5b37afeef5ae8b4856ddc32bbe443b82689a3b5f99992045cd1",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.296.tgz",
        bytes: 114_864_541,
        sha256: "sha256:eef5a2e2b09a5e7d2ce784d1b3ba335ae7fc6c7f6256c802a0ebf2c21e4fd0e1",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.296.tgz",
        bytes: 105_100_333,
        sha256: "sha256:50fbf594d6526c9aae7d66b8b45a08d89ae6b863e2bc9777bfb277d6521a7458",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-x64/-/claude-code-darwin-x64-2.1.296.tgz",
        bytes: 109_248_203,
        sha256: "sha256:ec29d962f0f7c70cf8c44606ee7fbb3cd7936a33f4dc13d3c2e3664c5f0b3fc4",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-arm64/-/claude-code-win32-arm64-2.1.296.tgz",
        bytes: 113_601_474,
        sha256: "sha256:7019eb71ba03dc70e76e7e909d5132192192948abe02bb18105ca34cae976547",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-x64/-/claude-code-win32-x64-2.1.296.tgz",
        bytes: 117_726_124,
        sha256: "sha256:f04bb1c18698dc6562912a573b031306225f5e202df4fe6bd2317351f5e492c5",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
];

/// The artifacts 2.1.288 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-arm64/-/claude-code-linux-arm64-2.1.288.tgz",
        bytes: 110_330_238,
        sha256: "sha256:a4e46f12f30ec01b978e3aa1a78fbc9c2fdd3a6478d0cc068c4a8f7d23cb5b75",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.288.tgz",
        bytes: 110_072_430,
        sha256: "sha256:b54d6a2c6bdd78f2c83139d1223598641686aab9e562c47cf537dd200653fa90",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.288.tgz",
        bytes: 100_276_471,
        sha256: "sha256:7cb1cbd826e6aa5ffc347e15193776c59373f840f40ec0fe3acfdcae342864da",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-x64/-/claude-code-darwin-x64-2.1.288.tgz",
        bytes: 104_351_206,
        sha256: "sha256:eb30a25c70fc07643d6da84a4bab97ea61fb8aae4e73af1ea4d1b9e8dbd453dc",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-arm64/-/claude-code-win32-arm64-2.1.288.tgz",
        bytes: 108_829_701,
        sha256: "sha256:d9c0e05de9ce0c2eb6075418153efc429c35eed9098bbd704f970b750905a5e7",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-x64/-/claude-code-win32-x64-2.1.288.tgz",
        bytes: 112_945_062,
        sha256: "sha256:1c3f4aa857914e6801520d566025d2f1970a70842026bfc4c63da306becbcb48",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
];

/// Claude Code's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "2.1.296",
    command: "claude",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "2.1.288",
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
