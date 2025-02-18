// SPDX-License-Identifier: MIT
// Project: tabprinter
// File: src/styles.rs
// Author: Volker Schwaberow <volker@schwaberow.de>
// Copyright (c) 2025 Volker Schwaberow

use crate::TableStyleConfig;
use crate::LineStyle;

macro_rules! define_styles {
    ($($name:ident: {
        $($field:ident: {
            $($inner_field:ident: $value:expr),+ $(,)?
        }),+ $(,)?
    }),+ $(,)?
    ) => {
        pub const STYLES: [TableStyleConfig; count_expr!($($name),+)] = [
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

macro_rules! count_expr {
    ($($e:expr),*) => { <[()]>::len(&[$(count_expr!(@sub $e)),*]) };
    (@sub $e:expr) => { () };
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
