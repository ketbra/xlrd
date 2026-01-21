use xlrd;

fn main() {
    let path = "tests/test_data/DateFormats.xls";
    let workbook = xlrd::open(path).expect("Failed to open DateFormats.xls");

    let sheet = workbook.get_sheet(&0).expect("Failed to get first sheet");
    println!("Sheet dimensions: {} rows x {} cols", sheet.get_highest_row(), sheet.get_highest_column());

    // Check cells in column B (column 2)
    for row in 1..=10 {
        for col in 1..=5 {
            if let Some(cell) = sheet.get_cell((row, col)) {
                let value = cell.get_value();
                let data_type = cell.get_data_type();
                println!("Cell [{},{}]: value='{}', type={:?}", row, col, value, data_type);
            }
        }
    }
}
