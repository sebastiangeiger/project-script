extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

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

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_dry_run: bool,
    arg_name: Vec<String>,
    cmd_list: bool,
    cmd_push: bool,
    flag_config: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
