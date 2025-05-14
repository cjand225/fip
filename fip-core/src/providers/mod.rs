mod apt;

use anyhow::{Result, bail};
use self::apt::AptProvider;
use crate::config::Config;

pub trait Provider {
    fn name(&self) -> &'static str;
    fn install(&self, package: &str) -> Result<()>;
    fn remove(&self, package: &str) -> Result<()>;
    fn search(&self, package: &str) -> Result<()>;
}

pub fn resolve_provider(provider: Option<&str>, config: &Config) -> Result<Box<dyn Provider>> {
    let provider = provider.unwrap_or(&config.default_provider);
    match provider {
        "apt" => Ok(Box::new(AptProvider::new())),
        _ => bail!("Unsupported provider: {}", provider),
    }
}
