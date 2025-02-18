use tabprinter::{Alignment, Table, TableStyle, Cell};

struct Person {
    name: String,
    age: u32,
    city: String,
}

fn main() {
    // Create a list of people
    let people = vec![
        Person {
            name: "Alice".to_string(),
            age: 30,
            city: "New York".to_string(),
        },
        Person {
            name: "Bob".to_string(),
            age: 25,
            city: "Los Angeles".to_string(),
        },
        Person {
            name: "Charlie".to_string(),
            age: 35,
            city: "Chicago".to_string(),
        },
    ];

    // Create a new table with the FancyGrid style
    let mut table = Table::new(TableStyle::FancyGrid);

    // Add columns to the table
    table.add_column("Name", 10, Alignment::Left);
    table.add_column("Age", 5, Alignment::Right);
    table.add_column("City", 15, Alignment::Center);

    // Add rows to the table using the list of people
    for person in people {
        table.add_row(vec![
            Cell::new(&person.name),
            Cell::new(&person.age.to_string()),
            Cell::new(&person.city),
        ]);
    }

    // Print the table
    table.print().unwrap();
}
