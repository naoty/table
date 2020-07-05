use csv;
use io::Write;
use std::env;
use std::io;
use std::process;
use table::ascii_table_writer;

const HELP_MESSAGE: &str = r#"
Usage:
  table (-H | --header)
  table -h | --help
  table -v | --version

Options:
  -h --help     Print help message
  -H --header   Print table with headers
  -v --version  Print version
"#;

const VERSION: &str = "0.3.0";

fn main() {
    let mut has_headers = false;

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
            "-H" | "--header" => {
                has_headers = true;
            }
            _ => eprintln!("TODO: implement other flags"),
        }
    }

    if let Err(error) = start(has_headers) {
        eprintln!("{}", error);
        process::exit(0);
    }
}

fn start(has_headers: bool) -> io::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(io::stdin());

    let mut writer = ascii_table_writer::new(io::stdout(), has_headers);

    for result in reader.records() {
        if let Err(error) = result {
            eprintln!("failed to read TSV: {}", error);
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
