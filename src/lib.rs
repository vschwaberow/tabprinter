// SPDX-License-Identifier: MIT
// Project: tabprinter
// File: src/lib.rs
// Author: Volker Schwaberow <volker@schwaberow.de>

mod styles;

use std::io::{self, Write};
use styles::STYLES;
use termcolor::{ColorSpec, ColorChoice, StandardStream, WriteColor};
pub use termcolor::Color;

#[cfg(test)]
mod tests;

/// Represents different styles for table rendering.
/// Each variant corresponds to a specific table style.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableStyle {
    /// Simple table style with no borders.
    Simple,
    /// Table style with grid borders.
    Grid,
    /// Table style with fancy grid borders.
    FancyGrid,
    /// Clean table style with minimal borders.
    Clean,
    /// Table style with rounded corners.
    Round,
    /// Banner style table with top and bottom borders.
    Banner,
    /// Block style table with solid borders.
    Block,
    /// Amiga style table with color support.
    Amiga,
    /// Minimal table style with thin borders.
    Minimal,
    /// Compact table style with thin borders.
    Compact,
    /// Markdown style table with markdown syntax.
    Markdown,
    /// Dotted table style with dotted borders.
    Dotted,
    /// Heavy table style with thick borders.
    Heavy,
    /// Neon table style with neon-like borders.
    Neon,
}

impl TableStyle {
    /// Returns the configuration for the table style.
    /// If the style does not have a specific configuration, returns `None`.
    fn config(&self) -> Option<&'static TableStyleConfig> {
        match self {
            TableStyle::Grid => Some(&STYLES[1]),
            TableStyle::FancyGrid => Some(&STYLES[2]),
            TableStyle::Clean => Some(&STYLES[3]),
            TableStyle::Round => Some(&STYLES[4]),
            TableStyle::Banner => Some(&STYLES[5]),
            TableStyle::Block => Some(&STYLES[6]),
            TableStyle::Minimal => Some(&STYLES[8]),
            TableStyle::Compact => Some(&STYLES[9]),
            TableStyle::Markdown => Some(&STYLES[10]),
            TableStyle::Dotted => Some(&STYLES[11]),
            TableStyle::Heavy => Some(&STYLES[12]),
            TableStyle::Neon => Some(&STYLES[13]),
            _ => None,
        }
    }
}

/// Represents text alignment within a table cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    /// Left-aligned text.
    Left,
    /// Center-aligned text.
    Center,
    /// Right-aligned text.
    Right,
}

#[derive(Debug)]
struct LineStyle {
    begin: &'static str,
    hline: &'static str,
    sep: &'static str,
    end: &'static str,
}

#[derive(Debug)]
struct TableStyleConfig {
    top: LineStyle,
    below_header: LineStyle,
    bottom: LineStyle,
    row: LineStyle,
}

#[derive(Clone, Debug)]
struct ColumnDef {
    header: String,
    alignment: Alignment,
}

#[derive(Clone, Copy, Debug)]
struct ColumnDim {
    /// Max estimated width (.chars().count()) needed for content alignment
    effective_content_width: usize,
    /// Max padding specified for any cell in this column
    max_padding: usize,
    /// Total width for drawing lines: eff_width + 2*max_pad + 2 spaces
    total_width_for_drawing: usize,
    /// Alignment for the column
    alignment: Alignment,
}

/// Represents the style of a cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellStyle {
    /// Whether the text is bold.
    pub bold: bool,
    /// Whether the text is italic.
    pub italic: bool,
    /// Whether the text is underlined.
    pub underline: bool,
    /// The padding of the cell.
    pub padding: usize,
    /// The number of decimal places for number formatting.
    pub decimal_places: Option<usize>,
    /// Whether to use thousand separators for number formatting.
    pub thousand_separator: bool,
    /// The background color of the cell.
    pub background_color: Option<Color>,
    /// The foreground color of the cell.
    pub foreground_color: Option<Color>,
    /// The alignment of the cell.
    pub align: Option<Alignment>,
}

impl CellStyle {
    /// Creates a new `CellStyle` with default values.
    pub fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            padding: 1, // Default padding
            decimal_places: None,
            thousand_separator: false,
            background_color: None,
            foreground_color: None,
            align: None,
        }
    }
}

impl Default for CellStyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a cell in the table.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell {
    /// The text content of the cell.
    pub content: String,
    /// The style of the cell.
    pub style: CellStyle,
}

impl Cell {
    /// Creates a new `Cell` with the specified content and default style.
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            style: CellStyle::default(), // Use default here
        }
    }

    /// Splits the cell content into lines.
    fn lines(&self) -> Vec<&str> {
        self.content.lines().collect()
    }

    /// Formats the cell content based on the style.
    fn formatted_content(&self) -> String {
        // Early return if no numeric formatting is needed
        if self.style.decimal_places.is_none() && !self.style.thousand_separator {
            return self.content.clone();
        }

        if let Ok(num) = self.content.trim().parse::<f64>() {
            let mut formatted_num_str = if let Some(dp) = self.style.decimal_places {
                format!("{:.dp$}", num, dp = dp)
            } else {
                num.to_string() // Standard f64 to string conversion
            };

            if self.style.thousand_separator {
                let mut parts: Vec<&str> = formatted_num_str.splitn(2, '.').collect();
                let mut integer_part_original = parts[0].to_string();
                let decimal_part_str = if parts.len() > 1 { Some(parts[1]) } else { None };

                let sign = if integer_part_original.starts_with('-') {
                    integer_part_original.remove(0);
                    "-"
                } else { "" };

                if !integer_part_original.is_empty() {
                    let mut separated_digits = String::new();
                    let digits_len = integer_part_original.len();
                    let mut first_group_len = digits_len % 3;
                    if first_group_len == 0 && digits_len > 0 {
                        first_group_len = 3;
                    }

                    if digits_len > 0 { // Only add if there are digits
                        separated_digits.push_str(&integer_part_original[..first_group_len]);

                        for chunk_bytes in integer_part_original[first_group_len..].as_bytes().chunks(3) {
                            separated_digits.push(',');
                            separated_digits.push_str(std::str::from_utf8(chunk_bytes).unwrap_or(""));
                        }
                        integer_part_original = separated_digits;
                    }
                }

                let final_integer_part = format!("{}{}", sign, integer_part_original);

                formatted_num_str = if let Some(dp_str) = decimal_part_str {
                    format!("{}.{}", final_integer_part, dp_str)
                } else {
                    final_integer_part
                };
            }
            return formatted_num_str;
        }
        self.content.clone()
    }

    // Get the maximum ESTIMATED width (.chars().count()) of any line in the cell
    fn max_line_estimated_width(&self) -> usize {
        self.lines().iter().map(|line| line.chars().count()).max().unwrap_or(0)
    }

    // Get number of lines
    fn height(&self) -> usize {
        self.lines().len().max(1)
    }
}

#[derive(Debug)]
/// Represents a formatted table with rows, columns, and styling information.
///
/// This structure contains all the necessary data to render a table, including
/// column definitions, cell content, and styling options.
///
/// # Fields
///
/// * `column_defs` - Definitions for each column in the table
/// * `rows` - Content of the table as a collection of cell values
/// * `style` - Styling options for the entire table
/// * `column_dims` - Calculated dimensions for each column (width, etc.), computed when needed
/// * `row_heights` - Calculated height for each row, computed when needed
///
/// The `column_dims` and `row_heights` fields are populated during table layout calculations
/// and may be `None` until the table dimensions are computed.
pub struct Table {
    column_defs: Vec<ColumnDef>,
    rows: Vec<Vec<Cell>>,
    style: TableStyle,
    column_dims: Option<Vec<ColumnDim>>,
    row_heights: Option<Vec<usize>>, // Index 0 for header, 1+ for data rows
    // dimensions_calculated: bool, // Replaced by checking Option presence
}

impl Table {
    /// Creates a new table with the specified style.
    pub fn new(style: TableStyle) -> Self {
        Self {
            column_defs: Vec::new(),
            rows: Vec::new(),
            style,
            column_dims: None,
            row_heights: None,
            // dimensions_calculated: false,
        }
    }

    /// Creates a table from a CSV file (requires 'csv' feature).
    #[cfg(feature = "csv")]
    pub fn from_csv(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        use std::fs::File;
        use csv::Reader;

        let file = File::open(file_path)?;
        let mut reader = Reader::from_reader(file);
        let mut table = Table::new(TableStyle::Grid);
        
        // Get headers
        if let Ok(headers) = reader.headers() {
            for header in headers.iter() {
                table.add_column(header, Alignment::Left);
            }
        }

        // Read data rows
        for result in reader.records() {
            let record = result?;
            let cells: Vec<Cell> = record.iter().map(|field| Cell::new(field)).collect();
            if cells.len() == table.column_defs.len() {
                table.add_row(cells);
            }
        }

        Ok(table)
    }

    /// Adds a column to the table with the specified header and alignment.
    /// The column width is automatically adjusted based on the content.
    /// The alignment can be `Left`, `Center`, or `Right`.
    pub fn add_column(&mut self, header: &str, alignment: Alignment) {
        self.column_defs.push(ColumnDef {
            header: header.to_string(),
            alignment,
        });
        self.invalidate_dimensions();
    }

    /// Adds a row to the table.
    /// The length of the row must match the number of columns.
    pub fn add_row(&mut self, row: Vec<Cell>) {
        assert_eq!(
            self.column_defs.len(), row.len(),
            "Row length ({}) must match number of columns ({})", row.len(), self.column_defs.len()
        );
        self.rows.push(row);
        self.invalidate_dimensions();
    }

    /// Auto-adjusts the widths of the columns based on the content.
    // pub fn auto_adjust_widths(&mut self) {
    //     for (i, col) in self.columns.iter_mut().enumerate() {
    //         let header_len = col.header.len();
    //         let max_cell = self
    //             .rows
    //             .iter()
    //             .map(|row| row[i].content.len())
    //             .max()
    //             .unwrap_or(0);
    //         col.width = header_len.max(max_cell) + 2;
    //     }
    // }

    /// Sorts the rows by the specified column index.
    /// If `ascending` is true, sorts in ascending order; otherwise, sorts in descending order.
    pub fn sort_by_column(&mut self, column_index: usize, ascending: bool) {
        self.rows.sort_by(|a, b| {
            let ord = a[column_index].content.cmp(&b[column_index].content);
            if ascending {
                ord
            } else {
                ord.reverse()
            }
        });
    }

    /// Filters the rows using a predicate function.
    /// Returns a new table with the matching rows.
    pub fn filter_rows<F>(&self, predicate: F) -> Self
    where
        F: Fn(&Vec<Cell>) -> bool,
    {
        let filtered_rows = self.rows.iter().cloned().filter(predicate).collect();
        Table {
            column_defs: self.column_defs.clone(),
            rows: filtered_rows,
            style: self.style,
            column_dims: None,
            row_heights: None,
        }
    }

    /// Gets the number of columns in the table.
    pub fn get_column_count(&self) -> usize {
        self.column_defs.len()
    }

    /// Calculates the sum of the specified column index.
    pub fn sum_column(&self, column_index: usize) -> Option<f64> {
        if column_index >= self.column_defs.len() || self.rows.is_empty() {
            return None;
        }

        let mut sum = 0.0;
        let mut has_valid_numbers = false;

        for row in &self.rows {
            if let Ok(num) = row[column_index].content.trim().parse::<f64>() {
                sum += num;
                has_valid_numbers = true;
            }
        }

        if has_valid_numbers { Some(sum) } else { None }
    }

    /// Calculates the average of the specified column index.
    pub fn average_column(&self, column_index: usize) -> Option<f64> {
        if let Some(sum) = self.sum_column(column_index) {
            let count = self.rows.iter()
                .filter(|row| row[column_index].content.trim().parse::<f64>().is_ok())
                .count();
            if count > 0 {
                Some(sum / count as f64)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Finds the minimum value in the specified column index.
    pub fn min_column(&self, column_index: usize) -> Option<f64> {
        if column_index >= self.column_defs.len() || self.rows.is_empty() {
            return None;
        }

        self.rows.iter()
            .filter_map(|row| row[column_index].content.trim().parse::<f64>().ok())
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Finds the maximum value in the specified column index.
    pub fn max_column(&self, column_index: usize) -> Option<f64> {
        if column_index >= self.column_defs.len() || self.rows.is_empty() {
            return None;
        }

        self.rows.iter()
            .filter_map(|row| row[column_index].content.trim().parse::<f64>().ok())
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Groups rows by the specified column index and adds subtotals.
    pub fn group_by_column_with_subtotals(&mut self, column_index: usize) {
        let mut grouped_rows: Vec<Vec<Cell>> = Vec::new();
        let mut current_group: Vec<Vec<Cell>> = Vec::new();
        let mut current_value: Option<String> = None;

        for row in &self.rows {
            let value = &row[column_index].content;
            if current_value.is_none() || current_value.as_ref().unwrap() != value {
                if !current_group.is_empty() {
                    let subtotal_row = self.calculate_subtotal(&current_group);
                    grouped_rows.push(subtotal_row);
                }
                current_value = Some(value.clone());
                grouped_rows.push(row.clone());
                current_group = Vec::new();
            } else {
                grouped_rows.push(row.clone());
            }
            current_group.push(row.clone());
        }

        if !current_group.is_empty() {
            let subtotal_row = self.calculate_subtotal(&current_group);
            grouped_rows.push(subtotal_row);
        }

        self.rows = grouped_rows;
    }

    /// Calculates the subtotal for a group of rows.
    fn calculate_subtotal(&self, group: &[Vec<Cell>]) -> Vec<Cell> {
        let mut subtotal_row: Vec<Cell> = Vec::new();
        for (i, _column) in self.column_defs.iter().enumerate() {
            if i == 0 {
                subtotal_row.push(Cell::new("Subtotal"));
            } else if group
                .iter()
                .all(|row| row[i].content.parse::<f64>().is_ok())
            {
                let subtotal: f64 = group
                    .iter()
                    .map(|row| row[i].content.parse::<f64>().unwrap())
                    .sum();
                subtotal_row.push(Cell::new(&subtotal.to_string()));
            } else {
                subtotal_row.push(Cell::new(""));
            }
        }
        subtotal_row
    }

    /// Prints the table to stdout.
    pub fn print(&mut self) -> std::io::Result<()> {
        let mut stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
        self.print_to_writer(&mut stdout)
    }

    /// Prints the table to the specified writer.
    pub fn print_to_writer<W: WriteColor>(&mut self, writer: &mut W) -> std::io::Result<()> {
        self.ensure_dimensions();
        match self.style {
            TableStyle::Amiga => self.print_amiga_color(writer),
            _ => {
                if let Some(style_cfg_ref) = self.style.config() {
                    self.print_styled(writer, style_cfg_ref)
                } else {
                    self.print_simple(writer)
                }
            }
        }
    }

    /// Prints the table with color support.
    pub fn print_color<W: WriteColor>(&mut self, writer: &mut W) -> std::io::Result<()> {
        self.ensure_dimensions();
        match self.style {
            TableStyle::Amiga => self.print_amiga_color(writer),
            _ => {
                if let Some(style_cfg_ref) = self.style.config() {
                    self.print_styled(writer, style_cfg_ref)
                } else {
                    self.print_simple_color(writer)
                }
            }
        }
    }

    /// Invalidates the cached dimensions of the table.
    fn invalidate_dimensions(&mut self) {
        self.column_dims = None;
        self.row_heights = None;
        // self.dimensions_calculated = false;
    }

    /// Ensures that the dimensions of the table are calculated.
    fn ensure_dimensions(&mut self) {
        if self.column_dims.is_some() && self.row_heights.is_some() {
            return;
        }

        let num_cols = self.column_defs.len();
        let mut max_effective_widths: Vec<usize> = vec![0; num_cols];
        let mut calculated_row_heights: Vec<usize> = Vec::new(); // Index 0 for header, 1+ for data

        // Headers
        if num_cols > 0 {
            let mut header_row_max_lines = 0; // Default to 0 if no headers actually exist
            let header_cells: Vec<Cell> = self.column_defs.iter()
                .map(|def| Cell::new(&def.header)) // Assuming default style for header cell for width calc
                .collect();

            if !header_cells.is_empty() { // Only proceed if there are headers
                header_row_max_lines = 1; // Min 1 line if headers are present
                for (i, header_cell) in header_cells.iter().enumerate() {
                    let formatted_header = header_cell.formatted_content();
                    max_effective_widths[i] = std::cmp::max(
                        max_effective_widths[i],
                        formatted_header.lines().map(|l| l.chars().count()).max().unwrap_or(0),
                    );
                    header_row_max_lines = std::cmp::max(header_row_max_lines, formatted_header.lines().count().max(1));
                }
            }
            calculated_row_heights.push(header_row_max_lines);
        } else {
            calculated_row_heights.push(0); // No header row defined
        }

        // Data rows
        for row in &self.rows {
            let mut current_row_max_lines = 1; // Min 1 line for any data row
            for (i, cell) in row.iter().enumerate() {
                if i < num_cols {
                    let formatted_cell_content = cell.formatted_content();
                    max_effective_widths[i] = std::cmp::max(
                        max_effective_widths[i],
                        formatted_cell_content.lines().map(|l| l.chars().count()).max().unwrap_or(0),
                    );
                    current_row_max_lines = std::cmp::max(current_row_max_lines, formatted_cell_content.lines().count().max(1));
                }
            }
            calculated_row_heights.push(current_row_max_lines);
        }

        let final_column_dims: Vec<ColumnDim> = self.column_defs.iter().enumerate().map(|(i, col_def)| {
            let default_cell_padding = CellStyle::default().padding; // Use default padding from CellStyle
            let content_width = max_effective_widths[i];
            let total_width = content_width + default_cell_padding * 2; // Add padding on both sides
            ColumnDim {
                effective_content_width: total_width, // This is the total width including padding
                max_padding: default_cell_padding,
                total_width_for_drawing: total_width,
                alignment: col_def.alignment,
            }
        }).collect();

        self.column_dims = Some(final_column_dims);
        self.row_heights = Some(calculated_row_heights);
        // self.dimensions_calculated = true;
    }

    /// Prints the table to the specified writer with simple style and color support.
    fn print_simple_color(&mut self, writer: &mut dyn WriteColor) -> io::Result<()> {
        self.ensure_dimensions();
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated (print_simple_color:column_dims)"))?;
        let row_heights = self.row_heights.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated (print_simple_color:row_heights)"))?;

        // Print header
        if !self.column_defs.is_empty() {
            let header_cells: Vec<Cell> = self.column_defs.iter().map(|def| Cell::new(&def.header)).collect();
            let header_actual_height = row_heights.get(0).copied().unwrap_or(0);
            if header_actual_height > 0 {
                for line_idx in 0..header_actual_height {
                    self.print_row_simple_color_line(writer, &header_cells, line_idx, column_dims)?;
                }
                self.print_separator_simple_line(writer, column_dims)?;
            }
        }

        // Print data rows
        for (row_data_idx, row_cells) in self.rows.iter().enumerate() {
            let data_row_actual_height = row_heights.get(row_data_idx + 1).copied().unwrap_or(1);
            for line_idx in 0..data_row_actual_height {
                self.print_row_simple_color_line(writer, row_cells, line_idx, column_dims)?;
            }
        }
        
        if !column_dims.is_empty() {
            self.print_separator_simple_line(writer, column_dims)?;
        }
        Ok(())
    }

    /// Prints the table to the specified writer with simple style.
    fn print_simple(&mut self, writer: &mut dyn WriteColor) -> io::Result<()> {
        self.ensure_dimensions();
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated (print_simple:column_dims)"))?;
        let row_heights = self.row_heights.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated (print_simple:row_heights)"))?;

        // Print header
        if !self.column_defs.is_empty() {
            let header_cells: Vec<Cell> = self.column_defs.iter().map(|def| Cell::new(&def.header)).collect();
            let header_actual_height = row_heights.get(0).copied().unwrap_or(0); // Height for header row
            if header_actual_height > 0 { // Only print header if it has content/height
                for line_idx in 0..header_actual_height {
                    self.print_row_simple_line(writer, &header_cells, line_idx, column_dims)?;
                }
                self.print_separator_simple_line(writer, column_dims)?;
            }
        }

        // Print data rows
        for (row_data_idx, row_cells) in self.rows.iter().enumerate() {
            // row_heights[0] is header, data rows start from row_heights[1]
            let data_row_actual_height = row_heights.get(row_data_idx + 1).copied().unwrap_or(1);
            for line_idx in 0..data_row_actual_height {
                self.print_row_simple_line(writer, row_cells, line_idx, column_dims)?;
            }
        }
        // Print bottom border only if there are columns
        if !column_dims.is_empty() {
            self.print_separator_simple_line(writer, column_dims)?;
        }
        Ok(())
    }

    fn print_separator_simple_line(&self, writer: &mut dyn WriteColor, column_dims: &[ColumnDim]) -> io::Result<()> {
        if column_dims.is_empty() { return Ok(()); }
        write!(writer, "+")?;
        for dim in column_dims.iter() {
            // The width used in print_row_simple_line for content area is dim.effective_content_width
            // The dashes should span this width.
            write!(writer, "{}", "-".repeat(dim.effective_content_width))?;
            write!(writer, "+")?;
        }
        writeln!(writer)?;
        Ok(())
    }

    /// Helper for printing a single line of a row for simple styles with color support.
    fn print_row_simple_color_line(
        &self,
        writer: &mut dyn WriteColor,
        row_data: &[Cell],
        line_idx: usize,
        column_dims: &[ColumnDim],
    ) -> io::Result<()> {
        if column_dims.is_empty() { return writeln!(writer); }
        write!(writer, "|")?;
        for (col_idx, cell) in row_data.iter().enumerate() {
            if col_idx >= column_dims.len() { continue; }

            let dim = &column_dims[col_idx];
            let width = dim.effective_content_width;
            let cell_style = &cell.style;
            
            let formatted_content = cell.formatted_content();
            let content_line = formatted_content.lines().nth(line_idx).unwrap_or("");

            // Apply colors
            let mut spec = ColorSpec::new();
            if let Some(cell_bg) = cell_style.background_color { spec.set_bg(Some(cell_bg)); }
            if let Some(cell_fg) = cell_style.foreground_color { spec.set_fg(Some(cell_fg)); }
            if cell_style.bold { spec.set_bold(true); }
            if cell_style.italic { spec.set_italic(true); }
            if cell_style.underline { spec.set_underline(true); }

            writer.set_color(&spec)?;

            let content_len = content_line.chars().count();
            let (padding_left, padding_right) = match cell_style.align.unwrap_or(Alignment::Left) {
                Alignment::Left => (1, width.saturating_sub(content_len).saturating_sub(1)),
                Alignment::Right => (width.saturating_sub(content_len).saturating_sub(1), 1),
                Alignment::Center => {
                    let total_padding = width.saturating_sub(content_len);
                    let pl = total_padding / 2;
                    let pr = total_padding.saturating_sub(pl);
                    (pl.max(1), pr.max(1))
                }
            };
            
            let final_padding_left = if content_len + padding_left + padding_right > width {
                1.min(padding_left)
            } else {
                padding_left
            };
            let final_padding_right = if content_len + final_padding_left + padding_right > width {
                (width.saturating_sub(content_len).saturating_sub(final_padding_left)).max(1)
            } else {
                padding_right
            };

            write!(writer, "{}", " ".repeat(final_padding_left))?;
            let available_width_for_content = width.saturating_sub(final_padding_left).saturating_sub(final_padding_right);
            let display_content: String = content_line.chars().take(available_width_for_content).collect();
            write!(writer, "{}", display_content)?;
            write!(writer, "{}", " ".repeat(width.saturating_sub(display_content.chars().count()).saturating_sub(final_padding_left)))?;

            writer.reset()?;
            write!(writer, "|")?;
        }
        writeln!(writer)?;
        Ok(())
    }

    /// Helper for printing a single line of a row for simple/default styles.
    fn print_row_simple_line(
        &self,
        writer: &mut dyn WriteColor,
        row_data: &[Cell],
        line_idx: usize,
        column_dims: &[ColumnDim],
    ) -> io::Result<()> {
        if column_dims.is_empty() { return writeln!(writer); } // Avoid panic if no columns
        write!(writer, "|")?;
        for (col_idx, cell) in row_data.iter().enumerate() {
            // Ensure col_idx is within bounds of column_dims
            if col_idx >= column_dims.len() { continue; }

            let dim = &column_dims[col_idx];
            let width = dim.effective_content_width;
            let cell_style = &cell.style; // Direct access, not an Option
            
            let formatted_content = cell.formatted_content();
            let content_line = formatted_content.lines().nth(line_idx).unwrap_or("");

            // Padding logic from original: 1 space on each side minimum
            let content_len = content_line.chars().count();
            let (padding_left, padding_right) = match cell_style.align.unwrap_or(Alignment::Left) {
                Alignment::Left => (1, width.saturating_sub(content_len).saturating_sub(1)),
                Alignment::Right => (width.saturating_sub(content_len).saturating_sub(1), 1),
                Alignment::Center => {
                    let total_padding = width.saturating_sub(content_len);
                    let mut pl = total_padding / 2;
                    let mut pr = total_padding.saturating_sub(pl);
                    if pl == 0 && width > 0 { pl = 1; pr = pr.saturating_sub(1); }
                    else if pr == 0 && width > 0 { pr = 1; pl = pl.saturating_sub(1); }
                    if width == 0 { (0,0) } else { (pl.max(1), pr.max(1)) } // Ensure at least 1 padding if width > 0
                }
            };
            
            // Adjust padding if content is too large for width
            let final_padding_left = if content_len + padding_left + padding_right > width {
                1.min(padding_left)
            } else {
                padding_left
            };
            let final_padding_right = if content_len + final_padding_left + padding_right > width {
                (width.saturating_sub(content_len).saturating_sub(final_padding_left)).max(1)
            } else {
                padding_right
            };


            write!(writer, "{}", " ".repeat(final_padding_left))?;
            // Truncate content_line if it's too long with padding
            let available_width_for_content = width.saturating_sub(final_padding_left).saturating_sub(final_padding_right);
            let display_content: String = content_line.chars().take(available_width_for_content).collect();
            write!(writer, "{}", display_content)?;
            write!(writer, "{}", " ".repeat(width.saturating_sub(display_content.chars().count()).saturating_sub(final_padding_left)))?;


            write!(writer, "|")?;
        }
        writeln!(writer)?;
        Ok(())
    }

    /// Prints the table to the standard output with Amiga-specific color logic.
    fn print_amiga_color(&mut self, writer: &mut dyn WriteColor) -> io::Result<()> {
        self.ensure_dimensions();
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated (print_amiga_color:column_dims)"))?;
        let row_heights = self.row_heights.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated (print_amiga_color:row_heights)"))?;

        // Print header
        if !self.column_defs.is_empty() {
            let header_cells: Vec<Cell> = self.column_defs.iter().map(|def| Cell::new(&def.header)).collect();
            let header_actual_height = row_heights.get(0).copied().unwrap_or(0);

            if header_actual_height > 0 {
                let mut header_spec = ColorSpec::new();
                header_spec.set_fg(Some(Color::Yellow)); // Example: Amiga headers often yellow

                for line_idx in 0..header_actual_height {
                    for (col_idx, cell) in header_cells.iter().enumerate() {
                        if col_idx >= column_dims.len() { continue; }
                        let dim = &column_dims[col_idx];
                        let formatted_header_line = cell.formatted_content().lines().nth(line_idx).unwrap_or("").to_string();
                        
                        writer.set_color(&header_spec)?;
                        self.print_formatted_cell_line(writer, &formatted_header_line, dim, &cell.style, true)?; // is_amiga = true
                        writer.reset()?;
                        if col_idx < header_cells.len() - 1 {
                            write!(writer, " ")?; // Space between columns
                        }
                    }
                    writeln!(writer)?;
                }
            }
        }

        // Print data rows
        for (row_data_idx, row_cells) in self.rows.iter().enumerate() {
            let data_row_actual_height = row_heights.get(row_data_idx + 1).copied().unwrap_or(1);
            for line_idx in 0..data_row_actual_height {
                for (col_idx, cell) in row_cells.iter().enumerate() {
                    if col_idx >= column_dims.len() { continue; }
                    let dim = &column_dims[col_idx];
                    let cell_style = &cell.style;
                    
                    let formatted_cell_line = cell.formatted_content().lines().nth(line_idx).unwrap_or("").to_string();

                    let mut cell_color_spec = ColorSpec::new();
                    // Apply Amiga-like colors based on cell style
                    if let Some(fg) = cell_style.foreground_color { cell_color_spec.set_fg(Some(fg)); }
                    else { cell_color_spec.set_fg(Some(Color::White)); } // Default data fg
                    if let Some(bg) = cell_style.background_color { cell_color_spec.set_bg(Some(bg)); }
                    if cell_style.bold { cell_color_spec.set_bold(true); }
                    if cell_style.italic { cell_color_spec.set_italic(true); }
                    if cell_style.underline { cell_color_spec.set_underline(true); }
                    
                    writer.set_color(&cell_color_spec)?;
                    self.print_formatted_cell_line(writer, &formatted_cell_line, dim, cell_style, true)?; // is_amiga = true
                    writer.reset()?;

                    if col_idx < row_cells.len() - 1 {
                        write!(writer, " ")?;
                    }
                }
                writeln!(writer)?;
            }
        }
        Ok(())
    }

    // Helper for print_amiga_color and potentially other color modes
    fn print_formatted_cell_line(
        &self,
        writer: &mut dyn WriteColor,
        content_line: &str,
        dim: &ColumnDim,
        cell_style: &CellStyle, // Pass full CellStyle for alignment etc.
        is_amiga: bool, // Flag for Amiga specific color choices if needed beyond CellStyle.color
    ) -> io::Result<()> {
        let effective_width = dim.effective_content_width;
        let line_char_count = content_line.chars().count();

        // Amiga style often had specific foreground colors, background was often shared or simple.
        // This is a generic color application based on cell_style.
        // If is_amiga is true, one might apply specific Amiga palette mapping here if CellStyle.color was an index.
        // Since CellStyle.color is termcolor::Color, we use it directly.

        let (pad_left, pad_right) = match cell_style.align.unwrap_or(dim.alignment) { // Use cell's align, fallback to column's
            Alignment::Left => (0, effective_width.saturating_sub(line_char_count)),
            Alignment::Right => (effective_width.saturating_sub(line_char_count), 0),
            Alignment::Center => {
                let padding = effective_width.saturating_sub(line_char_count);
                (padding / 2, padding.saturating_sub(padding / 2))
            }
        };
        write!(writer, "{}", " ".repeat(pad_left))?;
        write!(writer, "{}", content_line)?; // Already a single line
        write!(writer, "{}", " ".repeat(pad_right))?;
        Ok(())
    }


    /// Prints the table to the specified writer with a resolved style configuration.
    fn print_styled(
        &mut self,
        writer: &mut dyn WriteColor,
        style_cfg: &TableStyleConfig,
    ) -> io::Result<()> {
        self.ensure_dimensions();
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated (print_styled:column_dims)"))?;
        let row_heights = self.row_heights.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated (print_styled:row_heights)"))?;

        if column_dims.is_empty() { return Ok(()); }

        self.print_horizontal_border(writer, &style_cfg.top, column_dims)?;

        // Print header
        if !self.column_defs.is_empty() {
            let header_cells: Vec<Cell> = self.column_defs.iter().map(|def| Cell::new(&def.header)).collect();
            let header_actual_height = row_heights.get(0).copied().unwrap_or(0);

            if header_actual_height > 0 {
                for line_idx in 0..header_actual_height {
                    self.print_content_row_styled(writer, &header_cells, &style_cfg.row, line_idx, column_dims)?;
                }
                self.print_horizontal_border(writer, &style_cfg.below_header, column_dims)?;
            }
        }

        // Print data rows
        for (row_data_idx, row_cells) in self.rows.iter().enumerate() {
            let data_row_actual_height = row_heights.get(row_data_idx + 1).copied().unwrap_or(1);
            for line_idx in 0..data_row_actual_height {
                self.print_content_row_styled(writer, row_cells, &style_cfg.row, line_idx, column_dims)?;
            }
        }
        
        self.print_horizontal_border(writer, &style_cfg.bottom, column_dims)?;
        Ok(())
    }

    /// Prints a border line using the pre-calculated total widths.
    fn print_horizontal_border(&self, writer: &mut dyn WriteColor, line_style: &LineStyle, column_dims: &[ColumnDim]) -> io::Result<()> {
        if column_dims.is_empty() { return Ok(()); }
        write!(writer, "{}", line_style.begin)?;
        for (i, dim) in column_dims.iter().enumerate() {
            if i > 0 {
                write!(writer, "{}", line_style.sep)?;
            }
            // total_width_for_drawing includes padding. hline should span this.
            write!(writer, "{}", line_style.hline.repeat(dim.total_width_for_drawing))?;
        }
        writeln!(writer, "{}", line_style.end)
    }

    /// Prints a data row (or header row) applying styles and handling multi-line cells for "styled" tables.
    fn print_content_row_styled(
        &self,
        writer: &mut dyn WriteColor,
        row_data: &[Cell],
        line_style: &LineStyle,
        line_idx: usize,
        column_dims: &[ColumnDim],
    ) -> io::Result<()> {
        if column_dims.is_empty() { return writeln!(writer); }

        write!(writer, "{}", line_style.begin)?;

        for (col_idx, cell) in row_data.iter().enumerate() {
            if col_idx >= column_dims.len() { continue; }
            let dim = &column_dims[col_idx];
            let width = dim.effective_content_width;
            let cell_style = &cell.style;

            let formatted_content = cell.formatted_content();
            let content_line = formatted_content.lines().nth(line_idx).unwrap_or("");

            let mut spec = ColorSpec::new();
            if let Some(cell_bg) = cell_style.background_color { spec.set_bg(Some(cell_bg)); }
            if let Some(cell_fg) = cell_style.foreground_color { spec.set_fg(Some(cell_fg)); }
            if cell_style.bold { spec.set_bold(true); }
            if cell_style.italic { spec.set_italic(true); }
            if cell_style.underline { spec.set_underline(true); }

            writer.set_color(&spec)?;

            let content_len = content_line.chars().count();
            let alignment = cell_style.align.unwrap_or(dim.alignment);
            let (padding_left, padding_right) = match alignment {
                Alignment::Left => (cell_style.padding, width.saturating_sub(content_len).saturating_sub(cell_style.padding)),
                Alignment::Right => (width.saturating_sub(content_len).saturating_sub(cell_style.padding), cell_style.padding),
                Alignment::Center => {
                    let total_padding = width.saturating_sub(content_len);
                    let pl = total_padding / 2;
                    let pr = total_padding.saturating_sub(pl);
                    (pl.max(cell_style.padding), pr.max(cell_style.padding))
                }
            };
            
            let final_padding_left = padding_left.min(width.saturating_sub(content_len));
            let final_padding_right = width.saturating_sub(content_len).saturating_sub(final_padding_left);

            write!(writer, "{}", " ".repeat(final_padding_left))?;
            write!(writer, "{}", content_line)?;
            write!(writer, "{}", " ".repeat(final_padding_right))?;

            writer.reset()?;

            if col_idx < row_data.len() - 1 {
                write!(writer, "{}", line_style.sep)?;
            }
        }
        writeln!(writer, "{}", line_style.end)?;
        Ok(())
    }


    // The old print_content_row, print_row_color_line, print_row_amiga_color_line might be obsolete
    // or need to be adapted if specific non-styled color printing is still required differently.
    // For now, print_amiga_color and print_styled handle their line printing.
    // The print_content_row that was in the read_file output (from the failed diff) is replaced by print_content_row_styled.
}