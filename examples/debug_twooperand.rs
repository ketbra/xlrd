use xlrd;

fn main() {
    let path = "tests/test_data/TwoOperandNumericFunctionTestCaseData.xls";
    let workbook = xlrd::open(path).expect("Failed to open TwoOperandNumericFunctionTestCaseData.xls");

    println!("Number of sheets: {}", workbook.get_sheet_count());

    for sheet_idx in 0..workbook.get_sheet_count() {
        if let Some(sheet) = workbook.get_sheet(&sheet_idx) {
            let name = sheet.get_name();
            println!("\nSheet {}: '{}' ({} rows x {} cols)",
                sheet_idx, name, sheet.get_highest_row(), sheet.get_highest_column());

            // Print first 20 rows and 10 columns
            for row in 1..=std::cmp::min(20, sheet.get_highest_row()) {
                for col in 1..=std::cmp::min(10, sheet.get_highest_column()) {
                    if let Some(cell) = sheet.get_cell((col, row)) {
                        let value = cell.get_value();
                        let data_type = cell.get_data_type();
                        if !value.is_empty() {
                            println!("  [{},{}]: value='{}', type={:?}", row, col, value, data_type);
                        }
                    }
                }
            }
        }
    }
}
