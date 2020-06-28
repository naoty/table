use std::env;

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
            "-h" | "--help" => println!("{}", HELP_MESSAGE.trim()),
            "-v" | "--version" => println!("{}", VERSION),
            _ => println!("TODO: implement other flags"),
        }
    }
}
