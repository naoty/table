use std::io;

pub struct AsciiTableWriter<T: io::Write> {
  inner_writer: T,
  records: Vec<Vec<String>>,
}

pub fn new<T: io::Write>(writer: T) -> AsciiTableWriter<T> {
  AsciiTableWriter {
    inner_writer: writer,
    records: vec![vec![]],
  }
}

impl<T: io::Write> io::Write for AsciiTableWriter<T> {
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

    self.inner_writer.write(b"+------------+\n")?;

    for record in &self.records {
      for field in record {
        let cell = format!("| {} ", field);
        self.inner_writer.write(cell.as_bytes())?;
      }

      self.inner_writer.write(b"|\n")?;
    }

    self.inner_writer.write(b"+------------+\n")?;
    self.inner_writer.flush()
  }
}
