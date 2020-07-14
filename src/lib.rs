use std::{error, fmt, result};
use unicode_width::UnicodeWidthStr;

pub mod reader;
pub mod writer;

#[derive(Debug, Eq, PartialEq)]
pub struct Table {
  headers: Option<Vec<String>>,
  rows: Vec<Vec<String>>,
}

impl Table {
  pub fn new() -> Self {
    Table::from(None, vec![])
  }

  pub fn with_headers(headers: Option<Vec<String>>) -> Self {
    Table::from(headers, vec![])
  }

  pub fn from(headers: Option<Vec<String>>, rows: Vec<Vec<String>>) -> Self {
    Table { rows, headers }
  }

  pub fn push_row(&mut self, row: Vec<String>) {
    self.rows.push(row);
  }

  pub fn column_len(&self) -> usize {
    let column_len = self.rows.iter().fold(0, |num, row| num.max(row.len()));
    match &self.headers {
      Some(headers) => column_len.max(headers.len()),
      None => column_len,
    }
  }

  pub fn column_widths(&self) -> Vec<usize> {
    let mut max_widths = vec![0; self.column_len()];

    for row in self.rows.iter() {
      for (i, cell) in row.iter().enumerate() {
        let max_width = max_widths.get(i).unwrap().clone();
        max_widths[i] = max_width.max(UnicodeWidthStr::width(cell as &str));
      }
    }

    if let Some(headers) = &self.headers {
      for (i, cell) in headers.iter().enumerate() {
        let max_width = max_widths.get(i).unwrap().clone();
        max_widths[i] = max_width.max(UnicodeWidthStr::width(cell as &str));
      }
    }

    max_widths
  }
}

pub trait Read {
  fn read(&mut self) -> Result<Table>;
}

pub trait Write {
  fn write(&mut self, table: Table) -> Result<()>;
  fn flush(&mut self) -> Result<()>;
}

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
  InvalidInput,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::InvalidInput => write!(f, "invalid input"),
    }
  }
}

impl error::Error for Error {}

mod tests {
  #[test]
  fn table_column_len() {
    let test_cases = vec![
      (vec![], 0),
      (vec![vec![]], 0),
      (vec![vec![String::from("alice")]], 1),
      (vec![vec![String::from("alice"), String::from("100")]], 2),
      (
        vec![
          vec![String::from("alice"), String::from("100")],
          vec![String::from("bob")],
        ],
        2,
      ),
    ];

    for test_case in test_cases {
      let table = crate::Table::from(None, test_case.0);
      assert_eq!(table.column_len(), test_case.1);
    }
  }

  #[test]
  fn table_column_widths() {
    let test_cases = vec![
      (vec![], vec![]),
      (vec![vec![]], vec![]),
      (vec![vec![String::from("alice")]], vec![5]),
      (
        vec![vec![String::from("alice"), String::from("80")]],
        vec![5, 2],
      ),
      (
        vec![
          vec![String::from("alice"), String::from("80")],
          vec![String::from("bob"), String::from("100")],
        ],
        vec![5, 3],
      ),
      (
        vec![
          vec![String::from("alice"), String::from("80")],
          vec![String::from("ボブ"), String::from("100点")],
        ],
        vec![5, 5],
      ),
    ];

    for test_case in test_cases {
      let table = crate::Table::from(None, test_case.0);
      assert_eq!(table.column_widths(), test_case.1);
    }
  }
}
