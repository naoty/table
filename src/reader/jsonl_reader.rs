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
  fn read(&mut self) -> crate::Result<crate::Table> {
    let mut table = crate::Table::new();

    let mut buffer = String::new();
    self.reader.read_to_string(&mut buffer)?;

    let mut headers: Vec<String> = vec![];
    for line in buffer.split("\n") {
      let mut row: Vec<String> = vec![];

      let json_value: serde_json::Value = serde_json::from_str(line)?;
      let json_object = json_value.as_object().ok_or(crate::Error::InvalidInput)?;

      for (key, value) in json_object {
        let index = match headers.iter().position(|header| header == key) {
          Some(index) => index,
          None => {
            headers.push(key.clone());
            headers.len() - 1
          }
        };

        if index >= row.len() {
          row.resize_with(index + 1, Default::default);
          row[index] = match value {
            serde_json::Value::String(string) => string.clone(),
            _ => format!("{}", value),
          };
        }
      }

      table.push_row(row);
    }

    table.headers = Some(headers);
    Ok(table)
  }
}

mod tests {
  #[allow(unused)]
  use super::*;
  #[allow(unused)]
  use crate::Read;

  #[test]
  fn read() {
    let test_cases = vec![
      (
        r#"{"name":"alice","score":80}"#,
        crate::Table::from(
          Some(vec![String::from("name"), String::from("score")]),
          vec![vec![String::from("alice"), String::from("80")]],
        ),
      ),
      (
        r#"{"name":"alice","score":80}
        {"name":"bob","score":70}"#,
        crate::Table::from(
          Some(vec![String::from("name"), String::from("score")]),
          vec![
            vec![String::from("alice"), String::from("80")],
            vec![String::from("bob"), String::from("70")],
          ],
        ),
      ),
    ];

    for test_case in test_cases {
      let mut reader = JsonlReader::new(test_case.0.as_bytes());
      let table = reader.read().expect("failed to read");
      assert_eq!(table, test_case.1);
    }
  }
}
