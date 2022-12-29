use super::*;

impl Csv {
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

    #[test]
    fn it_works() {
        println!(
            "{:?}",
            Csv::parse("super,duper\r\nepic,gamer\r\nnew,lines", true)
        );
    }
}
