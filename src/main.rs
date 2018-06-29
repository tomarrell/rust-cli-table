extern crate clap;

mod csv;
mod render;

use std::io::{self, BufRead};

use clap::App;
use csv::{parse, CsvType};
use render::{render_data, render_header};

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

    let parsed_data: CsvType = match parse(&buffer) {
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

fn max_column_width<'a>(table: CsvType) -> Vec<usize> {
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
