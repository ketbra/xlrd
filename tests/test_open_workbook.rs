//! Tests for opening workbooks, ported from Python xlrd test_open_workbook.py

use std::path::PathBuf;

fn sample_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("samples")
        .join(name)
}

/// Test that namesdemo.xls can be opened without error
#[test]
fn test_names_demo() {
    let result = xlrd::open(sample_path("namesdemo.xls"));
    assert!(result.is_ok(), "Failed to open namesdemo.xls: {:?}", result.err());
}

/// Test that issue20.xls with ragged rows can be opened
#[test]
fn test_ragged_rows_tidied_with_formatting() {
    let result = xlrd::open(sample_path("issue20.xls"));
    assert!(result.is_ok(), "Failed to open issue20.xls: {:?}", result.err());
}

/// Test that picture_in_cell.xls can be opened (tests BYTES_X00 handling)
#[test]
fn test_bytes_x00() {
    let result = xlrd::open(sample_path("picture_in_cell.xls"));
    assert!(result.is_ok(), "Failed to open picture_in_cell.xls: {:?}", result.err());
}

/// Test that opening an xlsx file returns an error
#[test]
fn test_open_xlsx() {
    let result = xlrd::open(sample_path("sample.xlsx"));
    // The Rust library uses cfb which should fail on xlsx files
    assert!(result.is_err(), "Opening xlsx should fail");
}

/// Test that opening an unknown/unsupported format returns an error
#[test]
fn test_open_unknown() {
    let result = xlrd::open(sample_path("sample.txt"));
    assert!(result.is_err(), "Opening txt file should fail");
}

/// Test that profiles.xls can be opened
#[test]
fn test_open_profiles() {
    let result = xlrd::open(sample_path("profiles.xls"));
    assert!(result.is_ok(), "Failed to open profiles.xls: {:?}", result.err());
}

/// Test that ragged.xls can be opened
#[test]
fn test_open_ragged() {
    let result = xlrd::open(sample_path("ragged.xls"));
    assert!(result.is_ok(), "Failed to open ragged.xls: {:?}", result.err());
}

/// Test that Formate.xls can be opened
#[test]
fn test_open_formate() {
    let result = xlrd::open(sample_path("Formate.xls"));
    assert!(result.is_ok(), "Failed to open Formate.xls: {:?}", result.err());
}

/// Test that xf_class.xls can be opened
#[test]
fn test_open_xf_class() {
    let result = xlrd::open(sample_path("xf_class.xls"));
    assert!(result.is_ok(), "Failed to open xf_class.xls: {:?}", result.err());
}

/// Test that formula_test_sjmachin.xls can be opened
#[test]
fn test_open_formula_test() {
    let result = xlrd::open(sample_path("formula_test_sjmachin.xls"));
    assert!(result.is_ok(), "Failed to open formula_test_sjmachin.xls: {:?}", result.err());
}

/// Test that formula_test_names.xls can be opened
#[test]
fn test_open_formula_names() {
    let result = xlrd::open(sample_path("formula_test_names.xls"));
    assert!(result.is_ok(), "Failed to open formula_test_names.xls: {:?}", result.err());
}
