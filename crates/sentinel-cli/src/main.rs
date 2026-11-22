#![forbid(unsafe_code)]

use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use sentinel_cli::args::{Cli, Commands};
use sentinel_cli::commands;
use sentinel_cli::telemetry::{self, TelemetryConfig};

#[tokio::main]
async fn main() {
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(err) => {
            if err.use_stderr() {
                let _ = err.print();
                std::process::exit(commands::EXIT_INVALID_ARGS);
            } else {
                err.exit();
            }
        }
    };

    let needs_tracer = cli.verbose > 0 || cli.quiet;
    if needs_tracer {
        install_tracer(&cli);
    }

    let telemetry = telemetry::init(&TelemetryConfig::resolve(cli.otlp_endpoint.as_deref()));

    let exit_code = match cli.command {
        Commands::Check(args) => commands::run_check(args).await,
        Commands::CostMatrix(args) => commands::run_cost_matrix(args).await,
        Commands::Scan(args) => commands::run_scan(args).await,
        Commands::Bundle(args) => commands::run_bundle(args).await,
        Commands::AuditKeys(args) => commands::run_audit_keys(args).await,
        Commands::Restore(args) => commands::run_restore(args).await,
        Commands::InstallService(args) => commands::run_install_service(args),
        Commands::Init(args) => commands::run_init(args),
        Commands::KeyEncode(args) => commands::run_key_encode(args),
        Commands::KeyDecode(args) => commands::run_key_decode(args),
        Commands::DiffReports(args) => commands::run_diff_reports(args),
        Commands::Forecast(args) => commands::run_forecast(args).await,
    };

    // Flush buffered spans/metrics before exiting, since `process::exit`
    // bypasses destructors.
    telemetry.shutdown();
    std::process::exit(exit_code);
}

fn install_tracer(cli: &Cli) {
    let level = if cli.quiet {
        "error"
    } else {
        match cli.verbose {
            0 => "off",
            1 => "info",
            _ => "debug",
        }
    };

    let filter = EnvFilter::try_new(level).unwrap_or_else(|_| EnvFilter::new("off"));
    let registry = tracing_subscriber::registry().with(filter);

    if cli.log_format == "json" {
        let layer = fmt::layer()
            .json()
            .with_writer(std::io::stderr)
            .with_target(false)
            .with_ansi(false);
        let _ = registry.with(layer).try_init();
    } else {
        let layer = fmt::layer()
            .with_writer(std::io::stderr)
            .with_target(false)
            .with_ansi(false);
        let _ = registry.with(layer).try_init();
    }
}
