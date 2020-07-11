use unicode_width::UnicodeWidthStr;

pub mod reader;
pub mod writer;

#[derive(Debug, Eq, PartialEq)]
pub struct Table {
  rows: Vec<Vec<String>>,
  has_headers: bool,
}

impl Table {
  pub fn new() -> Self {
    Table::from(vec![], false)
  }

  pub fn with_headers(has_headers: bool) -> Self {
    Table::from(vec![], has_headers)
  }

  pub fn from(rows: Vec<Vec<String>>, has_headers: bool) -> Self {
    Table { rows, has_headers }
  }

  pub fn push_row(&mut self, row: Vec<String>) {
    self.rows.push(row);
  }

  pub fn column_len(&self) -> usize {
    self.rows.iter().fold(0, |num, row| num.max(row.len()))
  }

  pub fn column_widths(&self) -> Vec<usize> {
    let mut max_widths = vec![0; self.column_len()];

    for row in self.rows.iter() {
      for (i, cell) in row.iter().enumerate() {
        let max_width = max_widths.get(i).unwrap().clone();
        max_widths[i] = max_width.max(UnicodeWidthStr::width(cell as &str));
      }
    }

    max_widths
  }
}

#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
}

impl Error {
  pub fn read() -> Error {
    Error {
      kind: ErrorKind::Read,
    }
  }

  pub fn write() -> Error {
    Error {
      kind: ErrorKind::Write,
    }
  }
}

#[derive(Debug)]
pub enum ErrorKind {
  Read,
  Write,
}

pub trait Read {
  fn read(&mut self) -> Result<Table, Error>;
}

pub trait Write {
  fn write(&mut self, table: Table) -> Result<(), Error>;
  fn flush(&mut self) -> Result<(), Error>;
}

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
      let table = crate::Table::from(test_case.0, false);
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
      let table = crate::Table::from(test_case.0, false);
      assert_eq!(table.column_widths(), test_case.1);
    }
  }
}
