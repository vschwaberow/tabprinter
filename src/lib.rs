// SPDX-License-Identifier: MIT
// Project: tabprinter
// File: src/lib.rs
// Author: Volker Schwaberow <volker@schwaberow.de>
// Copyright (c) 2024 Volker Schwaberow

mod styles;

use std::io::{self, Write};
use styles::STYLES;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
#[derive(Clone, Copy, Debug)]
pub enum Alignment {
    /// Left-aligned text.
    Left,
    /// Center-aligned text.
    Center,
    /// Right-aligned text.
    Right,
}

struct LineStyle {
    begin: &'static str,
    hline: &'static str,
    sep: &'static str,
    end: &'static str,
}

struct TableStyleConfig {
    top: LineStyle,
    below_header: LineStyle,
    bottom: LineStyle,
    row: LineStyle,
}

/// Represents a column in the table.
#[derive(Clone)]
pub struct Column {
    /// The header text of the column.
    header: String,
    /// The width of the column.
    width: usize,
    /// The alignment of the text within the column.
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
}

impl CellStyle {
    /// Creates a new `CellStyle` with default values.
    pub fn new() -> Self {
        Self {
            bold: false,
            italic: false,
            underline: false,
            padding: 1,
            decimal_places: None,
            thousand_separator: false,
        }
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
            style: CellStyle::new(),
        }
    }

    /// Splits the cell content into lines.
    fn lines(&self) -> Vec<&str> {
        self.content.lines().collect()
    }

    /// Formats the cell content based on the style.
    fn formatted_content(&self) -> String {
        if let Ok(number) = self.content.parse::<f64>() {
            let mut formatted = if let Some(decimal_places) = self.style.decimal_places {
                format!("{:.1$}", number, decimal_places)
            } else {
                number.to_string()
            };
            if self.style.thousand_separator {
                let parts: Vec<&str> = formatted.split('.').collect();
                let mut integer_part = parts[0].to_string();
                let mut chars: Vec<char> = integer_part.chars().collect();
                let mut i = chars.len() as isize - 3;
                while i > 0 {
                    chars.insert(i as usize, ',');
                    i -= 3;
                }
                integer_part = chars.into_iter().collect();
                formatted = if parts.len() > 1 {
                    format!("{}.{}", integer_part, parts[1])
                } else {
                    integer_part
                };
            }
            formatted
        } else {
            self.content.clone()
        }
    }
}

/// Represents a table with columns and rows.
pub struct Table {
    /// The columns of the table.
    columns: Vec<Column>,
    /// The rows of the table.
    rows: Vec<Vec<Cell>>,
    /// The style of the table.
    style: TableStyle,
}

impl Table {
    /// Creates a new table with the specified style.
    pub fn new(style: TableStyle) -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            style,
        }
    }

    /// Adds a column to the table.
    pub fn add_column(&mut self, header: &str, width: usize, alignment: Alignment) {
        self.columns.push(Column {
            header: header.to_string(),
            width,
            alignment,
        });
    }

    /// Adds a row to the table.
    /// The length of the row must match the number of columns.
    pub fn add_row(&mut self, row: Vec<Cell>) {
        assert_eq!(
            self.columns.len(),
            row.len(),
            "Row length must match number of columns"
        );
        self.rows.push(row);
    }

    /// Auto-adjusts the widths of the columns based on the content.
    pub fn auto_adjust_widths(&mut self) {
        for (i, col) in self.columns.iter_mut().enumerate() {
            let header_len = col.header.len();
            let max_cell = self
                .rows
                .iter()
                .map(|row| row[i].content.len())
                .max()
                .unwrap_or(0);
            col.width = header_len.max(max_cell) + 2;
        }
    }

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
        let filtered = self.rows.iter().cloned().filter(predicate).collect();
        Self {
            columns: self.columns.clone(),
            rows: filtered,
            style: self.style,
        }
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
        for (i, _column) in self.columns.iter().enumerate() {
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

    /// Prints the table to the specified writer.
    pub fn print_to_writer(&self, writer: &mut dyn WriteColor) -> io::Result<()> {
        if let Some(style_cfg) = self.style.config() {
            self.print_styled(writer, style_cfg)
        } else {
            self.print_simple(writer)
        }
    }

    /// Prints the table with color support.
    pub fn print_color<W: Write + WriteColor>(&self, writer: &mut W) -> io::Result<()> {
        match self.style {
            TableStyle::Amiga => self.print_amiga_color(writer),
            _ => {
                if let Some(style_cfg) = self.style.config() {
                    self.print_styled(writer, style_cfg)
                } else {
                    self.print_simple(writer)
                }
            }
        }
    }

    /// Prints headers of the table.
    fn print_headers(&self, writer: &mut dyn WriteColor) -> io::Result<()> {
        for (i, column) in self.columns.iter().enumerate() {
            match column.alignment {
                Alignment::Left => write!(
                    writer,
                    "{:<width$}",
                    column.header,
                    width = column.width - 1
                )?,
                Alignment::Center => write!(
                    writer,
                    "{:^width$}",
                    column.header,
                    width = column.width - 1
                )?,
                Alignment::Right => write!(
                    writer,
                    "{:>width$}",
                    column.header,
                    width = column.width - 1
                )?,
            }
            if i < self.columns.len() - 1 {
                write!(writer, " ")?;
            }
        }
        writeln!(writer)
    }

    /// Prints a row of the table.
    fn print_row(&self, writer: &mut dyn WriteColor, row: &[Cell]) -> io::Result<()> {
        let max_lines = row.iter().map(|cell| cell.lines().len()).max().unwrap_or(1);
        for line_index in 0..max_lines {
            for (column, cell) in self.columns.iter().zip(row.iter()) {
                let lines = cell.lines();
                let _line = lines.get(line_index).unwrap_or(&"");
                let mut spec = ColorSpec::new();
                if cell.style.bold {
                    spec.set_bold(true);
                }
                if cell.style.italic {
                    spec.set_italic(true);
                }
                if cell.style.underline {
                    spec.set_underline(true);
                }
                writer.set_color(&spec)?;
                let padding = " ".repeat(cell.style.padding);
                let formatted_line = cell.formatted_content();
                match column.alignment {
                    Alignment::Left => write!(
                        writer,
                        "{}{:width$}{}",
                        padding,
                        formatted_line,
                        padding,
                        width = column.width - 1
                    )?,
                    Alignment::Center => write!(
                        writer,
                        "{}{:^width$}{}",
                        padding,
                        formatted_line,
                        padding,
                        width = column.width - 1
                    )?,
                    Alignment::Right => write!(
                        writer,
                        "{}{:>width$}{}",
                        padding,
                        formatted_line,
                        padding,
                        width = column.width - 1
                    )?,
                }
                writer.reset()?;
                write!(writer, " ")?;
            }
            writeln!(writer)?;
        }
        Ok(())
    }

    /// Prints a line of the table.
    fn print_line(&self, writer: &mut dyn WriteColor, style: &LineStyle) -> io::Result<()> {
        write!(writer, "{}", style.begin)?;
        for (i, column) in self.columns.iter().enumerate() {
            if i > 0 {
                write!(writer, "{}", style.sep)?;
            }
            write!(writer, "{}", style.hline.repeat(column.width + 2))?;
        }
        writeln!(writer, "{}", style.end)
    }

    /// Prints a row of the table with a specific style.
    fn print_row_styled(
        &self,
        writer: &mut dyn WriteColor,
        row: &[Cell],
        style: &LineStyle,
    ) -> io::Result<()> {
        let max_lines = row.iter().map(|cell| cell.lines().len()).max().unwrap_or(1);
        for line_index in 0..max_lines {
            write!(writer, "{}", style.begin)?;
            for (i, (cell, column)) in row.iter().zip(self.columns.iter()).enumerate() {
                if i > 0 {
                    write!(writer, "{}", style.sep)?;
                }
                let lines = cell.lines();
                let _line = lines.get(line_index).unwrap_or(&"");
                let mut spec = ColorSpec::new();
                if cell.style.bold {
                    spec.set_bold(true);
                }
                if cell.style.italic {
                    spec.set_italic(true);
                }
                if cell.style.underline {
                    spec.set_underline(true);
                }
                writer.set_color(&spec)?;
                let padding = " ".repeat(cell.style.padding);
                let formatted_line = cell.formatted_content();
                match column.alignment {
                    Alignment::Left => write!(
                        writer,
                        " {}{:width$}{} ",
                        padding,
                        formatted_line,
                        padding,
                        width = column.width
                    )?,
                    Alignment::Center => write!(
                        writer,
                        " {}{:^width$}{} ",
                        padding,
                        formatted_line,
                        padding,
                        width = column.width
                    )?,
                    Alignment::Right => write!(
                        writer,
                        " {}{:>width$}{} ",
                        padding,
                        formatted_line,
                        padding,
                        width = column.width
                    )?,
                }
                writer.reset()?;
            }
            writeln!(writer, "{}", style.end)?;
        }
        Ok(())
    }

    /// Prints the table to the specified writer with simple style.
    fn print_simple(&self, writer: &mut dyn WriteColor) -> io::Result<()> {
        self.print_headers(writer)?;
        for row in &self.rows {
            self.print_row(writer, row)?;
        }
        Ok(())
    }

    /// Prints the table to the specified writer with styled style.
    fn print_styled(
        &self,
        writer: &mut dyn WriteColor,
        style: &TableStyleConfig,
    ) -> io::Result<()> {
        self.print_line(writer, &style.top)?;
        self.print_row_styled(
            writer,
            &self
                .columns
                .iter()
                .map(|c| Cell::new(&c.header))
                .collect::<Vec<_>>(),
            &style.row,
        )?;
        self.print_line(writer, &style.below_header)?;
        for row in &self.rows {
            self.print_row_styled(writer, row, &style.row)?;
        }
        self.print_line(writer, &style.bottom)
    }

    /// Prints the table to the standard output with simple style.
    fn print_amiga_color<W: Write + WriteColor>(&self, writer: &mut W) -> io::Result<()> {
        let mut spec = ColorSpec::new();
        spec.set_fg(Some(Color::Blue));
        writer.set_color(&spec)?;
        self.print_headers(writer)?;
        spec.set_fg(Some(Color::White));
        writer.set_color(&spec)?;
        for row in &self.rows {
            self.print_row(writer, row)?;
        }
        writer.reset()?;
        Ok(())
    }

    /// Prints the table to the standard output with color support.
    pub fn print(&self) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        self.print_color(&mut stdout)
    }

    /// Aggregates the specified column using the provided aggregation function.
    pub fn aggregate_column<F>(&self, column_index: usize, aggregation_fn: F) -> Option<f64>
    where
        F: Fn(Vec<f64>) -> f64,
    {
        let values: Vec<f64> = self
            .rows
            .iter()
            .filter_map(|row| row[column_index].content.parse::<f64>().ok())
            .collect();
        if values.is_empty() {
            None
        } else {
            Some(aggregation_fn(values))
        }
    }

    /// Calculates the sum of the specified column.
    pub fn sum_column(&self, column_index: usize) -> Option<f64> {
        self.aggregate_column(column_index, |values| values.iter().sum())
    }

    /// Calculates the average of the specified column.
    pub fn average_column(&self, column_index: usize) -> Option<f64> {
        self.aggregate_column(column_index, |values| {
            values.iter().sum::<f64>() / values.len() as f64
        })
    }

    /// Finds the minimum value in the specified column.
    pub fn min_column(&self, column_index: usize) -> Option<f64> {
        self.aggregate_column(column_index, |values| {
            *values
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        })
    }

    /// Finds the maximum value in the specified column.
    pub fn max_column(&self, column_index: usize) -> Option<f64> {
        self.aggregate_column(column_index, |values| {
            *values
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        })
    }
}

#[cfg(feature = "csv")]
mod csv_support {
    use super::*;
    pub use csv;

    impl Table {
        /// Creates a table from a CSV file.
        /// The first row of the CSV file is used as the header.
        pub fn from_csv(path: &str) -> io::Result<Self> {
            let mut reader = csv::Reader::from_path(path)?;
            let headers = reader.headers()?;
            let mut table = Table::new(TableStyle::Simple);
            for header in headers {
                table.add_column(header, 10, Alignment::Left);
            }
            for result in reader.records() {
                let record = result?;
                table.add_row(record.iter().map(|s| Cell::new(s)).collect());
            }
            Ok(table)
        }

        /// Writes the table to a CSV file.
        pub fn to_csv(&self, path: &str) -> io::Result<()> {
            let mut writer = csv::Writer::from_path(path)?;
            for row in &self.rows {
                writer.write_record(row.iter().map(|cell| &cell.content))?;
            }
            Ok(writer.flush()?)
        }
    }
}
