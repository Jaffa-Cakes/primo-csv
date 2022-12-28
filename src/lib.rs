#[derive(Debug)]
pub struct Csv {
    pub headers: Option<Vec<String>>,
    pub lines: Vec<Vec<String>>,
}

pub fn parse(raw: &str, headers: bool) -> Csv {
    let mut lines: Vec<Vec<String>> = vec![];

    // RFC specifies that CRLF returns are the correct delimiters
    for raw_line in raw.split("\r\n") {
        let mut line: Vec<String> = vec![];

        for field in raw_line.split(',') {
            line.push(field.into());
        }

        lines.push(line);
    }

    let headers = match headers {
        true => {
            let values = lines.first().unwrap().to_owned();

            lines.remove(0);

            Some(values)
        }
        false => None,
    };

    Csv { headers, lines }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!(
            "{:?}",
            parse("super,duper\r\nepic,gamer\r\nnew,lines", true)
        );
    }
}
