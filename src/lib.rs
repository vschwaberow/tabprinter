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

impl Default for CellStyle { fn default() -> Self { Self::new() } }

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

    // Get the maximum ESTIMATED width (.chars().count()) of any line in the cell
    fn max_line_estimated_width(&self) -> usize {
        self.lines().iter().map(|line| line.chars().count()).max().unwrap_or(0)
    }

    // Get number of lines
    fn height(&self) -> usize {
        // Ensure empty cells still have height 1 for row calculation
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
    row_heights: Option<Vec<usize>>,
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
        }
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

    /// Calculates the sum of the specified column index.
    pub fn get_column_count(&self) -> usize {
        self.column_defs.len()
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

    /// Prints the table to the specified writer.
    pub fn print_to_writer(&mut self, writer: &mut dyn WriteColor) -> io::Result<()> {
        // Ensure dimensions are calculated before printing
        self.ensure_dimensions();

        // Handle Amiga style separately if it requires unique logic beyond styling characters
        if self.style == TableStyle::Amiga {
            return self.print_amiga_color(writer);
        }

        // Check if the style has a specific configuration
        let style_cfg = match self.style.config() {
            Some(cfg) => cfg,
            // Handle Simple/other styles that might return None from config() but still need printing
            None => {
                // Provide a default minimal config or handle Simple style explicitly
                 if self.style == TableStyle::Simple {
                    // Use a basic printing logic for Simple style
                    return self.print_simple_or_default(writer);
                 } else {
                     // Or return error / default behaviour for unconfigured styles
                     return Err(io::Error::new(io::ErrorKind::Other, "Unsupported table style"));
                 }
            }
        };

        // Print the table with the specified style
        self.print_styled(writer, style_cfg)
    }

    /// Fallback printing for Simple style or potentially other unstyled tables.
    fn print_simple_or_default(&self, writer: &mut dyn WriteColor) -> io::Result<()> {
        // Ensure dimensions are calculated before printing
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;
        let row_heights = self.row_heights.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;

        // Print Headers simply
        for (i, col_def) in self.column_defs.iter().enumerate() {
            let width = column_dims[i].effective_content_width;
            let header_text = &col_def.header;
            let estimated_width = header_text.chars().count();
            let padding_needed = width.saturating_sub(estimated_width);

            // Apply basic styling (bold, italic, underline) if needed
            match column_dims[i].alignment {
                Alignment::Left => write!(writer, "{}{}", header_text, " ".repeat(padding_needed))?,
                Alignment::Center => {
                    let left_pad = padding_needed / 2;
                    let right_pad = padding_needed - left_pad;
                    write!(writer, "{}{}{}", " ".repeat(left_pad), header_text, " ".repeat(right_pad))?
                }
                Alignment::Right => write!(writer, "{}{}", " ".repeat(padding_needed), header_text)?,
            }
                if i < self.column_defs.len() - 1 {
                // Add spacing between columns - simple spacing
                write!(writer, "  ")?;
                }
        }
        writeln!(writer)?;

        // Print Rows simply
        for (r, row) in self.rows.iter().enumerate() {
                let row_h = row_heights[r];
                for line_index in 0..row_h {
                for (i, cell) in row.iter().enumerate() {
                    let cell_lines = cell.lines();
                    let line_content = if let Some(line) = cell_lines.get(line_index) { *line } else { "" };
                    let width = column_dims[i].effective_content_width;
                    let estimated_width = line_content.chars().count();
                    let padding_needed = width.saturating_sub(estimated_width);

                    // Apply basic styling (bold, italic, underline) if needed
                    let mut spec = ColorSpec::new();
                    if cell.style.bold { spec.set_bold(true); }
                    if cell.style.italic { spec.set_italic(true); }
                    if cell.style.underline { spec.set_underline(true); }
                    writer.set_color(&spec)?;

                    match column_dims[i].alignment {
                        Alignment::Left => write!(writer, "{}{}", line_content, " ".repeat(padding_needed))?,
                        Alignment::Center => {
                            let left_pad = padding_needed / 2;
                            let right_pad = padding_needed - left_pad;
                            write!(writer, "{}{}{}", " ".repeat(left_pad), line_content, " ".repeat(right_pad))?
                        }
                        Alignment::Right => write!(writer, "{}{}", " ".repeat(padding_needed), line_content)?,
                    }
                    writer.reset()?;

                    if i < self.column_defs.len() - 1 {
                    write!(writer, "  ")?;
                    }
                }
                writeln!(writer)?;
            }
        }
        Ok(())
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

    /// Invalidates the cached dimensions of the table.
    /// This is used to force recalculation of column widths and row heights.
    /// It should be called whenever the content of the table changes.
    /// This is important for ensuring that the table is displayed correctly
    /// after adding or modifying rows or columns.
    fn invalidate_dimensions(&mut self) {
        self.column_dims = None;
        self.row_heights = None;
    }

    /// Ensures that the dimensions of the table are calculated.
    /// This function calculates the effective content width, maximum padding,
    /// and total width for drawing lines for each column.
    fn ensure_dimensions(&mut self) {
        if self.column_dims.is_some() && self.row_heights.is_some() {
            return;
        }

        let num_cols = self.column_defs.len();
        if num_cols == 0 {
            self.column_dims = Some(Vec::new());
            self.row_heights = Some(Vec::new());
            return;
        }

        let mut max_content_widths: Vec<usize> = vec![0; num_cols];
        let mut max_paddings: Vec<usize> = vec![1; num_cols];

        for (i, col_def) in self.column_defs.iter().enumerate() {
            max_content_widths[i] = max_content_widths[i].max(col_def.header.chars().count());
        }

        let mut row_heights_calc = Vec::with_capacity(self.rows.len());
        for row in &self.rows {
            let mut current_row_max_h = 1;
            for (i, cell) in row.iter().enumerate() {
                 max_content_widths[i] = max_content_widths[i].max(cell.max_line_estimated_width());
                 max_paddings[i] = max_paddings[i].max(cell.style.padding);
                 current_row_max_h = current_row_max_h.max(cell.height());
            }
            row_heights_calc.push(current_row_max_h);
        }

        let mut column_dims_calc = Vec::with_capacity(num_cols);
        for i in 0..num_cols {
            let eff_width = max_content_widths[i];
            let max_pad = max_paddings[i];
            let total_width = eff_width + 2 * max_pad + 2;
            column_dims_calc.push(ColumnDim {
                effective_content_width: eff_width,
                max_padding: max_pad,
                total_width_for_drawing: total_width,
                alignment: self.column_defs[i].alignment,
            });
        }

        self.column_dims = Some(column_dims_calc);
        self.row_heights = Some(row_heights_calc);
    }

    /// Prints headers of the table.
    fn print_headers(&self, writer: &mut dyn WriteColor) -> io::Result<()> {
        // Ensure dimensions are calculated
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;
        
        for (i, (col_def, dim)) in self.column_defs.iter().zip(column_dims.iter()).enumerate() {
            let width = dim.effective_content_width;
            match dim.alignment {
                Alignment::Left => write!(
                    writer,
                    "{:<width$}",
                    col_def.header,
                    width = width
                )?,
                Alignment::Center => write!(
                    writer,
                    "{:^width$}",
                    col_def.header,
                    width = width
                )?,
                Alignment::Right => write!(
                    writer,
                    "{:>width$}",
                    col_def.header,
                    width = width
                )?,
            }
            if i < self.column_defs.len() - 1 {
                write!(writer, " ")?;
            }
        }
        writeln!(writer)
    }

    /// Prints a row of the table.
    fn print_row(&self, writer: &mut dyn WriteColor, row: &[Cell]) -> io::Result<()> {
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;
        let max_lines = row.iter().map(|cell| cell.lines().len()).max().unwrap_or(1);
        
        for line_index in 0..max_lines {
            for (i, (cell, dim)) in row.iter().zip(column_dims.iter()).enumerate() {
                let lines = cell.lines();
                let _line_content = if let Some(line) = lines.get(line_index) { *line } else { "" };
                
                let mut spec = ColorSpec::new();
                if cell.style.bold { spec.set_bold(true); }
                if cell.style.italic { spec.set_italic(true); }
                if cell.style.underline { spec.set_underline(true); }
                writer.set_color(&spec)?;
                
                let padding = " ".repeat(cell.style.padding);
                let formatted_line = cell.formatted_content();
                let width = dim.effective_content_width;
                
                match dim.alignment {
                    Alignment::Left => write!(
                        writer,
                        "{}{:<width$}{}",
                        padding,
                        formatted_line,
                        padding,
                        width = width
                    )?,
                    Alignment::Center => write!(
                        writer,
                        "{}{:^width$}{}",
                        padding,
                        formatted_line,
                        padding,
                        width = width
                    )?,
                    Alignment::Right => write!(
                        writer,
                        "{}{:>width$}{}",
                        padding,
                        formatted_line,
                        padding,
                        width = width
                    )?,
                }
                writer.reset()?;
                if i < column_dims.len() - 1 {
                    write!(writer, " ")?;
                }
            }
            writeln!(writer)?;
        }
        Ok(())
    }

    /// Prints a border line using the pre-calculated total widths.
    fn print_line(&self, writer: &mut dyn WriteColor, style: &LineStyle) -> io::Result<()> {
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;
        write!(writer, "{}", style.begin)?;
        for (i, dim) in column_dims.iter().enumerate() {
            if i > 0 {
                write!(writer, "{}", style.sep)?;
            }
            write!(writer, "{}", style.hline.repeat(dim.total_width_for_drawing))?;
        }
        writeln!(writer, "{}", style.end)
    }

    /// Prints a data row (or header row) applying styles and handling multi-line cells.
    fn print_content_row(
        &self,
        writer: &mut dyn WriteColor,
        cells: &[Cell],
        row_height: usize,
        row_style: &LineStyle,
    ) -> io::Result<()> {
        let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;

        for line_index in 0..row_height {
            write!(writer, "{}", row_style.begin)?;
            for (i, (cell, dim)) in cells.iter().zip(column_dims.iter()).enumerate() {
                if i > 0 {
                    write!(writer, "{}", row_style.sep)?;
                }

                let cell_lines = cell.lines();
                // Get the content for the current line, or empty string if cell is shorter
                let line_content = if let Some(line) = cell_lines.get(line_index) { *line } else { "" };
                let line_estimated_width = line_content.chars().count();

                // Calculate padding needed for alignment within the effective content width
                let alignment_padding_needed = dim.effective_content_width.saturating_sub(line_estimated_width);
                let (left_align_pad, right_align_pad) = match dim.alignment {
                    Alignment::Left => (0, alignment_padding_needed),
                    Alignment::Center => {
                        let left = alignment_padding_needed / 2;
                        let right = alignment_padding_needed - left;
                        (left, right)
                    }
                    Alignment::Right => (alignment_padding_needed, 0),
                };

                // Get actual cell padding and calculate extra padding needed to reach max_padding
                let cell_padding = cell.style.padding;
                let extra_left_pad = dim.max_padding.saturating_sub(cell_padding);
                let extra_right_pad = dim.max_padding.saturating_sub(cell_padding);

                // Construct the full cell content with all padding/spacing
                let left_spacing = format!("{}{}", " ".repeat(extra_left_pad), " ".repeat(cell_padding));
                let right_spacing = format!("{}{}", " ".repeat(cell_padding), " ".repeat(extra_right_pad));
                let aligned_content_part = format!("{}{}{}", " ".repeat(left_align_pad), line_content, " ".repeat(right_align_pad));

                // Apply text styling (bold, italic, underline)
                let mut spec = ColorSpec::new();
                if cell.style.bold { spec.set_bold(true); }
                if cell.style.italic { spec.set_italic(true); }
                if cell.style.underline { spec.set_underline(true); }
                writer.set_color(&spec)?;

                // Write the cell segment: space + total_left_padding + aligned_content + total_right_padding + space
                write!(writer, " {}{}{} ", left_spacing, aligned_content_part, right_spacing)?;

                writer.reset()?; // Reset color/style
            }
            writeln!(writer, "{}", row_style.end)?;
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

    /// Prints the table to the specified writer with a resolved style configuration.
    fn print_styled(
        &self,
        writer: &mut dyn WriteColor,
        style_cfg: &TableStyleConfig,
    ) -> io::Result<()> {
         let row_heights = self.row_heights.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;

        self.print_line(writer, &style_cfg.top)?;

        let header_cells: Vec<Cell> = self.column_defs.iter()
            .map(|def| Cell::new(&def.header)) 
            .collect();
        if !header_cells.is_empty() {
            let header_height = header_cells.iter().map(|c| c.height()).max().unwrap_or(1);
            self.print_content_row(writer, &header_cells, header_height, &style_cfg.row)?;
            self.print_line(writer, &style_cfg.below_header)?;
        }

        for (r, row) in self.rows.iter().enumerate() {
            let row_height = row_heights[r];
            self.print_content_row(writer, row, row_height, &style_cfg.row)?;
        }

        self.print_line(writer, &style_cfg.bottom)
    }

    /// Prints the table to the standard output with Amiga-specific color logic.
    fn print_amiga_color(&self, writer: &mut dyn WriteColor) -> io::Result<()> {
        let mut header_spec = ColorSpec::new();
        header_spec.set_fg(Some(Color::Blue)).set_intense(true);

        let mut data_spec = ColorSpec::new();
        data_spec.set_fg(Some(Color::White));

         let column_dims = self.column_dims.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;
         let row_heights = self.row_heights.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Dimensions not calculated"))?;

        writer.set_color(&header_spec)?;
        for (i, col_def) in self.column_defs.iter().enumerate() {
            let width = column_dims[i].effective_content_width;
            let header_text = &col_def.header;
            let estimated_width = header_text.chars().count();
            let padding_needed = width.saturating_sub(estimated_width);

            match column_dims[i].alignment {
                Alignment::Left => write!(writer, "{}{}", header_text, " ".repeat(padding_needed))?,
                Alignment::Center => {/* ... center logic ... */},
                Alignment::Right => write!(writer, "{}{}", " ".repeat(padding_needed), header_text)?,
            }
             if i < self.column_defs.len() - 1 { write!(writer, "  ")?; }
        }
        writeln!(writer)?;
        writer.reset()?;

        writer.set_color(&data_spec)?;
        for (r, row) in self.rows.iter().enumerate() {
             let row_h = row_heights[r];
             for line_index in 0..row_h {
                for (i, cell) in row.iter().enumerate() {
                    let mut current_spec = data_spec.clone();
                    if cell.style.bold { current_spec.set_bold(true); }
                    if cell.style.italic { current_spec.set_italic(true); }
                    if cell.style.underline { current_spec.set_underline(true); }
                    writer.set_color(&current_spec)?;

                    let cell_lines = cell.lines();
                    let line_content = if let Some(line) = cell_lines.get(line_index) { *line } else { "" };
                    let width = column_dims[i].effective_content_width;
                    let estimated_width = line_content.chars().count();
                    let padding_needed = width.saturating_sub(estimated_width);

                    match column_dims[i].alignment {
                         Alignment::Left => write!(writer, "{}{}", line_content, " ".repeat(padding_needed))?,
                         Alignment::Center => {/* ... center logic ... */},
                         Alignment::Right => write!(writer, "{}{}", " ".repeat(padding_needed), line_content)?,
                    }
                     if i < self.column_defs.len() - 1 { write!(writer, "  ")?; }
                }
                writeln!(writer)?;
            }
        }
        writer.reset()?;
        Ok(())
    }

    /// Public print method using standard output.
    pub fn print(&mut self) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Auto);
        self.print_to_writer(&mut stdout)
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
        pub fn from_csv(path: &str) -> io::Result<Self> {
            let mut reader = csv::Reader::from_path(path)?;
            let headers = reader.headers()?.clone();
            let mut table = Table::new(TableStyle::Simple);

            for header in &headers {
                table.add_column(header, Alignment::Left);
            }

            for result in reader.records() {
                let record = result?;
                table.add_row(record.iter().map(|s| Cell::new(s)).collect());
            }

            Ok(table)
        }

        pub fn to_csv(&self, path: &str) -> io::Result<()> {
            let mut writer = csv::Writer::from_path(path)?;
            writer.write_record(self.column_defs.iter().map(|def| &def.header))?;
            for row in &self.rows {
                writer.write_record(row.iter().map(|cell| &cell.content))?;
            }
            Ok(writer.flush()?)
       }

    }
}
