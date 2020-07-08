pub mod ascii_writer;
pub mod markdown_writer;

pub struct Table {
  rows: Vec<Vec<String>>,
  has_headers: bool,
}

pub struct Error {}

pub trait Read {
  fn read(&mut self) -> Result<Table, Error>;
}

pub trait Write {
  fn write(&self, table: Table) -> Result<(), Error>;
}
