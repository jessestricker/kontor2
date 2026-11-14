use clap::{Parser, Subcommand};
use log::{debug, error};
use std::{
    io::Write,
    process::{ExitCode, Termination},
};

use crate::kontor::Kontor;

mod kontor;
mod nix;

/// Manage your home directory and environment.
#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Build the user environment and switch to it.
    Switch,
}

fn main() -> impl Termination {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .format(|buf, record| {
            let style = buf.default_level_style(record.level()).bold();
            writeln!(
                buf,
                "(kontor) {style}{}:{style:#} {}",
                record.level().as_str().to_ascii_lowercase(),
                record.args()
            )
        })
        .init();

    let cli = Cli::parse();
    debug!("{cli:?}");

    let kontor = Kontor::new();
    debug!("{kontor:?}");

    let result = match cli.command {
        Command::Switch => kontor.switch(),
    };

    result.map_or_else(
        |err| {
            error!("{err:?}");
            ExitCode::FAILURE
        },
        |_| ExitCode::SUCCESS,
    )
}
