use csv::Reader;
use std::{error::Error, fs::File};
use tabprinter::{Alignment, Table, TableStyle, Cell};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/data.csv")?;
    let mut rdr = Reader::from_reader(file);

    let mut table = Table::new(TableStyle::Neon);

    let headers = rdr.headers()?;
    for header in headers.iter() {
        table.add_column(header, 20, Alignment::Left);
    }

    for result in rdr.records() {
        let record = result?;
        let row: Vec<Cell> = record.iter().map(|s| Cell::new(s)).collect();
        table.add_row(row);
    }

    table.print()?;

    Ok(())
}
