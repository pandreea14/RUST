use clap::{App, Arg};
use csv::ReaderBuilder;
use serde_derive::Deserialize;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Deserialize)]
struct Record {
    atomic_number: String,
    symbol: String,
    name: String,
}

impl Record {
    fn deserialize_headers() -> Vec<String> {
        let data = r#"{ "Atomic Number": "", "Symbol": "", "Name": "" }"#;
        let value: Value = serde_json::from_str(data).expect("Failed to parse JSON");

        let headers: Vec<String> = value
            .as_object()
            .expect("Expected JSON/CSV object")
            .keys()
            .map(|s| s.to_string())
            .collect();

        // let mut headers = Vec::new();
        // let mut lines = data.lines();

        // if let Some(header_line) = lines.next() {
        //     for field in header_line.split(',') {
        //         headers.push(field.trim().to_string());
        //     }
        // }

        headers
    }
}

struct TableCell {
    content: String,
}

impl TableCell {
    fn new(content: &str) -> TableCell {
        TableCell {
            content: content.to_string(),
        }
    }

    fn content_len(&self) -> usize {
        self.content.len()
    }

    fn format_left(&self, width: usize) -> String {
        format!(" {:<width$} |", self.content, width = width)
    }

    fn format_center(&self, width: usize) -> String {
        format!(" {:^width$} |", self.content, width = width)
    }

    fn format_right(&self, width: usize) -> String {
        format!(" {:>width$} |", self.content, width = width)
    }
}

struct AsciiTable<'a> {
    headers: Vec<TableCell>,
    rows: Vec<Vec<TableCell>>,
    sep: i32,
    alignment: &'a str,
}

impl<'a> AsciiTable<'a> {
    fn new(headers: Vec<String>, sep_line: i32, align: &'a str) -> AsciiTable {
        let header_cells: Vec<TableCell> =
            headers.into_iter().map(|h| TableCell::new(&h)).collect();
        AsciiTable {
            headers: header_cells,
            rows: Vec::new(),
            sep: sep_line,
            alignment: align,
        }
    }

    fn add_row(&mut self, row_data: Vec<&str>) {
        let row_cells: Vec<TableCell> = row_data.into_iter().map(TableCell::new).collect();
        self.rows.push(row_cells);
    }

    fn calculate_max_width(&self) -> Vec<usize> {
        let num_columns = self.headers.len();
        let mut max_widths = vec![0; num_columns];

        for row in self.rows.iter().chain(std::iter::once(&self.headers)) {
            for (i, cell) in row.iter().enumerate() {
                max_widths[i] = max_widths[i].max(cell.content_len());
            }
        }

        max_widths
    }

    fn print(&self) {
        // calculate maximum width for each column
        let max_widths = self.calculate_max_width();

        // print header
        self.print_horizontal_line(&max_widths);
        self.print_row(&self.headers, &max_widths);
        if self.sep == 0 {
            self.print_horizontal_line(&max_widths);
        }

        // print rows
        for row in &self.rows {
            if self.sep != 0 {
                self.print_horizontal_line(&max_widths);
            }
            self.print_row(row, &max_widths);
        }

        // print bottom border
        self.print_horizontal_line(&max_widths);
    }

    fn o_print<W: Write>(&self, out: &mut W) {
        // calculate maximum width for each column
        let max_widths = self.calculate_max_width();

        // print header
        let _ = self.o_print_horizontal_line(&max_widths, out);
        let _ = self.o_print_row(&self.headers, &max_widths, out);
        if self.sep == 0 {
            let _ = self.o_print_horizontal_line(&max_widths, out);
        }

        //print rows
        for row in &self.rows {
            if self.sep != 0 {
                let _ = self.o_print_horizontal_line(&max_widths, out);
            }
            let _ = self.o_print_row(row, &max_widths, out);
        }

        // print bottom border
        let _ = self.o_print_horizontal_line(&max_widths, out);
    }

    fn print_horizontal_line(&self, max_widths: &[usize]) {
        for &max_width in max_widths {
            print!("+{:-<width$}", "", width = max_width + 2);
        }
        println!("+");
    }
    fn o_print_horizontal_line<W: Write>(
        &self,
        max_widths: &[usize],
        out: &mut W,
    ) -> Result<(), Box<dyn Error>> {
        for &max_width in max_widths {
            write!(out, "+{:-<width$}", "", width = max_width + 2)?;
        }
        writeln!(out, "+")?;
        Ok(())
    }

    fn print_row(&self, cells: &[TableCell], max_widths: &[usize]) {
        print!("|");
        if self.alignment == "c" {
            for (cell, &max_width) in cells.iter().zip(max_widths) {
                print!("{}", cell.format_center(max_width));
            }
        } else if self.alignment == "r" {
            for (cell, &max_width) in cells.iter().zip(max_widths) {
                print!("{}", cell.format_right(max_width));
            }
        } else {
            for (cell, &max_width) in cells.iter().zip(max_widths) {
                print!("{}", cell.format_left(max_width));
            }
        }
        println!();
    }
    fn o_print_row<W: Write>(
        &self,
        cells: &[TableCell],
        max_widths: &[usize],
        out: &mut W,
    ) -> Result<(), Box<dyn Error>> {
        write!(out, "|")?;
        if self.alignment == "c" {
            for (cell, &max_width) in cells.iter().zip(max_widths) {
                write!(out, "{}", cell.format_center(max_width))?;
            }
        } else if self.alignment == "r" {
            for (cell, &max_width) in cells.iter().zip(max_widths) {
                write!(out, "{}", cell.format_right(max_width))?;
            }
        } else {
            for (cell, &max_width) in cells.iter().zip(max_widths) {
                write!(out, "{}", cell.format_left(max_width))?;
            }
        }
        writeln!(out)?;
        Ok(())
    }
}

fn read_json(input_file: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let data: Vec<Record> = serde_json::from_reader(File::open(input_file)?)?;
    Ok(data)
}

fn read_csv(input_file: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_reader(File::open(input_file)?);
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;

        let instance = Record {
            atomic_number: record.get(0).unwrap_or_default().to_string(),
            symbol: record.get(1).unwrap_or_default().to_string(),
            name: record.get(2).unwrap_or_default().to_string(),
        };

        data.push(instance);
    }

    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect(); // collects the command line arguments into a Vec<String>

    if args.len() < 2 {
        eprintln!("Usage: cargo run <file.json or file.csv>");
        std::process::exit(1);
    }
    let matches = App::new("ascii")
        .arg(
            Arg::with_name("input")
                .short("i")
                .long("input")
                .takes_value(true)
                .required(true)
                .help("Specify the input CSV/JSON file"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .default_value("stdout")
                .help("Specify the output option"),
        )
        .arg(
            Arg::with_name("separator_line")
                .short("s")
                .long("separator_line")
                .takes_value(true)
                .default_value("0")
                .help("Specify if there is separation or not"),
        )
        .arg(
            Arg::with_name("alignment")
                .short("a")
                .long("alignment")
                .takes_value(true)
                .default_value("l")
                .help("Specify the type of alignment"),
        )
        .get_matches();

    let input_file = matches.value_of("input").expect("Input file is required");
    let output_file = matches.value_of("output").unwrap_or("stdout");
    let sep_line = matches.value_of("separator_line").unwrap_or("0");
    let align = matches.value_of("alignment").unwrap_or("l");

    if align.to_lowercase() != "l" && align.to_lowercase() != "c" && align.to_lowercase() != "r" {
        eprintln!("Not a valid option for alignment, try l for left, c for center or r for right!");
        std::process::exit(1);
    }

    let data = if input_file.ends_with(".json") {
        read_json(input_file)?
    } else if input_file.ends_with(".csv") {
        read_csv(input_file)?
    } else {
        return Err("Unsupported file format. Try a CSV or JSON file".into());
    };

    let headers: Vec<String> = Record::deserialize_headers();
    let mut table = AsciiTable::new(headers, sep_line.parse().unwrap(), align);

    for record in &data {
        let row_data: Vec<&str> = vec![&record.atomic_number, &record.symbol, &record.name];
        table.add_row(row_data.to_vec());
    }

    if output_file.to_lowercase() == "stdout" {
        println!("Output will be shown in stdout!");
        table.print();
    } else {
        let mut out = File::create(output_file)?;
        println!(
            "Check the output file created or overwritten: {}",
            output_file
        );
        table.o_print(&mut out);
        out.flush()?;
    }
    Ok(())
}
