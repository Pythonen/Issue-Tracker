mod args;
mod util;
mod actions;
use args::{CliArgs};
use clap::StructOpt;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let _ = args.parse_arguments();
    // get files from current directory
}


