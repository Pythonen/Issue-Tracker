use clap::Parser;

use super::actions;
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct CliArgs {
    // Find all todos in the project
    #[clap(subcommand)]
    cmds: SubCommand
}
#[derive(clap::Subcommand, Debug, PartialEq)]
enum SubCommand {
    Report,
    Print,
    Init
}

pub fn parse_arguments() {
    let args = CliArgs::parse();
    if args.cmds == SubCommand::Init {
        actions::init_new_project();
    }
}