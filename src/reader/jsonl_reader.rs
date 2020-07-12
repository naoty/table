use std::io;

pub struct JsonlReader<T: io::Read> {
  reader: T,
}

impl<T: io::Read> JsonlReader<T> {
  pub fn new(reader: T) -> Self {
    JsonlReader { reader }
  }
}

impl<T: io::Read> crate::Read for JsonlReader<T> {
  fn read(&mut self) -> Result<crate::Table, crate::Error> {
    let table = crate::Table::new();
    Ok(table)
  }
}
