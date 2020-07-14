use std::io;
use unicode_width::UnicodeWidthStr;

pub struct AsciiWriter<T: io::Write> {
  writer: T,
}

impl<T: io::Write> AsciiWriter<T> {
  pub fn new(writer: T) -> Self {
    AsciiWriter { writer }
  }
}

impl<T: io::Write> crate::Write for AsciiWriter<T> {
  fn write(&mut self, table: crate::Table) -> crate::Result<()> {
    let column_widths = table.column_widths();

    let mut border = String::new();
    for width in column_widths.iter() {
      border += &format!("+-{}-", "-".repeat(width.clone()));
    }
    border += "+\n";

    self.writer.write(border.as_bytes())?;

    if let Some(headers) = table.headers {
      for (j, value) in headers.iter().enumerate() {
        let spaces = " ".repeat(column_widths[j] - UnicodeWidthStr::width(value as &str));
        let cell = format!("| {}{} ", value, spaces);
        self.writer.write(cell.as_bytes())?;
      }

      self.writer.write(b"|\n")?;
      self.writer.write(border.as_bytes())?;
    }

    for row in table.rows.iter() {
      for (j, value) in row.iter().enumerate() {
        let spaces = " ".repeat(column_widths[j] - UnicodeWidthStr::width(value as &str));
        let cell = format!("| {}{} ", value, spaces);
        self.writer.write(cell.as_bytes())?;
      }

      self.writer.write(b"|\n")?;
    }

    self.writer.write(border.as_bytes())?;
    Ok(())
  }

  fn flush(&mut self) -> crate::Result<()> {
    self.writer.flush()?;
    Ok(())
  }
}
