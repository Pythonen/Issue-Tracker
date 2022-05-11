mod actions;
mod args;
mod util;
use args::CliArgs;
use clap::StructOpt;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let _ = args.parse_arguments();
}
