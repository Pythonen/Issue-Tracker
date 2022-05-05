use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct CliArgs {
    /// Initialize a new project with this argument
    init: Option<String>,
}

pub fn parse_arguments() {
    let args = CliArgs::parse();

    if args.init.is_some() {
        println!("Initializing project...");
    }
}