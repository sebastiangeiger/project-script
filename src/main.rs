extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use config::CliOptions;
use config::Args;

mod subcommands;
mod config;

const USAGE: &'static str = "
Project script suite

Usage:
  project (-h | --help)
  project --version
  project list push [--config=<FILE>]
  project list pull [--config=<FILE>]
  project list sync [--config=<FILE>]

Options:
  -h --help       Show this screen.
  --version       Show version.
  --dry-run       Propose changes, don't execute.
  --config=<FILE>  Configuration file [default: ~/.projects].
";

#[allow(dead_code)]
fn main() {
    let args: config::Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let cli_options = CliOptions::from(&args).unwrap();
    let subcommand = find_subcommand(&args);
    subcommand(cli_options);
}

fn find_subcommand(args: &Args) -> fn(config::CliOptions){
    match args {
        &Args { cmd_push: true, cmd_list: true, .. } => subcommands::list_push,
        &Args { .. } => panic!("Not a valid command")
    }
}

