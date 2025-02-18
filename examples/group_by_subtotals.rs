use tabprinter::{Alignment, Table, TableStyle, Cell};

fn main() {
    // Create a new table with the Simple style
    let mut table = Table::new(TableStyle::Simple);

    // Add columns to the table
    table.add_column("Category", 10, Alignment::Left);
    table.add_column("Amount", 10, Alignment::Right);

    // Add rows to the table
    table.add_row(vec![Cell::new("A"), Cell::new("100")]);
    table.add_row(vec![Cell::new("A"), Cell::new("200")]);
    table.add_row(vec![Cell::new("B"), Cell::new("300")]);
    table.add_row(vec![Cell::new("B"), Cell::new("400")]);

    // Group rows by the Category column and add subtotals
    table.group_by_column_with_subtotals(0);

    // Print the table
    table.print().unwrap();
}
