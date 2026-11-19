#![forbid(unsafe_code)]

use clap::Parser;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

use sentinel_cli::args::{Cli, Commands};
use sentinel_cli::commands::{run_scan, EXIT_INVALID_ARGS};

#[tokio::main]
async fn main() {
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(err) => {
            if err.use_stderr() {
                let _ = err.print();
                std::process::exit(EXIT_INVALID_ARGS);
            } else {
                err.exit();
            }
        }
    };

    let needs_tracer = cli.verbose > 0 || cli.quiet;
    if needs_tracer {
        install_tracer(&cli);
    }

    let exit_code = match cli.command {
        Commands::Scan(args) => run_scan(args).await,
    };

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
