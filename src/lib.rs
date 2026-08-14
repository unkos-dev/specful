//! Core Specful domain logic.

pub mod body;
pub mod diagnostics;
pub mod yaml;

mod schemas;

pub use schemas::{
    ADR_V1_SCHEMA_ID, CONFIG_V1_SCHEMA_ID, MSDD_V1_SCHEMA_ID, MSRS_V1_SCHEMA_ID, builtin_schema,
};
