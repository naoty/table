use csv::{Reader, ReaderBuilder};
use std::io;

pub struct CsvReader<T> {
  reader: Reader<T>,
  headers: bool,
}

impl<T: io::Read> CsvReader<T> {
  pub fn new(reader: T, delimiter: u8, headers: bool) -> CsvReader<T> {
    let reader = ReaderBuilder::new()
      .delimiter(delimiter)
      .has_headers(false)
      .from_reader(reader);

    CsvReader { reader, headers }
  }
}

impl<T: io::Read> crate::Read for CsvReader<T> {
  fn read(&mut self) -> Result<crate::Table, crate::Error> {
    let mut table = crate::Table::with_headers(self.headers);

    for result in self.reader.records() {
      match result {
        Ok(record) => {
          let row: Vec<String> = record.iter().map(|field| String::from(field)).collect();
          table.push_row(row);
        }
        Err(_) => {
          return Err(crate::Error {
            kind: crate::ErrorKind::Read,
          });
        }
      }
    }

    Ok(table)
  }
}

mod tests {
  #[allow(unused)]
  use super::*;
  #[allow(unused)]
  use crate::Read;

  #[test]
  fn read() {
    let test_cases = vec![
      (
        "alice\t80",
        false,
        crate::Table::from(vec![vec![String::from("alice"), String::from("80")]], false),
      ),
      (
        "alice\t80\nbob\t100",
        false,
        crate::Table::from(
          vec![
            vec![String::from("alice"), String::from("80")],
            vec![String::from("bob"), String::from("100")],
          ],
          false,
        ),
      ),
      (
        "name\tscore\nalice\t80",
        true,
        crate::Table::from(
          vec![
            vec![String::from("name"), String::from("score")],
            vec![String::from("alice"), String::from("80")],
          ],
          true,
        ),
      ),
    ];

    for test_case in test_cases {
      let mut reader = CsvReader::new(test_case.0.as_bytes(), b'\t', test_case.1);
      let table = reader.read().expect("failed to read table");
      assert_eq!(table, test_case.2);
    }
  }
}
