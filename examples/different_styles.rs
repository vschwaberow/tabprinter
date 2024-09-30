use tabprinter::{Alignment, Table, TableStyle};

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

        table.add_column("Name", Some(10), Alignment::Left);
        table.add_column("Age", Some(5), Alignment::Right);
        table.add_column("City", Some(15), Alignment::Center);

        table.add_row(vec![
            "Alice".to_string(),
            "30".to_string(),
            "New York".to_string(),
        ]);
        table.add_row(vec![
            "Bob".to_string(),
            "25".to_string(),
            "Los Angeles".to_string(),
        ]);

        table.print().unwrap();
        println!("\n");
    }
}
