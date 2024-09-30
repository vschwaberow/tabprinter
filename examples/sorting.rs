use tabprinter::{Alignment, Table, TableStyle};

fn main() -> std::io::Result<()> {
    // Create a new table with the Grid style
    let mut table = Table::new(TableStyle::Grid);

    // Add columns
    table.add_column("Title", None, Alignment::Left);
    table.add_column("Author", None, Alignment::Left);
    table.add_column("Year", Some(6), Alignment::Right);

    // Add rows
    table.add_row(vec![
        "1984".to_string(),
        "George Orwell".to_string(),
        "1949".to_string(),
    ]);
    table.add_row(vec![
        "To Kill a Mockingbird".to_string(),
        "Harper Lee".to_string(),
        "1960".to_string(),
    ]);
    table.add_row(vec![
        "The Great Gatsby".to_string(),
        "F. Scott Fitzgerald".to_string(),
        "1925".to_string(),
    ]);
    table.add_row(vec![
        "Pride and Prejudice".to_string(),
        "Jane Austen".to_string(),
        "1813".to_string(),
    ]);
    table.add_row(vec![
        "The Catcher in the Rye".to_string(),
        "J.D. Salinger".to_string(),
        "1951".to_string(),
    ]);

    println!("Original table:");
    table.print()?;

    // Sort the table by year (ascending order)
    table.sort_by_column(2, true);

    println!("\nTable sorted by year (ascending):");
    table.print()?;

    // Sort the table by year (descending order)
    table.sort_by_column(2, false);

    println!("\nTable sorted by year (descending):");
    table.print()?;

    // Filter the table for books published after 1950
    let mut filtered_table = table.filter(|row| row[2].parse::<i32>().unwrap_or(0) > 1950);

    println!("\nFiltered table (books published after 1950):");
    filtered_table.print()?;

    Ok(())
}
