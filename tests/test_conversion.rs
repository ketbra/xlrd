//! Tests for XLS to XLSX conversion functionality

use std::path::PathBuf;
use std::fs;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

fn temp_dir() -> PathBuf {
    let temp = std::env::temp_dir().join("xlrd_tests");
    fs::create_dir_all(&temp).ok();
    temp
}

/// Test converting profiles.xls to xlsx
#[test]
fn test_convert_profiles() {
    let src = sample_path("profiles.xls");
    let dest = temp_dir().join("profiles_converted.xls");

    // Copy source to temp location with .xls extension for conversion
    fs::copy(&src, &dest).expect("Failed to copy file");

    let result = xlrd::xls2xlsx(&dest);
    assert!(result.is_ok(), "Conversion failed: {:?}", result.err());

    // Verify the xlsx file was created
    let xlsx_path = dest.with_extension("xlsx");
    assert!(xlsx_path.exists(), "XLSX file should exist");

    // Cleanup
    fs::remove_file(&dest).ok();
    fs::remove_file(&xlsx_path).ok();
}

/// Test converting and saving maintains sheet count
#[test]
fn test_convert_maintains_sheets() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open");
    let original_count = workbook.get_sheet_count();

    let dest = temp_dir().join("profiles_test.xlsx");
    xlrd::save(&workbook, &dest).expect("Failed to save");

    // Read back and verify
    let reopened = umya_spreadsheet::reader::xlsx::read(&dest).expect("Failed to reopen");
    let new_count = reopened.get_sheet_count();

    assert_eq!(original_count, new_count, "Sheet count should be preserved");

    // Cleanup
    fs::remove_file(&dest).ok();
}

/// Test converting maintains cell values
#[test]
fn test_convert_maintains_values() {
    let workbook = xlrd::open(sample_path("profiles.xls")).expect("Failed to open");

    // Get original value from first cell
    let original_value = {
        let sheet = workbook.get_sheet_by_name("PROFILEDEF").expect("Sheet not found");
        let cell = sheet.get_cell((1, 1)).expect("Cell not found");
        cell.get_value().to_string()
    };

    let dest = temp_dir().join("profiles_values_test.xlsx");
    xlrd::save(&workbook, &dest).expect("Failed to save");

    // Read back and verify
    let reopened = umya_spreadsheet::reader::xlsx::read(&dest).expect("Failed to reopen");
    let sheet = reopened.get_sheet_by_name("PROFILEDEF").expect("Sheet not found");
    let cell = sheet.get_cell((1, 1)).expect("Cell not found");
    let new_value = cell.get_value().to_string();

    assert_eq!(original_value, new_value, "Cell value should be preserved");

    // Cleanup
    fs::remove_file(&dest).ok();
}

/// Test xls2xlsx returns error for non-.xls files
#[test]
fn test_xls2xlsx_requires_xls_extension() {
    let path = sample_path("sample.xlsx");
    let result = xlrd::xls2xlsx(&path);

    assert!(result.is_err(), "Should fail for .xlsx files");
}

/// Test converting ragged.xls
#[test]
fn test_convert_ragged() {
    let workbook = xlrd::open(sample_path("ragged.xls")).expect("Failed to open");

    let dest = temp_dir().join("ragged_test.xlsx");
    let result = xlrd::save(&workbook, &dest);

    assert!(result.is_ok(), "Conversion should succeed: {:?}", result.err());

    // Cleanup
    fs::remove_file(&dest).ok();
}

/// Test converting Formate.xls with formatting
#[test]
fn test_convert_formate() {
    let workbook = xlrd::open(sample_path("Formate.xls")).expect("Failed to open");

    let dest = temp_dir().join("formate_test.xlsx");
    let result = xlrd::save(&workbook, &dest);

    assert!(result.is_ok(), "Conversion should succeed: {:?}", result.err());

    // Cleanup
    fs::remove_file(&dest).ok();
}

/// Test converting xf_class.xls with extensive formatting
#[test]
fn test_convert_xf_class() {
    let workbook = xlrd::open(sample_path("xf_class.xls")).expect("Failed to open");

    let dest = temp_dir().join("xf_class_test.xlsx");
    let result = xlrd::save(&workbook, &dest);

    assert!(result.is_ok(), "Conversion should succeed: {:?}", result.err());

    // Cleanup
    fs::remove_file(&dest).ok();
}
