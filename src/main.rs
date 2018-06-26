extern crate clap;

use std::iter;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("rust-cli-table")
        .version("0.1.0")
        .author("Tom A. <tom.arrell@gmail.com>")
        .about("Renders a table in the command line from a CSV")
        .get_matches();

    let test_data: &[&[&str]] = &[
        &["Name", "Email", "Address", "Phone"],
        &["Tom", "tom@tom.com", "2 Cool St", "02145718"],
    ];

    render_header(test_data.get(0).unwrap());
    render_data(test_data.get(1..).unwrap());
}

fn render_data(data: &[&[&str]]) -> usize {
    let mut data_string = String::from("");

    for row in data {
        for cell in row.iter() {
            data_string.push_str(&format!("| {} ", cell));
        }
    }

    println!("{}|", data_string);

    1
}

fn render_header(headers: &[&str]) -> usize {
    let mut header_string = String::from("");

    for h in headers.iter() {
        header_string.push_str(&format!("| {} ", h));
    }

    // Absolute length of printed line, including beginning and end symbols
    let header_length = header_string.len() + 1;

    print_spacer(header_length);
    println!("{}|", header_string);
    print_spacer(header_length);

    header_length
}

fn print_spacer(length: usize) {
    println!("|{}|", "-".repeat(length - 2 as usize));
}
