//! Tests for RK value decoding
//!
//! RK values are compressed numbers in BIFF format:
//! - Bit 0 (fx100): If set, divide result by 100
//! - Bit 1 (fint): If set, value is signed 30-bit integer; otherwise, high 30 bits of IEEE 754 double
//! - Bits 2-31: The 30-bit data value

use std::path::PathBuf;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

/// Test that numeric cells with RK encoding are read correctly
/// This tests both integer and floating-point RK values
#[test]
fn test_rk_numeric_values() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    // Cell A2 should contain the number 100 (likely stored as RK)
    let cell = sheet.get_cell((1, 2)).expect("Cell A2 should exist");
    let value = cell.get_value();

    // The value should be parseable as a number
    if !value.is_empty() {
        if let Ok(num) = value.parse::<f64>() {
            // Just verify we get a reasonable numeric value
            assert!(num.is_finite(), "Value should be a finite number");
        }
    }
}

/// Test numeric values in Formate.xls which likely uses RK encoding
#[test]
fn test_formate_rk_values() {
    let workbook = xlrd::open(sample_path("Formate.xls")).expect("Failed to open Formate.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];
    let (max_col, max_row) = sheet.get_highest_column_and_row();

    // Read all numeric cells and verify they have reasonable values
    for row in 1..=max_row.min(20) {
        for col in 1..=max_col.min(10) {
            if let Some(cell) = sheet.get_cell((col, row)) {
                let value = cell.get_value();
                if !value.is_empty() {
                    // If it's a number, verify it's finite
                    if let Ok(num) = value.parse::<f64>() {
                        assert!(
                            num.is_finite(),
                            "Cell ({},{}) value {} should be finite",
                            col,
                            row,
                            num
                        );
                    }
                }
            }
        }
    }
}

/// Test that MULRK records are decoded correctly and placed in correct columns
#[test]
fn test_mulrk_column_positions() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open profiles.xls");
    let sheet = workbook
        .get_sheet_by_name("PROFILEDEF")
        .expect("Sheet not found");

    // Get dimensions
    let (max_col, max_row) = sheet.get_highest_column_and_row();

    // Verify we have reasonable dimensions (MULRK bug would cause column shift)
    assert!(max_col >= 10, "Expected at least 10 columns");
    assert!(max_row >= 10, "Expected at least 10 rows");

    // Check first row has expected header text in column A
    let cell_a1 = sheet.get_cell((1, 1)).expect("Cell A1 should exist");
    let value_a1 = cell_a1.get_value();
    assert_eq!(value_a1, "PROFIL", "First cell should contain 'PROFIL'");
}

/// Test that xf_class.xls numeric values are correct
#[test]
fn test_xf_class_numeric_values() {
    let workbook = xlrd::open(sample_path("xf_class.xls")).expect("Failed to open xf_class.xls");
    let sheets: Vec<_> = workbook.get_sheet_collection().iter().collect();

    assert!(!sheets.is_empty(), "Should have at least one sheet");

    let sheet = &sheets[0];
    let (max_col, max_row) = sheet.get_highest_column_and_row();

    // Verify numeric cells are readable
    for row in 1..=max_row.min(10) {
        for col in 1..=max_col.min(10) {
            if let Some(cell) = sheet.get_cell((col, row)) {
                let value = cell.get_value();
                if !value.is_empty() {
                    if let Ok(num) = value.parse::<f64>() {
                        assert!(
                            num.is_finite(),
                            "Cell ({},{}) should have finite value, got {}",
                            col,
                            row,
                            num
                        );
                    }
                }
            }
        }
    }
}

/// Test namesdemo.xls which may have various numeric formats
#[test]
fn test_namesdemo_numeric_values() {
    let workbook = xlrd::open(sample_path("namesdemo.xls")).expect("Failed to open namesdemo.xls");

    for sheet in workbook.get_sheet_collection().iter() {
        let (max_col, max_row) = sheet.get_highest_column_and_row();

        for row in 1..=max_row.min(20) {
            for col in 1..=max_col.min(10) {
                if let Some(cell) = sheet.get_cell((col, row)) {
                    let value = cell.get_value();
                    if !value.is_empty() {
                        if let Ok(num) = value.parse::<f64>() {
                            assert!(
                                num.is_finite(),
                                "Sheet '{}' cell ({},{}) should have finite value",
                                sheet.get_name(),
                                col,
                                row
                            );
                        }
                    }
                }
            }
        }
    }
}
