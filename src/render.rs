use csv::CsvType;

pub fn render_data(data: CsvType, max_width_matrix: Vec<usize>) -> usize {
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

    render_spacer(abs_width);

    abs_width
}

pub fn render_header(headers: Vec<&str>, max_width_matrix: Vec<usize>) -> usize {
    let mut header_string = String::from("");

    for (index, h) in headers.iter().enumerate() {
        header_string.push_str(&format!(
            "| {:width$} ",
            h,
            width = max_width_matrix.get(index).unwrap_or(&2)
        ));
    }

    // Absolute length of printed line, including beginning and end symbols
    header_string.push_str("|");
    let abs_width = header_string.chars().count();

    render_spacer(abs_width);
    println!("{}", header_string);
    render_spacer(abs_width);

    abs_width
}

fn render_spacer(length: usize) {
    println!("+{}+", "-".repeat(length - 2 as usize));
}
