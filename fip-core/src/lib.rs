pub mod cli;
pub mod config;
pub mod dispatch;
pub mod providers;

use cli::Cli;

pub fn run(cli: Cli) -> anyhow::Result<()> {
    let config = config::load_config()?;
    dispatch::handle_command(cli, config)
}
