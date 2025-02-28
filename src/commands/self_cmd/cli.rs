use anyhow::Result;
use clap::{Args, Subcommand};

/// Manage this application
#[derive(Args, Debug)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Metadata(super::metadata::Cli),
    Restore(super::restore::Cli),
    Update(super::update::Cli),
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.command {
            Commands::Metadata(cli) => cli.exec(),
            Commands::Restore(cli) => cli.exec(),
            Commands::Update(cli) => cli.exec(),
        }
    }
}
