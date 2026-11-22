use crate::body::ArtifactKind;

const ADR_PROFILE: &str = include_str!("../profiles/adr.md");
const DESIGN_PROFILE: &str = include_str!("../profiles/design.md");
const REQUIREMENT_PROFILE: &str = include_str!("../profiles/requirement.md");

/// Returns the canonical Markdown profile for an artifact kind.
#[must_use]
pub fn builtin_profile(kind: ArtifactKind) -> &'static str {
    match kind {
        ArtifactKind::Adr => ADR_PROFILE,
        ArtifactKind::Design => DESIGN_PROFILE,
        ArtifactKind::Requirement => REQUIREMENT_PROFILE,
    }
}
