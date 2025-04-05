# tabprinter

`tabprinter` is a Rust library for creating and printing formatted tables in the terminal. It supports various table styles and offers both color and non-color output options.

## Features

- **Versatile Table Styles**: 
  - 8 built-in styles: Simple, Grid, FancyGrid, Clean, Round, Banner, Block, and Amiga
  - Support for both ASCII and Unicode border characters
- **Flexible Content Handling**:
  - Customizable column widths (fixed, percentage-based, or auto)
  - Text alignment options (Left, Right, Center)
  - Multi-line cell content support
  - Automatic text wrapping
- **Advanced Formatting**:
  - Color output support via termcolor
  - Bold, italic, and underline text formatting
  - Custom cell background colors
- **Output Options**:
  - Direct terminal output
  - String conversion for further processing
  - File export capabilities
- **Developer-Friendly**:
  - Intuitive and easy-to-use API
  - Minimal dependencies
  - Comprehensive documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tabprinter = "0.2.1"
```

## Usage

Here's a basic example of how to use `tabprinter`:

```rust
use tabprinter::{Table, TableStyle, Alignment};

fn main() {
    let mut table = Table::new(TableStyle::Grid);
    
    // Define columns with headers and alignment
    table.add_column("Name", Alignment::Left);
    table.add_column("Age", Alignment::Right);
    table.add_column("City", Alignment::Center);
    
    // Add rows of data
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
    
    // Print the table to stdout
    table.print().unwrap();
}
```

This will output:

```bash
+------------+-------+-----------------+
| Name       | Age   | City            |
+------------+-------+-----------------+
| Alice      | 30    | New York        |
| Bob        | 25    | Los Angeles     |
+------------+-------+-----------------+
```


## Table Styles

`tabprinter` supports the following table styles:

- `Simple`: No borders
- `Grid`: ASCII borders
- `FancyGrid`: Unicode borders
- `Clean`: Minimal borders
- `Round`: Rounded corners
- `Banner`: Top and bottom banners
- `Block`: Block-style borders
- `Amiga`: Amiga-inspired style (color output only)

To change the style, simply use a different `TableStyle` when creating the table:

```rust
let mut table = Table::new(TableStyle::FancyGrid);
```

## Color Output

To use color output, use the `print_color` method instead of `print`:

```rust
use termcolor::{ColorChoice, StandardStream};
let mut stdout = StandardStream::stdout(ColorChoice::Always);
table.print_color(&mut stdout).unwrap();
```


## Examples

Check out the `examples` directory for more usage examples:

- `basic_usage.rs`: Demonstrates basic table creation and printing
- `different_styles.rs`: Shows all available table styles
- `custom_data.rs`: Example of using custom data structures with tables
- `csv_usage.rs`: Example of CSV usage
- `column_aggregatations.rs`: Example of Column Aggregrations
- `group_by_subtotals.rs`: Example of Group by Subtotals

To run an example:

```bash
cargo run --example basic_usage
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
