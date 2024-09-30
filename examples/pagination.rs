use std::io;
use tabprinter::{Alignment, Table, TableStyle};
use termcolor::{ColorChoice, StandardStream};

fn main() -> io::Result<()> {
    // Create a new table with Grid style
    let mut table = Table::new(TableStyle::Grid);

    // Add columns
    table.add_column("ID", Some(5), Alignment::Right);
    table.add_column("Name", Some(20), Alignment::Left);
    table.add_column("Age", Some(5), Alignment::Center);
    table.add_column("City", Some(15), Alignment::Left);

    // Add sample data
    for i in 1..=25 {
        table.add_row(vec![
            i.to_string(),
            format!("Person {}", i),
            (20 + i % 30).to_string(),
            format!("City {}", (i % 5) + 1),
        ]);
    }

    // Set page size
    table.set_page_size(10);

    // Create a StandardStream for colored output
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Print the table with pagination
    table.print_color_paginated(&mut stdout)?;

    Ok(())
}
