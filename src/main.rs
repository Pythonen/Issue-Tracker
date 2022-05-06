mod args;
mod util;
mod actions;
use args::{parse_arguments, CliArgs};
use clap::StructOpt;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let _ = parse_arguments(args);
    // get files from current directory
}


