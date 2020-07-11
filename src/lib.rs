pub mod ascii_writer;
pub mod markdown_writer;
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
}

#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
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
  fn write(&self, table: Table) -> Result<(), Error>;
}
