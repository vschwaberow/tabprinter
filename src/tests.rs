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
        "Alice".to_string(),
        "30".to_string(),
        "New York".to_string(),
    ]);
    table.add_row(vec![
        "Bob".to_string(),
        "25".to_string(),
        "Los Angeles".to_string(),
    ]);
    table
}

#[test]
fn test_amiga_table_no_crash() {
    let table = create_test_table(TableStyle::Amiga);
    let mut output = Vec::new();
    table.print_to_writer(&mut output).unwrap();
    assert!(!output.is_empty());
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

#[test]
fn test_add_row() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Test", 10, Alignment::Left);
    table.add_row(vec!["Value".to_string()]);
    assert_eq!(table.rows.len(), 1);
    assert_eq!(table.rows[0][0], "Value");
}

#[test]
#[should_panic(expected = "Row length must match columns")]
fn test_add_row_mismatch() {
    let mut table = Table::new(TableStyle::Simple);
    table.add_column("Test", 10, Alignment::Left);
    table.add_row(vec!["Value1".to_string(), "Value2".to_string()]);
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
    let mut buffer = Vec::new();
    table.print_to_writer(&mut buffer).unwrap();
    let result = String::from_utf8(buffer).unwrap();
    assert!(!result.is_empty());
}
