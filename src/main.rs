use csv;
use io::Write;
use std::env;
use std::io;
use std::process;

mod ascii_table_writer;

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

    if let Err(error) = start() {
        println!("{}", error);
        process::exit(0);
    }
}

fn start() -> io::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(io::stdin());

    let mut writer = ascii_table_writer::new(io::stdout());

    for result in reader.records() {
        if let Err(error) = result {
            println!("failed to read TSV: {}", error);
            process::exit(1);
        }

        let record = result.unwrap();

        for field in record.iter() {
            writer.write(field.as_bytes())?;
        }

        writer.write(b"\n")?;
    }

    writer.flush()
}
