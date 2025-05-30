use anyhow::Result;
use fip_core::{
    cli::{Cli, Commands},
    run,
};

#[test]
fn test_basic_run() -> Result<()> {
    let cli = Cli {
        command: Commands::Search {
            name: "test-package".to_string(),
        },
    };

    let result = run(cli);
    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_config_loading() -> Result<()> {
    use fip_core::config;

    let result = config::load_config();
    assert!(result.is_ok());

    Ok(())
}
