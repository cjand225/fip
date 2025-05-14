use std::process::Command;

fn main() {
    println!("Launching FIP GUI...");

    let output = Command::new("cargo")
        .args(["run", "-p", "fip-cli", "--", "--name", "GUI"])
        .output()
        .expect("Failed to run CLI");

    println!("CLI output: {}", String::from_utf8_lossy(&output.stdout));
}
