use clap::{crate_version, App, Arg};
use std::io;
use std::process;
use table::{reader, writer};

const DESCRIPTION: &str = "A command to print ASCII table from stdin";
const USAGE_TEMPLATE: &str = r#"
Usage:
    {usage}

Flags:
{flags}

Options:
{options}
"#;

fn main() {
    let matches = App::new("table")
        .version(crate_version!())
        .author("Naoto Kaneko <naoty.k@gmail.com>")
        .about(DESCRIPTION)
        .template(USAGE_TEMPLATE.trim())
        .version_short("v")
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .takes_value(true)
                .default_value("")
                .value_name("FORMAT")
                .help("Config input/output data format"),
        )
        .arg(
            Arg::with_name("header")
                .short("H")
                .long("header")
                .help("Prints table with headers"),
        )
        .get_matches();

    let format = matches.value_of("format").unwrap_or_default();
    let mut tokens = format.split(":").take(2);

    let mut csv_reader = reader::CsvReader::new(io::stdin(), b',', matches.is_present("header"));
    let mut tsv_reader = reader::CsvReader::new(io::stdin(), b'\t', matches.is_present("header"));
    let reader: &mut dyn table::Read = match tokens.next() {
        Some("csv") => &mut csv_reader,
        _ => &mut tsv_reader,
    };

    let mut ascii_writer = writer::AsciiWriter::new(io::stdout());
    let writer: &mut dyn table::Write = match tokens.next() {
        _ => &mut ascii_writer,
    };

    let result = reader
        .read()
        .and_then(|table| writer.write(table))
        .and_then(|_| writer.flush());

    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(error) => {
            eprintln!("{:?}", error);
            process::exit(1);
        }
    }
}
