// SPDX-License-Identifier: MIT
// Project: tabprinter
// File: src/lib.rs
// Author: Volker Schwaberow <volker@schwaberow.de>
// Copyright (c) 2024 Volker Schwaberow

use std::cmp;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug)]
pub enum TableStyle {
    Simple,
    Grid,
    FancyGrid,
    Clean,
    Round,
    Banner,
    Block,
    Amiga,
    Minimal,
    Compact,
    Markdown,
    Dotted,
    Heavy,
    Neon,
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Left,
    Center,
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

macro_rules! define_styles {
    ($($name:ident: {
        $($field:ident: {
            $($inner_field:ident: $value:expr),+
        $(,)?}),+
    $(,)?}),+) => {
        const STYLES: [TableStyleConfig; 14] = [
            $(
                TableStyleConfig {
                    $($field: LineStyle {
                        $($inner_field: $value,)+
                    },)+
                },
            )+
        ];
    };
}

define_styles! {
    Simple: {
        top: { begin: "", hline: "", sep: "", end: "" },
        below_header: { begin: "", hline: "", sep: "", end: "" },
        bottom: { begin: "", hline: "", sep: "", end: "" },
        row: { begin: "", hline: "", sep: "", end: "" }
    },
    Grid: {
        top: { begin: "+", hline: "-", sep: "+", end: "+" },
        below_header: { begin: "+", hline: "-", sep: "+", end: "+" },
        bottom: { begin: "+", hline: "-", sep: "+", end: "+" },
        row: { begin: "|", hline: "", sep: "|", end: "|" }
    },
    FancyGrid: {
        top: { begin: "╒", hline: "═", sep: "╤", end: "╕" },
        below_header: { begin: "╞", hline: "═", sep: "╪", end: "╡" },
        bottom: { begin: "╘", hline: "═", sep: "╧", end: "╛" },
        row: { begin: "│", hline: "", sep: "│", end: "│" }
    },
    Clean: {
        top: { begin: "", hline: "─", sep: " ", end: "" },
        below_header: { begin: "", hline: "─", sep: " ", end: "" },
        bottom: { begin: "", hline: "─", sep: " ", end: "" },
        row: { begin: "", hline: "", sep: " ", end: "" }
    },
    Round: {
        top: { begin: "╭", hline: "─", sep: "┬", end: "╮" },
        below_header: { begin: "├", hline: "─", sep: "┼", end: "┤" },
        bottom: { begin: "╰", hline: "─", sep: "┴", end: "╯" },
        row: { begin: "│", hline: "", sep: "│", end: "│" }
    },
    Banner: {
        top: { begin: "╒", hline: "═", sep: "╤", end: "╕" },
        below_header: { begin: "╘", hline: "═", sep: "╧", end: "╛" },
        bottom: { begin: "╘", hline: "═", sep: "╧", end: "╛" },
        row: { begin: "│", hline: "", sep: "│", end: "│" }
    },
    Block: {
        top: { begin: "◢", hline: "■", sep: "■", end: "◣" },
        below_header: { begin: " ", hline: "━", sep: "━", end: " " },
        bottom: { begin: "◥", hline: "■", sep: "■", end: "◤" },
        row: { begin: "", hline: "", sep: " ", end: "" }
    },
    Amiga: {
        top: { begin: "", hline: "", sep: "", end: "" },
        below_header: { begin: "", hline: "", sep: "", end: "" },
        bottom: { begin: "", hline: "", sep: "", end: "" },
        row: { begin: "", hline: "", sep: "", end: "" }
    },
     Minimal: {
        top: { begin: "┌", hline: "─", sep: "┬", end: "┐" },
        below_header: { begin: "├", hline: "─", sep: "┼", end: "┤" },
        bottom: { begin: "└", hline: "─", sep: "┴", end: "┘" },
        row: { begin: "│", hline: "", sep: "│", end: "│" }
    },
    Compact: {
        top: { begin: "┌", hline: "─", sep: "┬", end: "┐" },
        below_header: { begin: "├", hline: "─", sep: "┼", end: "┤" },
        bottom: { begin: "└", hline: "─", sep: "┴", end: "┘" },
        row: { begin: "│", hline: "", sep: "│", end: "│" }
    },
    Markdown: {
        top: { begin: "", hline: "", sep: "", end: "" },
        below_header: { begin: "|", hline: "-", sep: "|", end: "|" },
        bottom: { begin: "", hline: "", sep: "", end: "" },
        row: { begin: "|", hline: "", sep: "|", end: "|" }
    },
    Dotted: {
        top: { begin: ".", hline: ".", sep: ".", end: "." },
        below_header: { begin: ":", hline: ".", sep: ":", end: ":" },
        bottom: { begin: "'", hline: ".", sep: "'", end: "'" },
        row: { begin: ":", hline: "", sep: ":", end: ":" }
    },
    Heavy: {
        top: { begin: "┏", hline: "━", sep: "┳", end: "┓" },
        below_header: { begin: "┣", hline: "━", sep: "╋", end: "┫" },
        bottom: { begin: "┗", hline: "━", sep: "┻", end: "┛" },
        row: { begin: "┃", hline: "", sep: "┃", end: "┃" }
    },
    Neon: {
        top: { begin: "┏", hline: "━", sep: "┳", end: "┓" },
        below_header: { begin: "┣", hline: "━", sep: "╋", end: "┫" },
        bottom: { begin: "┗", hline: "━", sep: "┻", end: "┛" },
        row: { begin: "┃", hline: "", sep: "┃", end: "┃" }
    }
}

#[derive(Clone, Copy)]
pub struct CustomColor {
    r: u8,
    g: u8,
    b: u8,
}

impl CustomColor {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        CustomColor { r, g, b }
    }

    fn to_color(&self) -> Color {
        Color::Rgb(self.r, self.g, self.b)
    }
}

#[derive(Clone)]
pub struct Column {
    header: String,
    width: Option<usize>,
    alignment: Alignment,
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
    style: TableStyle,
    auto_width: bool,
    page_size: Option<usize>,
    header_color: Option<CustomColor>,
    row_color: Option<CustomColor>,
    border_color: Option<CustomColor>,
}

impl Table {
    pub fn new(style: TableStyle) -> Self {
        Table {
            columns: Vec::new(),
            rows: Vec::new(),
            style,
            auto_width: true,
            page_size: None,
            header_color: None,
            row_color: None,
            border_color: None,
        }
    }

    pub fn from_csv(path: &str) -> io::Result<Self> {
        let mut reader = csv::Reader::from_path(path)?;
        let headers = reader.headers()?;
        let mut table = Table::new(TableStyle::Simple);
        for header in headers {
            table.add_column(header, Some(10), Alignment::Left);
        }
        for result in reader.records() {
            let record = result?;
            table.add_row(record.iter().map(|s| s.to_string()).collect());
        }
        Ok(table)
    }

    pub fn to_csv(&self, path: &str) -> io::Result<()> {
        let mut writer = csv::Writer::from_path(path)?;
        for row in &self.rows {
            writer.write_record(row)?;
        }
        Ok(())
    }

    pub fn print_to_writer(&self, writer: &mut dyn Write) -> io::Result<()> {
        match self.style {
            TableStyle::Simple => self.print_simple(writer),
            TableStyle::Grid => self.print_styled(writer, &STYLES[1]),
            TableStyle::FancyGrid => self.print_styled(writer, &STYLES[2]),
            TableStyle::Clean => self.print_styled(writer, &STYLES[3]),
            TableStyle::Round => self.print_styled(writer, &STYLES[4]),
            TableStyle::Banner => self.print_styled(writer, &STYLES[5]),
            TableStyle::Block => self.print_styled(writer, &STYLES[6]),
            TableStyle::Amiga => self.print_simple(writer),
            TableStyle::Minimal => self.print_simple(writer),
            TableStyle::Compact => self.print_simple(writer),
            TableStyle::Markdown => self.print_simple(writer),
            TableStyle::Dotted => self.print_simple(writer),
            TableStyle::Heavy => self.print_simple(writer),
            TableStyle::Neon => self.print_simple(writer),
        }
    }

    pub fn add_column(&mut self, header: &str, width: Option<usize>, alignment: Alignment) {
        self.columns.push(Column {
            header: header.to_string(),
            width,
            alignment,
        });
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        assert_eq!(
            self.columns.len(),
            row.len(),
            "Row length must match columns"
        );
        self.rows.push(row);
    }

    pub fn print(&mut self) -> io::Result<()> {
        self.calculate_column_widths();
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        self.print_color(&mut stdout)
    }

    pub fn print_color<W: Write + WriteColor>(&mut self, writer: &mut W) -> io::Result<()> {
        self.calculate_column_widths();
        let mut color_spec = ColorSpec::new();

        if let Some(border_color) = self.border_color {
            color_spec.set_fg(Some(border_color.to_color()));
            writer.set_color(&color_spec)?;
        }

        match self.style {
            TableStyle::Simple => self.print_simple_color(writer, &mut color_spec),
            TableStyle::Grid => self.print_styled_color(writer, &STYLES[1], &mut color_spec),
            TableStyle::FancyGrid => self.print_styled_color(writer, &STYLES[2], &mut color_spec),
            TableStyle::Clean => self.print_styled_color(writer, &STYLES[3], &mut color_spec),
            TableStyle::Round => self.print_styled_color(writer, &STYLES[4], &mut color_spec),
            TableStyle::Banner => self.print_styled_color(writer, &STYLES[5], &mut color_spec),
            TableStyle::Block => self.print_styled_color(writer, &STYLES[6], &mut color_spec),
            TableStyle::Amiga => self.print_amiga_color(writer),
            TableStyle::Minimal => self.print_styled_color(writer, &STYLES[7], &mut color_spec),
            TableStyle::Compact => self.print_styled_color(writer, &STYLES[8], &mut color_spec),
            TableStyle::Markdown => self.print_styled_color(writer, &STYLES[9], &mut color_spec),
            TableStyle::Dotted => self.print_styled_color(writer, &STYLES[10], &mut color_spec),
            TableStyle::Heavy => self.print_styled_color(writer, &STYLES[11], &mut color_spec),
            TableStyle::Neon => self.print_styled_color(writer, &STYLES[12], &mut color_spec),
        }
    }

    fn print_amiga_color<W: Write + WriteColor>(&self, stream: &mut W) -> io::Result<()> {
        let mut color_spec = ColorSpec::new();
        color_spec.set_fg(Some(termcolor::Color::Blue));

        stream.set_color(&color_spec)?;
        self.print_headers(stream)?;

        color_spec.set_fg(Some(termcolor::Color::White));
        stream.set_color(&color_spec)?;

        self.rows
            .iter()
            .try_for_each(|row| self.print_row(stream, row))
    }

    fn print_headers(&self, writer: &mut dyn Write) -> io::Result<()> {
        for (i, column) in self.columns.iter().enumerate() {
            let width = column.width.unwrap_or(0);
            match column.alignment {
                Alignment::Left => write!(writer, "{:<width$}", column.header, width = width - 1)?,
                Alignment::Center => {
                    write!(writer, "{:^width$}", column.header, width = width - 1)?
                }
                Alignment::Right => write!(writer, "{:>width$}", column.header, width = width - 1)?,
            }
            if i < self.columns.len() - 1 {
                write!(writer, " ")?;
            }
        }
        writeln!(writer)
    }

    fn print_row(&self, writer: &mut dyn Write, row: &[String]) -> io::Result<()> {
        for (column, cell) in self.columns.iter().zip(row.iter()) {
            let width = column.width.unwrap_or(0);
            match column.alignment {
                Alignment::Left => write!(writer, "{:<width$}", cell, width = width - 1)?,
                Alignment::Center => write!(writer, "{:^width$}", cell, width = width - 1)?,
                Alignment::Right => write!(writer, "{:>width$}", cell, width = width - 1)?,
            }
            write!(writer, " ")?;
        }
        writeln!(writer)
    }

    fn print_line(&self, writer: &mut dyn Write, style: &LineStyle) -> io::Result<()> {
        write!(writer, "{}", style.begin)?;
        for (i, column) in self.columns.iter().enumerate() {
            if i > 0 {
                write!(writer, "{}", style.sep)?;
            }
            write!(
                writer,
                "{}",
                style.hline.repeat(column.width.unwrap_or(0) + 2)
            )?;
        }
        writeln!(writer, "{}", style.end)
    }

    fn print_row_styled(
        &self,
        writer: &mut dyn Write,
        row: &[impl AsRef<str>],
        style: &LineStyle,
    ) -> io::Result<()> {
        write!(writer, "{}", style.begin)?;
        for (i, (cell, column)) in row.iter().zip(self.columns.iter()).enumerate() {
            if i > 0 {
                write!(writer, "{}", style.sep)?;
            }
            let width = column.width.unwrap_or(0);
            match column.alignment {
                Alignment::Left => write!(writer, " {:<width$} ", cell.as_ref(), width = width)?,
                Alignment::Center => write!(writer, " {:^width$} ", cell.as_ref(), width = width)?,
                Alignment::Right => write!(writer, " {:>width$} ", cell.as_ref(), width = width)?,
            }
        }
        writeln!(writer, "{}", style.end)
    }

    fn print_simple(&self, writer: &mut dyn Write) -> io::Result<()> {
        self.print_headers(writer)?;
        self.rows
            .iter()
            .try_for_each(|row| self.print_row(writer, row))
    }

    pub fn print_color_paginated<W: Write + WriteColor>(&self, writer: &mut W) -> io::Result<()> {
        let page_size = self.page_size.unwrap_or(self.rows.len());
        let total_pages = (self.rows.len() + page_size - 1) / page_size;

        for page in 0..total_pages {
            let start_page = page * page_size;
            let end_page = cmp::min((page + 1) * page_size, self.rows.len());

            writeln!(writer, "Page {} of {}", page + 1, total_pages)?;
            self.print_page(writer, start_page, end_page)?;

            if page < total_pages - 1 {
                writeln!(writer, "Press Enter to continue...")?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
            }
        }

        Ok(())
    }

    fn print_page<W: Write + WriteColor>(
        &self,
        writer: &mut W,
        start: usize,
        end: usize,
    ) -> io::Result<()> {
        self.print_headers(writer)?;
        for row in &self.rows[start..end] {
            self.print_row(writer, row)?;
        }
        Ok(())
    }

    fn print_styled(&self, writer: &mut dyn Write, style: &TableStyleConfig) -> io::Result<()> {
        self.print_line(writer, &style.top)?;
        self.print_row_styled(
            writer,
            &self.columns.iter().map(|c| &c.header).collect::<Vec<_>>(),
            &style.row,
        )?;
        self.print_line(writer, &style.below_header)?;
        for row in &self.rows {
            self.print_row_styled(writer, row, &style.row)?;
        }
        self.print_line(writer, &style.bottom)
    }

    pub fn set_page_size(&mut self, page_size: usize) {
        self.page_size = Some(page_size);
    }

    pub fn sort_by_column(&mut self, column_index: usize, ascending: bool) {
        self.rows.sort_by(|a, b| {
            let cmp = a[column_index].cmp(&b[column_index]);
            if ascending {
                cmp
            } else {
                cmp.reverse()
            }
        });
    }

    pub fn filter<F>(&self, predicate: F) -> Table
    where
        F: Fn(&[String]) -> bool,
    {
        let filtered_rows: Vec<Vec<String>> = self
            .rows
            .iter()
            .filter(|row| predicate(row))
            .cloned()
            .collect();

        Table {
            columns: self.columns.clone(),
            rows: filtered_rows,
            ..*self
        }
    }

    fn calculate_column_widths(&mut self) {
        if !self.auto_width {
            return;
        }

        for (i, column) in self.columns.iter_mut().enumerate() {
            if column.width.is_none() {
                let max_width = self
                    .rows
                    .iter()
                    .map(|row| row[i].len())
                    .chain(std::iter::once(column.header.len()))
                    .max()
                    .unwrap_or(0);
                column.width = Some(max_width + 2);
            }
        }
    }

    pub fn set_header_color(&mut self, color: CustomColor) {
        self.header_color = Some(color);
    }

    pub fn set_row_color(&mut self, color: CustomColor) {
        self.row_color = Some(color);
    }

    pub fn set_border_color(&mut self, color: CustomColor) {
        self.border_color = Some(color);
    }

    fn print_simple_color<W: Write + WriteColor>(
        &self,
        writer: &mut W,
        color_spec: &mut ColorSpec,
    ) -> io::Result<()> {
        if let Some(header_color) = self.header_color {
            color_spec.set_fg(Some(header_color.to_color()));
            writer.set_color(color_spec)?;
        }
        self.print_headers(writer)?;

        if let Some(row_color) = self.row_color {
            color_spec.set_fg(Some(row_color.to_color()));
            writer.set_color(color_spec)?;
        }
        for row in &self.rows {
            self.print_row(writer, row)?;
        }

        writer.reset()?;
        Ok(())
    }

    fn print_styled_color<W: Write + WriteColor>(
        &self,
        writer: &mut W,
        style: &TableStyleConfig,
        color_spec: &mut ColorSpec,
    ) -> io::Result<()> {
        self.print_line_color(writer, &style.top, color_spec)?;

        if let Some(header_color) = self.header_color {
            color_spec.set_fg(Some(header_color.to_color()));
            writer.set_color(color_spec)?;
        }
        self.print_row_styled(
            writer,
            &self.columns.iter().map(|c| &c.header).collect::<Vec<_>>(),
            &style.row,
        )?;

        if let Some(border_color) = self.border_color {
            color_spec.set_fg(Some(border_color.to_color()));
            writer.set_color(color_spec)?;
        }
        self.print_line_color(writer, &style.below_header, color_spec)?;

        if let Some(row_color) = self.row_color {
            color_spec.set_fg(Some(row_color.to_color()));
            writer.set_color(color_spec)?;
        }
        for row in &self.rows {
            self.print_row_styled(writer, row, &style.row)?;
        }

        if let Some(border_color) = self.border_color {
            color_spec.set_fg(Some(border_color.to_color()));
            writer.set_color(color_spec)?;
        }
        self.print_line_color(writer, &style.bottom, color_spec)?;

        writer.reset()?;
        Ok(())
    }

    fn print_line_color<W: Write + WriteColor>(
        &self,
        writer: &mut W,
        style: &LineStyle,
        color_spec: &mut ColorSpec,
    ) -> io::Result<()> {
        if let Some(border_color) = self.border_color {
            color_spec.set_fg(Some(border_color.to_color()));
            writer.set_color(color_spec)?;
        }
        self.print_line(writer, style)
    }
}
