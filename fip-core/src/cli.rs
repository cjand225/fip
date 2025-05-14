use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fip", version, about = "Fast Install Package")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Install {
        name: String,
        #[arg(short, long)]
        provider: Option<String>,
    },
    Remove {
        name: String,
    },
    Search {
        name: String,
    },
}
