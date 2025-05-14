use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::providers::*;

pub fn handle_command(cli: Cli, config: Config) -> anyhow::Result<()> {
    match cli.command {
        Commands::Install { name, provider } => {
            let provider = resolve_provider(provider.as_deref(), &config)?;
            provider.install(&name)
        }
        Commands::Remove { name } => {
            let provider = resolve_provider(None, &config)?;
            provider.remove(&name)
        }
        Commands::Search { name } => {
            let provider = resolve_provider(None, &config)?;
            provider.search(&name)
        }
    }
}
