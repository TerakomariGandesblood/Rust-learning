// CLI (command line interface)

use std::fmt::Write;
use std::thread;
use std::time::Duration;
use std::{env, fs, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;
use clap_verbosity_flag::Verbosity;
use human_panic::setup_panic;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing_log::log;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::LocalTime;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug, Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: PathBuf,

    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Config {
    version: u8,
}

fn app_name() -> String {
    let path = env::current_exe().unwrap();
    path.file_name().unwrap().to_str().unwrap().to_string()
}

fn main() -> Result<()> {
    setup_panic!(Metadata {
        name: env!("CARGO_PKG_NAME").into(),
        version: env!("CARGO_PKG_VERSION").into(),
        authors: env!("CARGO_PKG_AUTHORS").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
    });

    let args = Cli::parse();

    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_max_level(convert_filter(args.verbose.log_level_filter()))
        .init();

    info!("info");
    tracing::debug!("debug");
    tracing::trace!("trace");
    tracing::error!("error");
    tracing::warn!("warn");

    let app_name = app_name();
    let cfg: Config = confy::load(&app_name, None)?;
    confy::store(&app_name, None, cfg)?;

    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", &args.path.to_str().unwrap()))?;

    let pb = ProgressBar::new(5);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    for _ in 0..5 {
        pb.inc(1);
        thread::sleep(Duration::from_millis(200));
    }
    pb.finish_with_message("done");

    cli::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}

fn convert_filter(filter: log::LevelFilter) -> LevelFilter {
    match filter {
        log::LevelFilter::Off => LevelFilter::OFF,
        log::LevelFilter::Error => LevelFilter::ERROR,
        log::LevelFilter::Warn => LevelFilter::WARN,
        log::LevelFilter::Info => LevelFilter::INFO,
        log::LevelFilter::Debug => LevelFilter::DEBUG,
        log::LevelFilter::Trace => LevelFilter::TRACE,
    }
}
