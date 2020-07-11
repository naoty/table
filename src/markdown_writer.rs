use std::io;
use unicode_width::UnicodeWidthStr;

pub struct MarkdownWriter<T: io::Write> {
  writer: T,
  records: Vec<Vec<String>>,
}

pub fn new<T: io::Write>(writer: T) -> MarkdownWriter<T> {
  MarkdownWriter {
    writer,
    records: vec![vec![]],
  }
}

impl<T: io::Write> MarkdownWriter<T> {
  fn column_widths(&self) -> Vec<usize> {
    let mut widths: Vec<usize> = self
      .records
      .first()
      .unwrap_or(&vec![])
      .iter()
      .map(|field| UnicodeWidthStr::width(field as &str))
      .collect();

    for record in self.records.iter() {
      for (i, field) in record.iter().enumerate() {
        widths[i] = widths[i].max(UnicodeWidthStr::width(field as &str));
      }
    }

    widths
  }
}

impl<T: io::Write> io::Write for MarkdownWriter<T> {
  fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
    let field = String::from_utf8(buf.to_vec());
    let field = field.map_err(|_| io::Error::from(io::ErrorKind::InvalidData))?;

    if field == "\n" {
      self.records.push(vec![]);
    } else {
      self.records.last_mut().unwrap().push(field);
    }

    Ok(buf.len())
  }

  fn flush(&mut self) -> io::Result<()> {
    if self.records.last().unwrap().is_empty() {
      let _ = self.records.pop();
    }

    let column_widths = self.column_widths();

    let mut border = String::from("");
    for width in column_widths.iter() {
      border += &format!("| {} ", "-".repeat(*width));
    }
    border += "|\n";

    for (i, record) in self.records.iter().enumerate() {
      for (j, field) in record.iter().enumerate() {
        let column_width = column_widths[j];
        let spaces = " ".repeat(column_width - UnicodeWidthStr::width(field as &str));
        let cell = format!("| {field}{spaces} ", field = field, spaces = spaces);
        self.writer.write(cell.as_bytes())?;
      }

      self.writer.write(b"|\n")?;

      if i == 0 {
        self.writer.write(border.as_bytes())?;
      }
    }

    self.writer.flush()
  }
}