use std::io;
use tabprinter::{Alignment, CustomColor, Table, TableStyle};

fn print_table_with_style(style: TableStyle) -> io::Result<()> {
    let mut table = Table::new(style);

    match style {
        TableStyle::Simple | TableStyle::Grid | TableStyle::FancyGrid | TableStyle::Clean => {
            table.set_header_color(CustomColor::new(255, 0, 0));
            table.set_row_color(CustomColor::new(0, 255, 0));
            table.set_border_color(CustomColor::new(0, 0, 255));
        }
        TableStyle::Round | TableStyle::Banner | TableStyle::Block | TableStyle::Amiga => {
            table.set_header_color(CustomColor::new(255, 165, 0));
            table.set_row_color(CustomColor::new(138, 43, 226));
            table.set_border_color(CustomColor::new(0, 255, 255));
        }
        _ => {
            table.set_header_color(CustomColor::new(255, 255, 0));
            table.set_row_color(CustomColor::new(255, 192, 203));
            table.set_border_color(CustomColor::new(0, 128, 0));
        }
    }

    table.add_column("Name", None, Alignment::Left);
    table.add_column("Age", None, Alignment::Right);
    table.add_column("City", None, Alignment::Center);
    table.add_row(vec![
        "Alice".to_string(),
        "30".to_string(),
        "New York".to_string(),
    ]);
    table.add_row(vec![
        "Bob".to_string(),
        "25".to_string(),
        "London".to_string(),
    ]);
    table.add_row(vec![
        "Charlie".to_string(),
        "35".to_string(),
        "Paris".to_string(),
    ]);

    println!("\n{:?} style:", style);
    table.print()?;
    println!();

    Ok(())
}

fn main() -> io::Result<()> {
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

    for style in &styles {
        print_table_with_style(*style)?;
    }

    Ok(())
}
