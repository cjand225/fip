use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_provider: String,
}

pub fn load_config() -> anyhow::Result<Config> {
    // TODO: Load config based on platform
    Ok(Config {
        default_provider: "apt".into(),
    })
}
