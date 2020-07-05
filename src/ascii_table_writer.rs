use std::io;
use unicode_width::UnicodeWidthStr;

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

impl<T: io::Write> AsciiTableWriter<T> {
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

    let column_widths = self.column_widths();

    let mut border = String::from("");
    for width in column_widths.iter() {
      border += &format!("+-{}-", "-".repeat(*width));
    }
    border += "+\n";

    self.inner_writer.write(border.as_bytes())?;

    for record in self.records.iter() {
      for (i, field) in record.iter().enumerate() {
        let column_width = column_widths[i];
        let spaces = " ".repeat(column_width - UnicodeWidthStr::width(field as &str));
        let cell = format!("| {field}{spaces} ", field = field, spaces = spaces);
        self.inner_writer.write(cell.as_bytes())?;
      }

      self.inner_writer.write(b"|\n")?;
    }

    self.inner_writer.write(border.as_bytes())?;
    self.inner_writer.flush()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use io::Write;
  use std::io;

  #[test]
  fn column_widths() {
    struct TestCase<'a> {
      fields: Vec<&'a [u8]>,
      expected: Vec<usize>,
    }

    let test_cases = vec![
      TestCase {
        fields: vec![b"alice", b"80", b"\n", b"bob", b"100"],
        expected: vec![5, 3],
      },
      TestCase {
        fields: vec![b"alice", b"80", b"90", b"\n", b"bob", b"100", b"0"],
        expected: vec![5, 3, 2],
      },
      TestCase {
        fields: vec![
          b"alice",
          b"80",
          b"\n",
          "ボブ".as_bytes(),
          "100点".as_bytes(),
          b"\n",
        ],
        expected: vec![5, 5],
      },
    ];

    for test_case in test_cases {
      let mut writer = new(io::sink());

      for field in test_case.fields {
        writer.write(field).expect("failed to write");
      }

      assert_eq!(test_case.expected, writer.column_widths());
    }
  }
}
