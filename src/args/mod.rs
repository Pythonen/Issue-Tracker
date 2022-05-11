use clap::Parser;

use super::actions;
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct CliArgs {
    // Find all todos in the project
    #[clap(subcommand)]
    cmds: SubCommand,
}
#[derive(clap::Subcommand, Debug, PartialEq)]
enum SubCommand {
    Report,
    Print,
    Init,
    Login,
}

impl CliArgs {
    pub async fn parse_arguments(self) -> Result<(), String> {
        if self.cmds == SubCommand::Init {
            // TODO: Check that the operation succeeds
            actions::init_new_project();
            return Ok(());
        } else if self.cmds == SubCommand::Print {
            // TODO: Check that the operation succeeds
            actions::print_todos();
            return Ok(());
        } else if self.cmds == SubCommand::Report {
            // TODO: Check that the operation succeeds !! IMPORTANT !!
            actions::report_todos().await;
            return Ok(());
        } else if self.cmds == SubCommand::Login {
            // TODO: Check that the operation succeeds !! IMPORTANT !!
            actions::login();
            return Ok(());
        }
        return Err("Unknown command".to_string());
    }
}

#[cfg(test)]
mod tests {

    use super::{CliArgs, SubCommand};
    #[tokio::test]
    async fn test_parse_arguments_with_init() {
        let args = CliArgs {
            cmds: SubCommand::Init,
        };
        let parsed = args.parse_arguments().await;
        assert!(parsed.is_ok());
    }

    #[tokio::test]
    #[should_panic]
    // TODO: This should panic only for now as this command is not yet implemented
    async fn test_parse_arguments_with_report() {
        let args = CliArgs {
            cmds: SubCommand::Report,
        };
        let parsed = args.parse_arguments().await;
        assert!(parsed.is_err());
    }

    #[tokio::test]
    async fn test_parse_arguments_with_print() {
        let args = CliArgs {
            cmds: SubCommand::Print,
        };
        let parsed = args.parse_arguments().await;
        assert!(parsed.is_ok());
    }
}
