use super::*;

use std::{fs::read_to_string, path::Path};

impl Csv {
    pub fn parse_file<P: AsRef<Path>>(path: P, headers: bool) -> Csv {
        let raw = read_to_string(path).unwrap();

        Csv::parse(&raw, headers)
    }

    pub fn parse(raw: &str, headers: bool) -> Csv {
        let mut rows: Vec<Row> = vec![];

        // RFC specifies that CRLF returns are the correct delimiters
        for raw_row in raw.split("\r\n") {
            let mut row = Row::default();

            for field in raw_row.split(',') {
                row.push(field.into());
            }

            rows.push(row);
        }

        let headers = match headers {
            true => {
                let values = rows.first().unwrap().to_owned();

                rows.remove(0);

                Some(values)
            }
            false => None,
        };

        Csv { headers, rows }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_raw(name: &str) -> String {
        let path = format!("files/{}.csv", name);

        read_to_string(path).unwrap()
    }

    #[test]
    fn without_headers() {
        let result = Csv::parse(&get_raw("simple"), false);

        println!("{:?}", result);
    }

    #[test]
    fn with_headers() {
        let result = Csv::parse(&get_raw("simple"), true);

        println!("{:?}", result);
    }

    #[test]
    fn file_without_headers() {
        let result = Csv::parse_file("files/simple.csv", false);

        println!("{:?}", result);
    }

    #[test]
    fn file_with_headers() {
        let result = Csv::parse_file("files/simple.csv", true);

        println!("{:?}", result);
    }
}
