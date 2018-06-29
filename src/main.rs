extern crate clap;

use std::error::Error;
use std::io::{self, BufRead};

use clap::App;

type Csv<'a> = Vec<Vec<&'a str>>;

fn main() {
    App::new("rust-cli-table")
        .version("0.1.0")
        .author("Tom A. <tom.arrell@gmail.com>")
        .about("Renders a table in the command line from a CSV")
        .get_matches();

    let mut buffer = String::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(x) => {
                buffer.push_str(&format!("{}\n", &x));
            }
            // Add better error handling
            Err(_) => (),
        }
    }

    let parsed_data: Csv = match parse_csv(&buffer) {
        Ok(x) => x,
        Err(_) => panic!("Failed to parse stdin CSV"),
    };

    let max_column_width_matrix = max_column_width(parsed_data.clone());

    let first_row = match parsed_data.get(0) {
        Some(x) => x,
        None => panic!("Failed to get header"),
    };

    let data_rest = match parsed_data.get(1..) {
        Some(x) => x,
        None => panic!("Failed to get rest of data"),
    };

    render_header(first_row.to_vec(), max_column_width_matrix.clone());
    render_data(data_rest.to_vec(), max_column_width_matrix.clone());
}

fn parse_csv<'a>(input: &'a str) -> Result<Vec<Vec<&str>>, Box<Error>> {
    let lines = input.trim().split("\n");

    let csv: Vec<Vec<&str>> = lines
        .map(|line| line.split(", ").collect::<Vec<&str>>())
        .collect();

    Ok(csv)
}

fn max_column_width<'a>(table: Csv) -> Vec<usize> {
    let col_count = match table.get(0) {
        Some(x) => x.len(),
        None => panic!("Failed to get column count from first row"),
    };

    let mut width_matrix = vec![0; col_count];

    for row in table {
        for (i, item) in row.iter().enumerate() {
            let curr_max_width = match width_matrix.get_mut(i) {
                Some(x) => x,
                None => continue,
            };

            if curr_max_width < &mut item.len() {
                *curr_max_width = item.len();
            }
        }
    }

    width_matrix
}

fn render_data(data: Csv, max_width_matrix: Vec<usize>) -> usize {
    let mut data_string = String::from("");

    for row in data {
        for (index, cell) in row.iter().enumerate() {
            data_string.push_str(&format!(
                "| {:width$} ",
                cell,
                width = max_width_matrix.get(index).unwrap_or(&1)
            ));
        }
        data_string.push_str("|\n");
    }

    print!("{}", data_string);

    let abs_width = data_string
        .as_str()
        .split("\n")
        .next()
        .unwrap_or("")
        .chars()
        .count();

    print_spacer(abs_width);

    abs_width
}

fn render_header(headers: Vec<&str>, max_width_matrix: Vec<usize>) -> usize {
    let mut header_string = String::from("");

    for (index, h) in headers.iter().enumerate() {
        header_string.push_str(&format!(
            "| {:width$} ",
            h,
            width = max_width_matrix.get(index).unwrap_or(&2)
        ));
    }

    // Absolute length of printed line, including beginning and end symbols
    let abs_width = header_string.chars().count() + 1;

    print_spacer(abs_width);
    println!("{}|", header_string);
    print_spacer(abs_width);

    abs_width
}

fn print_spacer(length: usize) {
    println!("+{}+", "-".repeat(length - 2 as usize));
}
