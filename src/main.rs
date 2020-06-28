use std::env;

const HELP_MESSAGE: &str = r#"
Usage:
  table -h | --help

Options:
  -h --help  Print help message
"#;

fn main() {
    for arg in env::args() {
        match &*arg {
            "-h" | "--help" => println!("{}", HELP_MESSAGE.trim()),
            _ => println!("TODO: implement other flags"),
        }
    }
}
