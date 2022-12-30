pub mod parse;

#[derive(Debug)]
pub struct Csv {
    pub headers: Option<Row>,
    pub rows: Vec<Row>,
}

pub type Row = Vec<String>;
