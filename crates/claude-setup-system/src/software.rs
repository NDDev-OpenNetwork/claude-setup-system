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
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-arm64/-/claude-code-linux-arm64-2.1.270.tgz",
        bytes: 100_068_500,
        sha256: "sha256:9c2c52fc53e97cc8e5e20848865c53425193251e6764c2abe975c6642587e9f3",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.270.tgz",
        bytes: 99_604_884,
        sha256: "sha256:a2c0b69773f9da730b0616c19771094de51ffd23f0dfaa7c34ccead34eb76dd0",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.270.tgz",
        bytes: 89_845_026,
        sha256: "sha256:1233c82cf589f08ca89906a6ec5c074423d8e329fdf3a1f5eb9331da2a67d461",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-x64/-/claude-code-darwin-x64-2.1.270.tgz",
        bytes: 93_887_186,
        sha256: "sha256:aea6314b241d64d5590e56877ff5e4ed9f28bb9bc871e9b1d0e27d4fe3492584",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-arm64/-/claude-code-win32-arm64-2.1.270.tgz",
        bytes: 99_462_738,
        sha256: "sha256:3803595058dfe1a50c444c9f22f019f5484ec2c4c181f0c7afc6fc55451867a7",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-x64/-/claude-code-win32-x64-2.1.270.tgz",
        bytes: 102_507_269,
        sha256: "sha256:abd989279998f7a6604cfef5dc3b0219738f7b26a7b64ffcf4d172073c9f2648",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
];

/// The artifacts 2.1.263 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-arm64/-/claude-code-linux-arm64-2.1.263.tgz",
        bytes: 97_379_799,
        sha256: "sha256:64238be6b64968b17ce006fe15fb6e3a96b89ceeaa231fee4610cb34a521c44c",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-linux-x64/-/claude-code-linux-x64-2.1.263.tgz",
        bytes: 96_941_283,
        sha256: "sha256:8b6207348ad56fdcde085a0ad1f7cff0dfe06ce2c6c1bf97f69f1a1a7b6d0945",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-arm64/-/claude-code-darwin-arm64-2.1.263.tgz",
        bytes: 87_267_290,
        sha256: "sha256:f1c0d2da0e49acdb26f87d9d1a6fd036d03c1d31d9ae8a5682c453c8f127f32e",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-darwin-x64/-/claude-code-darwin-x64-2.1.263.tgz",
        bytes: 91_268_005,
        sha256: "sha256:9804d73f7e6dfe4d3c1f5cf0816499ea60d5d2e31099d210efe31d7e3fcfaf33",
        shape: Shape::GzipTar,
        member: "package/claude",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-arm64/-/claude-code-win32-arm64-2.1.263.tgz",
        bytes: 96_708_382,
        sha256: "sha256:f1eaf09f6285a061e94e0a0c5fddad9b64535d4a6346fe7c3ec421914997975f",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://registry.npmjs.org/@anthropic-ai/claude-code-win32-x64/-/claude-code-win32-x64-2.1.263.tgz",
        bytes: 99_788_276,
        sha256: "sha256:ced273be517e9afdbc91be4d7923d0bc8f7ce40e439c68db70b8d7015d0fe362",
        shape: Shape::GzipTar,
        member: "package/claude.exe",
    },
];

/// Claude Code's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "2.1.270",
    command: "claude",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "2.1.263",
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
