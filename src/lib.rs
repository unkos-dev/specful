//! Core Specful domain logic.

pub mod authoring;
pub mod body;
pub mod config;
pub mod diagnostics;
pub mod frontmatter;
pub mod index;
pub mod query;
pub mod repo;
pub mod yaml;

mod profiles;
mod schemas;

pub use profiles::builtin_profile;
pub use schemas::{
    ADR_V1_SCHEMA_ID, CONFIG_V1_SCHEMA_ID, DESIGN_V1_SCHEMA_ID, REQUIREMENT_V1_SCHEMA_ID,
    builtin_schema,
};
