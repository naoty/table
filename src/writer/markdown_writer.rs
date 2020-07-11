use std::io;
use unicode_width::UnicodeWidthStr;

pub struct MarkdownWriter<T: io::Write> {
  writer: T,
}

impl<T: io::Write> MarkdownWriter<T> {
  pub fn new(writer: T) -> Self {
    MarkdownWriter { writer }
  }
}

impl<T: io::Write> crate::Write for MarkdownWriter<T> {
  fn write(&mut self, table: crate::Table) -> Result<(), crate::Error> {
    let column_widths = table.column_widths();

    let mut border = String::new();
    for column_width in column_widths.iter() {
      border += &format!("| {} ", "-".repeat(column_width.clone()));
    }
    border += "|\n";

    for (i, row) in table.rows.iter().enumerate() {
      for (j, value) in row.iter().enumerate() {
        let spaces = " ".repeat(column_widths[j] - UnicodeWidthStr::width(value as &str));
        let cell = format!("| {}{} ", value, spaces);

        self
          .writer
          .write(cell.as_bytes())
          .map_err(|_| crate::Error::write())?;
      }

      self
        .writer
        .write(b"|\n")
        .map_err(|_| crate::Error::write())?;

      if i == 0 {
        self
          .writer
          .write(border.as_bytes())
          .map_err(|_| crate::Error::write())?;
      }
    }

    Ok(())
  }

  fn flush(&mut self) -> Result<(), crate::Error> {
    self.writer.flush().map_err(|_| crate::Error::write())
  }
}
