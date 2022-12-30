use super::*;

use std::fs::read_to_string;

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

fn get_raw(name: &str) -> String {
    let path = format!("files/{}.csv", name);

    read_to_string(path).unwrap()
}
