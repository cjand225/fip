use super::Provider;
use anyhow::Result;
use std::process::Command;

pub struct AptProvider;

impl AptProvider {
    pub fn new() -> Self {
        Self
    }

    fn run_command(args: &[&str]) -> Result<()> {
        let status = Command::new("sudo") // Because apt usually requires sudo
            .arg("apt")
            .args(args)
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "APT command failed: apt {}",
                args.join(" ")
            ))
        }
    }
}

impl Provider for AptProvider {
    fn name(&self) -> &'static str {
        "apt"
    }

    fn install(&self, package: &str) -> Result<()> {
        println!("[apt] installing {package}");
        Self::run_command(&["install", "-y", package])
    }

    fn remove(&self, package: &str) -> Result<()> {
        println!("[apt] removing {package}");
        Self::run_command(&["remove", "-y", package])
    }

    fn search(&self, package: &str) -> Result<()> {
        println!("[apt] searching {package}");
        let status = Command::new("apt").args(&["search", package]).status()?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("APT search failed"))
        }
    }
}
