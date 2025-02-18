use tabprinter::{Alignment, Table, TableStyle, Cell};

fn main() {
    let styles = [
        TableStyle::Simple,
        TableStyle::Grid,
        TableStyle::FancyGrid,
        TableStyle::Clean,
        TableStyle::Round,
        TableStyle::Banner,
        TableStyle::Block,
        TableStyle::Amiga,
        TableStyle::Minimal,
        TableStyle::Compact,
        TableStyle::Markdown,
        TableStyle::Dotted,
        TableStyle::Heavy,
        TableStyle::Neon,
    ];
    for style in styles.iter() {
        println!("{:?} style:", style);
        let mut table = Table::new(*style);

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
        println!("\n");
    }
}
