/// Canonical identifier for the built-in ADR profile version 1 schema.
pub const ADR_V1_SCHEMA_ID: &str = "https://unkos-dev.github.io/specful/schemas/adr/v1.schema.json";

/// Canonical identifier for the built-in Requirement profile version 1 schema.
pub const REQUIREMENT_V1_SCHEMA_ID: &str =
    "https://unkos-dev.github.io/specful/schemas/requirement/v1.schema.json";

/// Canonical identifier for the built-in Design profile version 1 schema.
pub const DESIGN_V1_SCHEMA_ID: &str =
    "https://unkos-dev.github.io/specful/schemas/design/v1.schema.json";

/// Canonical identifier for the built-in repository configuration version 1 schema.
pub const CONFIG_V1_SCHEMA_ID: &str =
    "https://unkos-dev.github.io/specful/schemas/config/v1.schema.json";

const ADR_V1_SCHEMA: &str = include_str!("../schemas/adr/v1.schema.json");
const REQUIREMENT_V1_SCHEMA: &str = include_str!("../schemas/requirement/v1.schema.json");
const DESIGN_V1_SCHEMA: &str = include_str!("../schemas/design/v1.schema.json");
const CONFIG_V1_SCHEMA: &str = include_str!("../schemas/config/v1.schema.json");

/// Returns a built-in schema by its canonical JSON Schema identifier.
#[must_use]
pub fn builtin_schema(canonical_id: &str) -> Option<&'static str> {
    match canonical_id {
        ADR_V1_SCHEMA_ID => Some(ADR_V1_SCHEMA),
        REQUIREMENT_V1_SCHEMA_ID => Some(REQUIREMENT_V1_SCHEMA),
        DESIGN_V1_SCHEMA_ID => Some(DESIGN_V1_SCHEMA),
        CONFIG_V1_SCHEMA_ID => Some(CONFIG_V1_SCHEMA),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ADR_V1_SCHEMA_ID, CONFIG_V1_SCHEMA_ID, DESIGN_V1_SCHEMA_ID, REQUIREMENT_V1_SCHEMA_ID,
        builtin_schema,
    };
    use serde_json::Value;

    const REGISTERED: [&str; 4] = [
        ADR_V1_SCHEMA_ID,
        REQUIREMENT_V1_SCHEMA_ID,
        DESIGN_V1_SCHEMA_ID,
        CONFIG_V1_SCHEMA_ID,
    ];

    #[test]
    fn resolves_builtin_schemas_only_by_canonical_id() {
        for id in REGISTERED {
            let schema_source =
                builtin_schema(id).unwrap_or_else(|| panic!("{id} should be registered"));
            let schema: Value = serde_json::from_str(schema_source)
                .unwrap_or_else(|error| panic!("{id} should contain valid JSON: {error}"));
            assert_eq!(schema["$id"], id);
        }

        assert!(builtin_schema("./schemas/adr/v1.schema.json").is_none());
        assert!(
            builtin_schema("https://unkos-dev.github.io/specful/schemas/adr/missing.json")
                .is_none()
        );
    }

    #[test]
    fn compiles_every_builtin_schema() {
        for id in REGISTERED {
            let schema: Value = serde_json::from_str(
                builtin_schema(id).unwrap_or_else(|| panic!("{id} should be registered")),
            )
            .unwrap_or_else(|error| panic!("{id} should contain valid JSON: {error}"));
            jsonschema::draft202012::options()
                .should_validate_formats(true)
                .build(&schema)
                .unwrap_or_else(|error| panic!("{id} should compile: {error}"));
        }
    }
}
