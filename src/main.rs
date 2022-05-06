mod args;
mod util;
mod actions;
use args::parse_arguments;

#[tokio::main]
async fn main() {
    parse_arguments();
    // get files from current directory
}


