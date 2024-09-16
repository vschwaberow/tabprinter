// SPDX-License-Identifier: MIT
// Project: tabprinter
// File: src/lib.rs
// Author: Volker Schwaberow <volker@schwaberow.de>
// Copyright (c) 2024 Volker Schwaberow

use std::io::{self, Write};
use termcolor::{ColorChoice, ColorSpec, StandardStream, WriteColor};

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
        const STYLES: [TableStyleConfig; 8] = [
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
    }
}

pub struct Column {
    header: String,
    width: usize,
    alignment: Alignment,
}

pub struct Table {
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
    style: TableStyle,
}

impl Table {
    pub fn new(style: TableStyle) -> Self {
        Table {
            columns: Vec::new(),
            rows: Vec::new(),
            style,
        }
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
        }
    }

    pub fn add_column(&mut self, header: &str, width: usize, alignment: Alignment) {
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

    pub fn print(&self) -> io::Result<()> {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        self.print_color(&mut stdout)
    }

    pub fn print_color<W: Write + WriteColor>(&self, writer: &mut W) -> io::Result<()> {
        match self.style {
            TableStyle::Simple => self.print_simple(writer),
            TableStyle::Grid => self.print_styled(writer, &STYLES[1]),
            TableStyle::FancyGrid => self.print_styled(writer, &STYLES[2]),
            TableStyle::Clean => self.print_styled(writer, &STYLES[3]),
            TableStyle::Round => self.print_styled(writer, &STYLES[4]),
            TableStyle::Banner => self.print_styled(writer, &STYLES[5]),
            TableStyle::Block => self.print_styled(writer, &STYLES[6]),
            TableStyle::Amiga => self.print_amiga_color(writer),
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

    fn print_row(&self, writer: &mut dyn Write, row: &[String]) -> io::Result<()> {
        for (column, cell) in self.columns.iter().zip(row.iter()) {
            match column.alignment {
                Alignment::Left => write!(writer, "{:<width$}", cell, width = column.width - 1)?,
                Alignment::Center => write!(writer, "{:^width$}", cell, width = column.width - 1)?,
                Alignment::Right => write!(writer, "{:>width$}", cell, width = column.width - 1)?,
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
            write!(writer, "{}", style.hline.repeat(column.width + 2))?;
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
            match column.alignment {
                Alignment::Left => {
                    write!(writer, " {:<width$} ", cell.as_ref(), width = column.width)?
                }
                Alignment::Center => {
                    write!(writer, " {:^width$} ", cell.as_ref(), width = column.width)?
                }
                Alignment::Right => {
                    write!(writer, " {:>width$} ", cell.as_ref(), width = column.width)?
                }
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
}
