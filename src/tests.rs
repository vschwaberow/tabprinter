// SPDX-License-Identifier: MIT
// Project: tabprinter
// File: src/tests.rs
// Author: Volker Schwaberow <volker@schwaberow.de>
// Copyright (c) 2024 Volker Schwaberow

use super::*;

fn create_test_table(style: TableStyle) -> Table {
    let mut table = Table::new(style);
    table.add_column("Name", 8, Alignment::Left);
    table.add_column("Age", 5, Alignment::Right);
    table.add_column("City", 13, Alignment::Center);
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
    table
}

#[test]
fn test_amiga_table_no_crash() {
    let table = create_test_table(TableStyle::Amiga);
    let mut buffer = termcolor::Buffer::ansi();
    table.print_to_writer(&mut buffer).unwrap();
    assert!(!buffer.is_empty());
}

#[test]
fn test_add_column() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Test", 10, Alignment::Left);
    assert_eq!(table.columns.len(), 1);
    assert_eq!(table.columns[0].header, "Test");
    assert_eq!(table.columns[0].width, 10);
    assert!(matches!(table.columns[0].alignment, Alignment::Left));
}

#[cfg(feature = "csv")]
#[test]
fn test_csv_usage() {
    let table = Table::from_csv("examples/data.csv").unwrap();
    table.print().unwrap();
}

#[test]
fn test_add_row() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Test", 10, Alignment::Left);
    table.add_row(vec![Cell::new("Value")]);
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0][0].content, "Value");
}

#[test]
#[should_panic(expected = "Row length must match number of columns")]
fn test_add_row_mismatch() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Test", 10, Alignment::Left);
    table.add_row(vec![Cell::new("Value1"), Cell::new("Value2")]);
}

#[test]
fn test_print_color() {
    let table = create_test_table(TableStyle::Grid);
    let mut buffer = termcolor::Buffer::ansi();
    table.print_color(&mut buffer).unwrap();
    let result = String::from_utf8(buffer.into_inner()).unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_print_to_writer() {
    let table = create_test_table(TableStyle::Grid);
    let mut buffer = termcolor::Buffer::ansi();
    table.print_to_writer(&mut buffer).unwrap();
    let result = String::from_utf8(buffer.into_inner()).unwrap();
    assert!(!result.is_empty());
}

#[test]
fn test_auto_adjust_widths() {
    let mut table = create_test_table(TableStyle::Simple);
    table.auto_adjust_widths();
    assert!(table.columns.iter().all(|col| col.width > 0));
}

#[test]
fn test_sort_by_column() {
    let mut table = create_test_table(TableStyle::Simple);
    table.sort_by_column(1, true); // Sort by Age in ascending order
    assert_eq!(table.rows[0][1].content, "25");
    assert_eq!(table.rows[1][1].content, "30");
}

#[test]
fn test_filter_rows() {
    let table = create_test_table(TableStyle::Simple);
    let filtered = table.filter_rows(|row| row[1].content == "30"); // Filter rows where Age is 30
    assert_eq!(filtered.rows.len(), 1);
    assert_eq!(filtered.rows[0][1].content, "30");
}

#[test]
fn test_cell_style_default() {
    let style = CellStyle::new();
    assert!(!style.bold);
    assert!(!style.italic);
    assert!(!style.underline);
    assert_eq!(style.padding, 1);
}

#[test]
fn test_cell_padding() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Test", 10, Alignment::Left);
    let mut cell = Cell::new("Value");
    cell.style.padding = 2;
    table.add_row(vec![cell]);
    let mut buffer = termcolor::Buffer::ansi();
    table.print_to_writer(&mut buffer).unwrap();
    let result = String::from_utf8(buffer.into_inner()).unwrap();
    assert!(result.contains("  Value  "));
}

#[test]
fn test_number_formatting() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Number", 15, Alignment::Right);
    let mut cell = Cell::new("1234567.8910");
    cell.style.decimal_places = Some(2);
    cell.style.thousand_separator = true;
    table.add_row(vec![cell]);
    let mut buffer = termcolor::Buffer::ansi();
    table.print_to_writer(&mut buffer).unwrap();
    let result = String::from_utf8(buffer.into_inner()).unwrap();
    assert!(result.contains("1,234,567.89"));
}

#[test]
fn test_group_by_column_with_subtotals() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Category", 10, Alignment::Left);
    table.add_column("Amount", 10, Alignment::Right);
    table.add_row(vec![Cell::new("A"), Cell::new("100")]);
    table.add_row(vec![Cell::new("A"), Cell::new("200")]);
    table.add_row(vec![Cell::new("B"), Cell::new("300")]);
    table.add_row(vec![Cell::new("B"), Cell::new("400")]);
    table.group_by_column_with_subtotals(0);
    let mut buffer = termcolor::Buffer::ansi();
    table.print_to_writer(&mut buffer).unwrap();
    let result = String::from_utf8(buffer.into_inner()).unwrap();
    assert!(result.contains("Subtotal"));
    assert!(result.contains("300"));
    assert!(result.contains("700"));
}

#[test]
fn test_sum_column() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Amount", 10, Alignment::Right);
    table.add_row(vec![Cell::new("100")]);
    table.add_row(vec![Cell::new("200")]);
    table.add_row(vec![Cell::new("300")]);
    assert_eq!(table.sum_column(0), Some(600.0));
}

#[test]
fn test_average_column() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Amount", 10, Alignment::Right);
    table.add_row(vec![Cell::new("100")]);
    table.add_row(vec![Cell::new("200")]);
    table.add_row(vec![Cell::new("300")]);
    assert_eq!(table.average_column(0), Some(200.0));
}

#[test]
fn test_min_column() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Amount", 10, Alignment::Right);
    table.add_row(vec![Cell::new("100")]);
    table.add_row(vec![Cell::new("200")]);
    table.add_row(vec![Cell::new("300")]);
    assert_eq!(table.min_column(0), Some(100.0));
}

#[test]
fn test_max_column() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Amount", 10, Alignment::Right);
    table.add_row(vec![Cell::new("100")]);
    table.add_row(vec![Cell::new("200")]);
    table.add_row(vec![Cell::new("300")]);
    assert_eq!(table.max_column(0), Some(300.0));
}
