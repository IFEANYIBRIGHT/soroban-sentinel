//! CLI command execution modules.

pub mod check;
pub mod scan;
pub mod benchmark_rpc;

pub use check::run_check;

pub use scan::{
    resolve_rpc_url, run_scan, EXIT_CRITICAL_OR_ARCHIVED, EXIT_INVALID_ARGS, EXIT_OK,
    EXIT_RPC_FAILURE, EXIT_WARN,
};
pub use benchmark_rpc::{EXIT_BENCHMARK_OK, EXIT_BENCHMARK_ERROR, run_benchmark};
