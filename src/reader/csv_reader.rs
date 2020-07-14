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
  fn read(&mut self) -> crate::Result<crate::Table> {
    let mut table = crate::Table::new();

    for (i, result) in self.reader.records().enumerate() {
      let record = result?;
      let row: Vec<String> = record.iter().map(|field| String::from(field)).collect();
      if i == 0 && self.headers {
        table.headers = Some(row);
      } else {
        table.push_row(row);
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
        crate::Table::from(None, vec![vec![String::from("alice"), String::from("80")]]),
      ),
      (
        "alice\t80\nbob\t100",
        false,
        crate::Table::from(
          None,
          vec![
            vec![String::from("alice"), String::from("80")],
            vec![String::from("bob"), String::from("100")],
          ],
        ),
      ),
      (
        "name\tscore\nalice\t80",
        true,
        crate::Table::from(
          Some(vec![String::from("name"), String::from("score")]),
          vec![vec![String::from("alice"), String::from("80")]],
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
