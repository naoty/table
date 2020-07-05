use clap::{crate_version, App, Arg};
use csv;
use io::Write;
use std::io;
use std::process;
use table::ascii_table_writer;

const DESCRIPTION: &str = "A command to print ASCII table from stdin";
const USAGE_TEMPLATE: &str = r#"
Usage:
    {usage}

Flags:
{flags}
"#;

fn main() {
    let matches = App::new("table")
        .version(crate_version!())
        .author("Naoto Kaneko <naoty.k@gmail.com>")
        .about(DESCRIPTION)
        .template(USAGE_TEMPLATE.trim())
        .version_short("v")
        .arg(
            Arg::with_name("has_headers")
                .short("H")
                .long("header")
                .help("Prints table with headers"),
        )
        .get_matches();

    if let Err(error) = start(matches.is_present("has_headers")) {
        eprintln!("{}", error);
        process::exit(1);
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
