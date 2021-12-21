# Changelog

This project adheres to [Semantic Versioning].

[Semantic Versioning]: http://semver.org/spec/v2.0.0.html

## [0.1.6] - 2021-12-21

### Changed

- Minor documentation updates.

## [0.1.5] - 2021-12-21

### Added

- `Row::with_ansi_cell` and `add_ansi_cell` add cells that potentially have
  ANSI color codes in them; the color codes are stripped out while computing
  the width.
- `Row::with_custom_width_cell` and `add_custom_width_cell` add cells with a
  custom width.

### Changed

- Project forked and renamed to `taalika`.

---

The following releases were for the tabular crate, which this is a fork of.

## [0.1.4] - 2019-12-29

### Changed
- Oldest supported Rust version is now 1.31.0.

## [0.1.3] - 2019-12-29

### Changed
- No longer depends on deprecated `str::trim_right` method.

## [0.1.2] - 2018-09-18

### Added
- `Table::set_line_end()` method for changing the line ending used by
formatted tables.
- `row!()` and `table!()` macros.

### Changed
- Centering now rounds to left rather than right; I think it looks better.

## [0.1.1] - 2018-09-18

### Added
- `{:^}` column specifier for centering.

## [0.1.0] - 2019/09/18

Initial release.

[0.1.6]: https://github.com/sunshowers-code/taalika/releases/tag/0.1.6
[0.1.5]: https://github.com/sunshowers-code/taalika/releases/tag/0.1.5
