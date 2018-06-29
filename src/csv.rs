use std::error::Error;

pub type CsvType<'a> = Vec<Vec<&'a str>>;

pub fn parse<'a>(input: &'a str) -> Result<CsvType, Box<Error>> {
    let lines = input.trim().split("\n");

    let csv: Vec<Vec<&str>> = lines
        .map(|line| line.split(", ").collect::<Vec<&str>>())
        .collect();

    Ok(csv)
}
