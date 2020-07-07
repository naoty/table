use clap::{crate_version, App, Arg};
use csv::ReaderBuilder;
use std::io;
use std::process;
use table::{ascii_writer, markdown_writer};

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

    if let Err(error) = start(
        matches.value_of("format").unwrap(),
        matches.is_present("header"),
    ) {
        eprintln!("{}", error);
        process::exit(1);
    }
}

fn start(format: &str, has_headers: bool) -> io::Result<()> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_reader(io::stdin());

    let tokens = format.split(":").take(2);
    let mut ascii_writer = ascii_writer::new(io::stdout(), has_headers);
    let mut markdown_writer = markdown_writer::new(io::stdout());
    let writer: &mut dyn io::Write = match tokens.last() {
        Some("markdown") => &mut markdown_writer,
        _ => &mut ascii_writer,
    };

    for result in reader.records() {
        match result {
            Ok(record) => {
                for field in record.iter() {
                    writer.write(field.as_bytes())?;
                }
                writer.write(b"\n")?;
            }
            Err(error) => {
                eprintln!("failed to read record: {}", error);
                process::exit(1);
            }
        }
    }

    writer.flush()
}
