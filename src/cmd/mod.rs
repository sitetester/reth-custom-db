pub mod export;
pub mod import;

use clap::Subcommand;
use reth_cli_runner::CliRunner;
use reth_ethereum::cli::ExtendedCommand;

#[derive(Debug, Subcommand)]
pub enum EntityCommands {
    #[command(name = "entity-export")]
    Export(export::DbExportCommand),

    #[command(name = "entity-import")]
    Import(import::DbImportCommand),
}

impl ExtendedCommand for EntityCommands {
    fn execute(self, _runner: CliRunner) -> eyre::Result<()> {
        match self {
            Self::Export(cmd) => cmd.export(),
            Self::Import(cmd) => cmd.import(),
        }
    }
}
