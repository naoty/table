use std::env;
use std::process;

const HELP_MESSAGE: &str = r#"
Usage:
  table -h | --help
  table -v | --version

Options:
  -h --help     Print help message
  -v --version  Print version
"#;

const VERSION: &str = "0.3.0";

fn main() {
    for (i, arg) in env::args().enumerate() {
        if i == 0 {
            continue;
        }

        match &*arg {
            "-h" | "--help" => {
                println!("{}", HELP_MESSAGE.trim());
                process::exit(0);
            }
            "-v" | "--version" => {
                println!("{}", VERSION);
                process::exit(0);
            }
            _ => println!("TODO: implement other flags"),
        }
    }
}
