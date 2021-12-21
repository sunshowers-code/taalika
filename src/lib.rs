// Copyright (c) taalika Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Builds plain, automatically-aligned tables of monospaced text.
//! This is basically what you want if you are implementing `ls`.
//!
//! # Example
//!
//! ```
//! use taalika::{Table, Row};
//! use std::path::Path;
//!
//! fn ls(dir: &Path) -> ::std::io::Result<()> {
//!     let mut table = Table::new("{:>}  {:<}{:<}  {:<}");
//!     for entry_result in ::std::fs::read_dir(dir)? {
//!         let entry    = entry_result?;
//!         let metadata = entry.metadata()?;
//!
//!         table.add_row(Row::new()
//!              .with_cell(metadata.len())
//!              .with_cell(if metadata.permissions().readonly() {"r"} else {""})
//!              .with_cell(if metadata.is_dir() {"d"} else {""})
//!              .with_cell(entry.path().display()));
//!     }
//!
//!     print!("{}", table);
//!
//!     Ok(())
//! }
//!
//! ls(Path::new("target")).unwrap();
//! ```
//!
//! produces something like
//!
//! ```text
//! 1198     target/.rustc_info.json
//! 1120  d  target/doc
//!  192  d  target/package
//! 1056  d  target/debug
//! ```
//!
//! # Other features
//!
//!   - The [`Table::with_heading`] and [`Table::add_heading`] methods add
//!     lines that span all columns.
//!
//!   - The [`row!`] macro builds a row with a fixed number of columns
//!     using less syntax.
//!
//!   - The [`Table::set_line_end`] method allows changing the line ending
//!     to include a carriage return (or whatever you want).
//!
//!   - The [`Row::with_ansi_cell`] and [`Row::add_ansi_cell`] methods can be
//!     used to add cells with ANSI color codes, and still have their widths be
//!     computed correctly.
//!
//! # Usage
//!
//! `taalika` is on [crates.io](https://crates.io/crates/taalika).
//!
//! # Features
//!
//! * Feature `unicode-width` is used to compute the width of columns in terms of
//!   Unicode graphemes. It is enabled by default and depends on the
//!   [unicode-width](https://crates.io/crates/unicode-width) crate.
//!
//!   Note that without `unicode-width`, alignment will be based on the count of the
//!   `std::str::Chars` iterator.
//!
//! This crate supports Rust version 1.46.0 and later.
//!
//! # See also
//!
//! You may also want:
//!
//! - [tabular](https://crates.io/crates/tabular) - `taalika` is a fork of this
//!   crate with additional features.
//!
//! - [text-tables](https://crates.io/crates/text-tables) – This is more automatic
//!   than tabular. You give it an array of arrays, it renders a nice table with
//!   borders. Tabular doesn't do borders.
//!
//! - [prettytable](https://crates.io/crates/prettytable-rs) — This has an API more
//!   similar to tabular’s in terms of building a table, but it does a lot more,
//!   including, color, borders, and CSV import.
//!
//! [`row!`]: macro.row.html

#![warn(missing_docs)]

mod column_spec;
mod error;
mod macros;
mod row;
mod table;
mod width_string;

pub use crate::{
    error::{Error, Result},
    row::Row,
    table::Table,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment() {
        let mut table = Table::new("{:>}  ({:<}) {:<}");
        table
            .add_row(Row::new().with_cell(1).with_cell("I").with_cell("one"))
            .add_row(Row::new().with_cell(5).with_cell("V").with_cell("five"))
            .add_row(Row::new().with_cell(10).with_cell("X").with_cell("ten"))
            .add_row(Row::new().with_cell(50).with_cell("L").with_cell("fifty"))
            .add_row(
                Row::new()
                    .with_cell(100)
                    .with_cell("C")
                    .with_cell("one-hundred"),
            );
        assert_eq!(
            format!("\n{}", table),
            r#"
  1  (I) one
  5  (V) five
 10  (X) ten
 50  (L) fifty
100  (C) one-hundred
"#
        );
    }

    #[test]
    fn heading() {
        let _row = Row::from_cells(vec!["a", "b", "c"]);
        //        eprintln!("{:?}", _row);

        let table = Table::new("{:<} {:<} {:>}")
            .with_row(Row::from_cells(vec!["a", "b", "d"]))
            .with_heading("This is my table")
            .with_row(Row::from_cells(vec!["ab", "bc", "cd"]));

        //        eprintln!("\n\n{:?}\n\n", table);

        assert_eq!(
            format!("\n{}", table),
            r#"
a  b   d
This is my table
ab bc cd
"#
        );
    }

    #[test]
    fn centering() {
        let table = Table::new("{:<} {:^} {:>}")
            .with_row(Row::from_cells(vec!["a", "b", "c"]))
            .with_row(Row::from_cells(vec!["a", "bc", "d"]))
            .with_row(Row::from_cells(vec!["a", "bcd", "e"]))
            .with_row(Row::from_cells(vec!["a", "bcde", "f"]))
            .with_row(Row::from_cells(vec!["a", "bcdef", "g"]));

        assert_eq!(
            format!("\n{}", table),
            r#"
a   b   c
a  bc   d
a  bcd  e
a bcde  f
a bcdef g
"#
        );
    }

    #[test]
    fn temporary() {}
}
