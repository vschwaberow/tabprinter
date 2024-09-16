# tabprinter

`tabprinter` is a Rust library for creating and printing formatted tables in the terminal. It supports various table styles and offers both color and non-color output options.

## Features

- Multiple table styles: Simple, Grid, FancyGrid, Clean, Round, Banner, Block, and Amiga
- Customizable column widths and alignments
- Color output support (using termcolor)
- Easy-to-use API for creating and populating tables

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tabprinter = "0.1.0"
```

## Usage

Here's a basic example of how to use `tabprinter`:

```rust
use tabprinter::{Table, TableStyle, Alignment};
fn main() {
let mut table = Table::new(TableStyle::Grid);
table.add_column("Name", 10, Alignment::Left);
table.add_column("Age", 5, Alignment::Right);
table.add_column("City", 15, Alignment::Center);
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
```

This will output:

```bash
+------------+-------+-----------------+
| Name | Age | City |
+------------+-------+-----------------+
| Alice | 30 | New York |
| Bob | 25 | Los Angeles |
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

To run an example:

```bash
cargo run --example basic_usage
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
