use tabprinter::{Alignment, Table, TableStyle, Cell};

fn main() {
    // Create a new table with the Grid style
    let mut table = Table::new(TableStyle::Grid);

    // Add columns to the table
    table.add_column("Name", 10, Alignment::Left);
    table.add_column("Age", 5, Alignment::Right);
    table.add_column("City", 15, Alignment::Center);

    // Add rows to the table
    table.add_row(vec![
        Cell::new("Alice"),
        Cell::new("30"),
        Cell::new("New York"),
    ]);
    table.add_row(vec![
        Cell::new("Bob"),
        Cell::new("25"),
        Cell::new("Los Angeles"),
    ]);

    // Print the table
    table.print().unwrap();
}
