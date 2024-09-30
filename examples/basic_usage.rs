use tabprinter::{Alignment, Table, TableStyle};

fn main() {
    let mut table = Table::new(TableStyle::Grid);

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
}
