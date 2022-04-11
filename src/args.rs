use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    // Initialize project with 
    init: Option<String>,
}
