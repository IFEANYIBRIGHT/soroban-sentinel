//! Command-line interface library for Soroban Sentinel.

#![forbid(unsafe_code)]

pub mod args;
pub mod commands;
pub mod config;
pub mod formatters;

pub use args::{Cli, Commands, ScanArgs};
pub use commands::{
    generate_schema, lint_manifest, schema_command,
    resolve_rpc_url, run_scan, EXIT_CRITICAL_OR_ARCHIVED, EXIT_OK, EXIT_RPC_FAILURE, EXIT_WARN,
};
pub use formatters::{render_json, render_text, JsonEntryReport, JsonSummary, JsonTTLReport};
