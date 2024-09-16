use tabprinter::{Alignment, Table, TableStyle};

struct Person {
    name: String,
    age: u32,
    city: String,
}

fn main() {
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

    let mut table = Table::new(TableStyle::FancyGrid);

    table.add_column("Name", 10, Alignment::Left);
    table.add_column("Age", 5, Alignment::Right);
    table.add_column("City", 15, Alignment::Center);

    for person in people {
        table.add_row(vec![person.name, person.age.to_string(), person.city]);
    }

    table.print().unwrap();
}
