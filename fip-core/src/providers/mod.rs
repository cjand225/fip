mod apt;
mod appimage;

use self::apt::AptProvider;
use self::appimage::AppImageProvider;
use crate::config::Config;
use anyhow::{Result, bail};
use std::path::Path;

pub trait Provider {
    fn name(&self) -> &'static str;
    fn install(&self, package: &str) -> Result<()>;
    fn remove(&self, package: &str) -> Result<()>;
    fn search(&self, package: &str) -> Result<()>;
}

pub fn resolve_provider(provider: Option<&str>, config: &Config) -> Result<Box<dyn Provider>> {
    match provider {
        Some("apt") => Ok(Box::new(AptProvider::new())),
        Some("appimage") => Ok(Box::new(AppImageProvider::new(config))),
        Some(p) => bail!("Unsupported provider: {}", p),
        None => {
            // If no provider specified, check if the package path is an AppImage
            if let Some(package_path) = config.package_path.as_deref() {
                if AppImageProvider::is_appimage(Path::new(package_path))? {
                    return Ok(Box::new(AppImageProvider::new(config)));
                }
            }
            // Default to the configured default provider
            match config.default_provider.as_str() {
                "apt" => Ok(Box::new(AptProvider::new())),
                "appimage" => Ok(Box::new(AppImageProvider::new(config))),
                p => bail!("Unsupported default provider: {}", p),
            }
        }
    }
}
