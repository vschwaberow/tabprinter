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

    // Calculate and print column aggregations
    if let Some(sum) = table.sum_column(1) {
        println!("Sum: {}", sum);
    }
    if let Some(average) = table.average_column(1) {
        println!("Average: {}", average);
    }
    if let Some(min) = table.min_column(1) {
        println!("Min: {}", min);
    }
    if let Some(max) = table.max_column(1) {
        println!("Max: {}", max);
    }

    // Print the table
    table.print().unwrap();
}