extern crate clap;

use std::error::Error;

use clap::App;

type Csv<'a> = Vec<Vec<&'a str>>;

fn main() {
    App::new("rust-cli-table")
        .version("0.1.0")
        .author("Tom A. <tom.arrell@gmail.com>")
        .about("Renders a table in the command line from a CSV")
        .get_matches();

    let stdin = "header1, header2\n1, 2\n";

    let test_data: Csv = parse_csv(stdin).unwrap();

    let max_column_width_matrix = max_column_width(test_data.clone());

    render_header(
        test_data.get(0).unwrap().to_vec(),
        max_column_width_matrix.clone(),
    );
    render_data(
        test_data.get(1..).unwrap().to_vec(),
        max_column_width_matrix.clone(),
    );
}

fn parse_csv<'a>(input: &'a str) -> Result<Vec<Vec<&str>>, Box<Error>> {
    let lines = input.trim().split("\n");

    let csv: Vec<Vec<&str>> = lines
        .map(|line| line.split(", ").collect::<Vec<&str>>())
        .collect();

    Ok(csv)
}

fn max_column_width<'a>(table: Csv) -> Vec<usize> {
    // Might panic
    let col_count = table.get(0).unwrap().len();

    let mut width_matrix = vec![0; col_count];

    for row in table {
        for (i, item) in row.iter().enumerate() {
            if width_matrix.get(i).unwrap() < &item.len() {
                width_matrix[i] = item.len();
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
        .unwrap()
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
