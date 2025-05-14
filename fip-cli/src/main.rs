use clap::Parser;
use fip_core::greet;

#[derive(Parser)]
#[command(name = "fip", version, about = "Fast Install Package")]
struct Cli {
    #[arg(short, long)]
    name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let name = cli.name.as_deref().unwrap_or("world");
    println!("{}", greet(name));
}
