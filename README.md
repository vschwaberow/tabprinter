# tabprinter

`tabprinter` is a Rust library for creating and printing formatted tables in the terminal. It supports various table styles and offers both color and non-color output options.

## Features

- **Versatile Table Styles**: 
  - 14 built-in styles: Simple, Grid, FancyGrid, Clean, Round, Banner, Block, Amiga, Minimal, Compact, Markdown, Dotted, Heavy, and Neon
  - Support for both ASCII and Unicode border characters
- **Flexible Content Handling**:
  - Customizable column widths (fixed, percentage-based, or auto)
  - Text alignment options (Left, Right, Center)
  - Multi-line cell content support
  - Automatic text wrapping
- **Advanced Formatting**:
  - Color output support via termcolor
  - Bold, italic, and underline text formatting
  - Custom cell foreground and background colors
  - Number formatting with decimal places and thousand separators
- **Output Options**:
  - Direct terminal output
  - String conversion for further processing
  - File export capabilities
- **Data Analysis Features**:
  - Column aggregations (sum, average, min, max)
  - Group by with subtotals
  - CSV file integration
  - Data filtering and sorting
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
use tabprinter::{Table, TableStyle, Alignment, Cell};

fn main() {
    let mut table = Table::new(TableStyle::Grid);
    
    // Define columns with headers and alignment
    table.add_column("Name", Alignment::Left);
    table.add_column("Age", Alignment::Right);
    table.add_column("City", Alignment::Center);
    
    // Add rows of data
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
- `Minimal`: Thin borders
- `Compact`: Compact thin borders
- `Markdown`: Markdown table syntax
- `Dotted`: Dotted borders
- `Heavy`: Thick borders
- `Neon`: Neon-style borders

To change the style, simply use a different `TableStyle` when creating the table:

```rust
let mut table = Table::new(TableStyle::FancyGrid);
```

## Color Output

To use color output, use the `print_color` method instead of `print`:

```rust
use termcolor::{ColorChoice, StandardStream};
use tabprinter::{Cell, Color};

// Create a cell with background color
let mut cell = Cell::new("Colored Cell");
cell.style.background_color = Some(Color::Red);
cell.style.foreground_color = Some(Color::White);
cell.style.bold = true;

// Print with color support
let mut stdout = StandardStream::stdout(ColorChoice::Always);
table.print_color(&mut stdout).unwrap();
```

## Advanced Features

### Column Aggregations

You can calculate statistics on numeric columns:

```rust
let sum = table.sum_column(1).unwrap_or(0.0);
let avg = table.average_column(1).unwrap_or(0.0);
let min = table.min_column(1).unwrap_or(0.0);
let max = table.max_column(1).unwrap_or(0.0);
```

### CSV Integration

Load tables directly from CSV files:

```rust
let table = Table::from_csv("data.csv")?;
```

### Number Formatting

Format numbers with decimal places and thousand separators:

```rust
let mut cell = Cell::new("1234.567");
cell.style.decimal_places = Some(2);
cell.style.thousand_separator = true;
// Displays as: 1,234.57
```

## Examples

Check out the `examples` directory for more usage examples:

- `basic_usage.rs`: Demonstrates basic table creation and printing
- `different_styles.rs`: Shows all available table styles
- `custom_data.rs`: Example of using custom data structures with tables
- `csv_usage.rs`: Example of CSV usage
- `column_aggregations.rs`: Example of Column Aggregations
- `group_by_subtotals.rs`: Example of Group by Subtotals

To run an example:

```bash
cargo run --example basic_usage
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
