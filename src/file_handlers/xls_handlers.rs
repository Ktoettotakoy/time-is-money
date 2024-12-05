use calamine::{open_workbook, DataType, Reader, Xlsx};
use xlsxwriter::prelude::*;
use std::path::Path;

use crate::utils::folder_file_utils::{ prepare_folder_structure, get_latest_backup};
use crate::utils::structs::MonthExpenses;

const YEAR_MONTH_COLUMN: u32 = 1; // index of column "C" (A = 0 B = 1)
const STARTING_ROW: u32 = 1; // starting position of a table. (row 1 = pos 0, row 2 = pos 1)

const TMP_FOLDER: &str = "tmp";
const TMP_WORKBOOK_NAME: &str = "tmp_mask.xlsx"; // hardcoded for test
const RES_WORKBOOK_NAME: &str = "myAccTest.xlsx"; // hardcoded for test

// TODO! replace hardcoded workbook path with a variable in xls_insert_monthly_expenses 

// function has to unite existing .xlsx file and newly crated "mask" sheet 
// in order to fulfil main purpose - allow me inserting data using one click into the 
// excel spreadsheet

// returns true if success
pub fn xls_perform_workbook_update(me: MonthExpenses, destination_path: &str) -> bool{
    
    if let Err(e) = prepare_folder_structure(destination_path, RES_WORKBOOK_NAME) {
        println!("Error preparing folder structure: {:?}", e);
        return false;
    }
    
    // Prepare paths for backup and temp workbooks
    let result_workbook_path = Path::new(destination_path).join(RES_WORKBOOK_NAME);
    let backup_workbook_path = get_latest_backup(destination_path).expect("Failed to get latest backup");
    let tmp_workbook_path = Path::new(destination_path).join(TMP_FOLDER).join(TMP_WORKBOOK_NAME);
    
    
    // convert path to &str
    let backup_workbook_path_str = backup_workbook_path.to_str().unwrap();
    let tmp_workbook_path_str = tmp_workbook_path.to_str().unwrap();
    let result_workbook_path_str = result_workbook_path.to_str().unwrap();

    // Insert the new expense data into the "mask" workbook
    if !xls_insert_monthly_expense_entry_in_a_new_workbook(me.clone(), backup_workbook_path_str, tmp_workbook_path_str) {
        println!("Failed to create 'mask' workbook with new data.");
        return false;
    }
    
    
    // Open the existing workbook
    let mut existing_workbook: Xlsx<_> = open_workbook(backup_workbook_path_str).expect("Cannot open existing workbook");
    
    // Open the newly created "mask" workbook
    let mut new_workbook: Xlsx<_> = open_workbook(tmp_workbook_path_str).expect("Cannot open 'mask' workbook");
    
    // Create a new workbook to store the merged data
    let write_workbook_result: Result<Workbook, XlsxError> = Workbook::new(result_workbook_path_str);
    
    match write_workbook_result {
        Ok(merged_workbook) => {
            // Create a new sheet in the merged workbook
            let sheet_result = merged_workbook.add_worksheet(Some("Sheet1"));

            match sheet_result {
                Ok(mut sheet) => {
                    // Iterate over the Existing data and copy it to the new workbook
                    if let Some(Ok(existing_range)) = existing_workbook.worksheet_range("Sheet1") {
                        for row in 0..existing_range.height() {
                            for col in 0..existing_range.width() {
                                if let Some(cell) = existing_range.get_value((row as u32, col as u32)) {
                                    // Copy cell from existing workbook
                                    match cell {
                                        DataType::String(val) => {
                                            sheet.write_string(row as u32, col as u16, val, None).expect("Failed to write string");
                                        }
                                        DataType::Float(val) => {
                                            sheet.write_number(row as u32, col as u16, *val, None).expect("Failed to write number");
                                        }
                                        DataType::Int(val) => {
                                            sheet.write_number(row as u32, col as u16, *val as f64, None).expect("Failed to write int");
                                        }
                                        _ => {} // Handle other data types as necessary
                                    }
                                }
                            }
                        }
                    }

                    // Merge the "mask" workbook data into the new workbook
                    if let Some(Ok(new_range)) = new_workbook.worksheet_range("Sheet1") {
                        for row in 0..new_range.height() {
                            for col in 0..new_range.width() {
                                if let Some(cell) = new_range.get_value((row as u32, col as u32)) {
                                    // If the cell contains new data from the "mask", insert it
                                    match cell {
                                        DataType::String(val) => {
                                            sheet.write_string(row as u32, col as u16, val, None).expect("Failed to write string from mask");
                                        }
                                        DataType::Float(val) => {
                                            sheet.write_number(row as u32, col as u16, *val, None).expect("Failed to write number from mask");
                                        }
                                        DataType::Int(val) => {
                                            sheet.write_number(row as u32, col as u16, *val as f64, None).expect("Failed to write int from mask");
                                        }
                                        _ => {} // Handle other data types as necessary
                                    }
                                }
                            }
                        }
                    }

                    // Save the merged workbook with changes
                    merged_workbook.close().expect("Cannot save merged workbook");
                    return true;
                }
                Err(e) => {
                    println!("Failed to add sheet to the merged workbook: {:?}", e);
                    return false;
                }
            }
        }
        Err(e) => {
            println!("Failed to create new merged workbook: {:?}", e);
            return false;
        }
    }
}

// function which inserts data in a correct position in a new "mask" workbook
// due to xlsxwriter restrictions
// returns true if file was created successfully
pub fn xls_insert_monthly_expense_entry_in_a_new_workbook(me: MonthExpenses, path_to_back_up_workbook: &str, path_to_tmp_workbook: &str) -> bool {
    let year_to_find = me.year;
    let month_to_find = me.month;
    let expenses_data = me.expenses_data;
    
    // Find the row for the given year
    if let Some(year_row) = xls_find_year_entry_row_number(YEAR_MONTH_COLUMN, year_to_find, path_to_back_up_workbook) {
        
        // Find the correct row for the month
        if let Some(month_row) = xls_find_month_entry_row_number(year_row, month_to_find) {
            
            // Get categories from the same row
            if let Some(categories) = xls_categories_to_vec(year_row, path_to_back_up_workbook) {

                // Create a new workbook for writing (xlsxwriter cannot modify existing files directly)
                let write_workbook_result: Result<Workbook, XlsxError> = Workbook::new(path_to_tmp_workbook);
                match write_workbook_result{
                    Ok( workbook_result) => {
                        
                        let sheet_result = workbook_result.add_worksheet(Some("Sheet1"));

                        match sheet_result {
                            Ok(mut sheet) => {
                                // Loop through the categories and insert data from the hashmap
                                for (col, category) in categories.iter().enumerate() {
                                    if let Some(expense) = expenses_data.get(category) {
                                        // Insert expense into the corresponding column
                                        sheet.write_number(
                                            month_row as u32,
                                        col as u16 + YEAR_MONTH_COLUMN as u16 + 1,
                                    *expense, None).expect("Cannot write expense");
                                    }
                                }
                                
                                // Save the new workbook with changes
                                workbook_result.close().expect("Cannot save file");
                                return true;
                            }
                            Err(e) => {
                                println!("Failed to add sheet: {:?}", e);
                                return false
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to add worksheet: {:?}", e);
                        return false
                    }
                }
            }
        }
    }
    false
}



// To improve the performance I could do indexing first to avoid String comparisons

// Function to extract categories from a specific row in the Excel file
fn xls_categories_to_vec(row: u32, workbook_backup_path: &str) -> Option<Vec<String>> {
    let mut categories: Vec<String> = Vec::new();
    let mut workbook: Xlsx<_> = open_workbook(workbook_backup_path).expect("Cannot open file");

    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        // Iterate through the columns in the specified row
        let mut col = YEAR_MONTH_COLUMN + 1;
        while let Some(cell) = range.get_value((row, col)) {
            match cell {
                DataType::String(category) => {
                    // Add category to the vector
                    categories.push(category.clone());  
                }
                DataType::Empty => {
                    // Stop when an empty cell is encountered
                    println!("Empty Cell");
                    break;  
                }
                _ => {
                    println!("Category type mismatch {:?}", cell);
                    return None;
                }
            }
            col += 1;
        }
        if !categories.is_empty(){
            return Some(categories);
        }
    }
    println!("No Category found");
    None
}


// There exists column C which contains 
// year (int) followed by 12 month (String) = (13 rows).
// Each year is separated with 2 blank lines.
// So starting from base row and column I can reach any next year 
// shifting row number by 15

// returns row number of correct year entry
fn xls_find_year_entry_row_number(column: u32, year_to_find: i64, path_to_workbook: &str) -> Option<u32> {
   
    // opens a new workbook
    let mut workbook: Xlsx<_> = open_workbook(path_to_workbook).expect("Cannot open file");

    // Read whole worksheet data
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        let mut row_number: u32 = STARTING_ROW;
        
        while row_number < range.height().try_into().unwrap() {
            if let Some(cell) = range.get_value((row_number, column)) { 
                
                // println!("{}",cell); // debug print
                
                // Check if the cell contains the desired year
                match cell {
                    DataType::Int(year) => { // added just in case
                        if *year == year_to_find {
                            return Some(row_number);
                        }
                    }
                    DataType::Float(year_float) => {
                        // General cell format in excel converts any integer to floats
                        // Convert float to i64 and compare
                        if *year_float as i64 == year_to_find {
                            return Some(row_number); 
                        }
                    }
                    _ => {
                        println!("Unexpected cell type: {:?}", cell);
                        return None;
                    }
                }
            }
            
            // Skip 15 rows at a time as each year block takes 13 rows (12 months + year) 
            // followed by 2 blank rows
            row_number += 15;
        }
    }
    println!("No year found");
    None  
}

// There exists column C which contains 
// year (int) followed by 12 month (String) = (13 rows).
// Each year is separated with 2 blank lines.
// So starting from base row and column I can reach any next year 
// shifting row number by 15

// returns row number of correct month entry based on starting row (year)
fn xls_find_month_entry_row_number(year_row: u32, month_to_find: String) -> Option<u32> {
    let month_to_find = month_to_find.as_str();
    // Mapping months directly to their index
    let month_index = match month_to_find {
        "January" => 1,
        "February" => 2,
        "March" => 3,
        "April" => 4,
        "May" => 5,
        "June" => 6,
        "July" => 7,
        "August" => 8,
        "September" => 9,
        "October" => 10,
        "November" => 11,
        "December" => 12,
        _ => return None, // If the month is not valid, return None
        // in future I can add here string default output that would notify me that 
        // month in my .txt file has a typo or I messed up my format
    };

    // Since each month is in a fixed row order, we can directly calculate the row number
    Some(year_row + month_index as u32)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // used for manual tests
    // const WORKBOOK_PATH: &str = "src/data/test_file_result.xlsx";
    const WORKBOOK_PATH_LAST_BACK_UP: &str = "src/data/test_existing_workbook.xlsx";
    const WORKBOOK_PATH_TPM: &str = "src/data/test_mask_workbook.xlsx";

    #[test]
    fn test_xls_find_year_entry_row_number(){

        // this test assumes that we have a file src/data/test_file.xlsx
        // and starting position of tables is (1,2) - (row 2 column C)

        // here is the test table I use
        // |   C       |   D       |   E           |
        // |-----------|-----------|---------------|
        // | 2023      | Groceries | Sweets        | Restaurants |
        // | January   | 202.70    | 40.45         | 98.30       |
        // | February  | 219.82    | 52.80         | 8.80        |
        // | ...       | ...       | ...           | ...         |
        // | 2024      | ...       | ...           | ...         |   (Row 17, Column C)
        // | ...       | ...       | ...           | ...         |
        // | 2026      | ...       | ...           | ...         |   (Row 32, Column C)

        // check the present year in a test_file
        let mut year_to_find = 2026;
        let path = WORKBOOK_PATH_LAST_BACK_UP;
        
        // use function
        let result = xls_find_year_entry_row_number(YEAR_MONTH_COLUMN, year_to_find, path);
        // expect Some u32
        assert!(result.is_some());

        // test year which is not present
        year_to_find = 2022;
        let result = xls_find_year_entry_row_number(YEAR_MONTH_COLUMN, year_to_find, path);
        // expect None
        assert!(result.is_none());
    }

    #[test]
    fn test_xls_find_month_entry_row_number() {
        // This test assumes we have a file src/data/test_file.xlsx
        // with a valid year_row for testing purposes.
        let year_row = 1; // Example starting row for 2023

        // Test for a valid month
        let month_to_find = "January".to_string();
        let result = xls_find_month_entry_row_number(year_row, month_to_find);
        assert_eq!(result, Some(2)); // January should be in row 2 (year_row + 1)

        let month_to_find = "February".to_string();
        let result = xls_find_month_entry_row_number(year_row, month_to_find);
        assert_eq!(result, Some(3)); // February should be in row 3 (year_row + 2)

        // Test for an invalid month
        let month_to_find = "InvalidMonth".to_string();
        let result = xls_find_month_entry_row_number(year_row, month_to_find);
        assert!(result.is_none()); // Should return None for an invalid month
    }

    #[test]
    fn test_xls_categories_to_vec(){

        // this test assumes that we have a file src/data/test_file.xlsx
        // and starting position of tables is (1,2) - (row 2 column C)

        // here is the test table I use
        // |   C       |   D       |   E           |
        // |-----------|-----------|---------------|
        // | 2023      | Groceries | Sweets        | Restaurants |
        // | January   | 202.70    | 40.45         | 98.30       |
        // | February  | 219.82    | 52.80         | 8.80        |
        // | ...       | ...       | ...           | ...         |
        // | 2024      | ...       | ...           | ...         |   (Row 17, Column C)
        // | ...       | ...       | ...           | ...         |        
        // | 2026      | ...       | ...           | ...         |   (Row 32, Column C)


        let mut row = 1; // choose row number 2 with desired data
        let result = xls_categories_to_vec(row, WORKBOOK_PATH_LAST_BACK_UP);
        assert!(result.is_some());

        // compare expected with actual
        let expected_categories = vec![
            "Groceries".to_string(),
            "Sweets".to_string(),
            "Restaurants".to_string(),
        ];
        let actual_categories = result.unwrap();
        assert_eq!(actual_categories, expected_categories);

        row = 0; // choose row with incorrect data
        let result = xls_categories_to_vec(row, WORKBOOK_PATH_LAST_BACK_UP);
        assert!(result.is_none());
    }

    #[test]
    fn test_xls_insert_monthly_expense_entry() {
        // Prepare a sample MonthExpenses object
        let month_expenses = MonthExpenses {
            year: 2023,
            month: "January".to_string(),
            expenses_data: {
                let mut data = HashMap::new();
                data.insert("Groceries".to_string(), 150.00);
                data.insert("Other".to_string(), 75.50);
                data.insert("Sweets".to_string(), 50.00);
                data
            },
        };

        // Call the function to insert monthly expense entry
        let result = xls_insert_monthly_expense_entry_in_a_new_workbook(month_expenses, WORKBOOK_PATH_LAST_BACK_UP, WORKBOOK_PATH_TPM);
        
        // Check if the function returned true
        assert!(result);

        // Note this test should be verified manually to simplify the test.
        // Don't forget to delete new file before running the test again 
        // otherwise it can get falsely result, but generally it is impossible to 
        // break it until you use correct input format for .txt and existing .xls 

        // Since xlsxwriter doesn't allow modifying existing files,
        // test should create new file-mask that has to be used later on
        // to combine 2 files (workbooks join :D): existing and new
    }

    #[test]
    fn test_xls_perform_workbook_update() {
        // Step 1: Prepare test MonthExpenses data
        let month_expenses = MonthExpenses {
            year: 2023,
            month: "January".to_string(),
            expenses_data: {
                let mut data = HashMap::new();
                data.insert("Groceries".to_string(), 150.00);
                data.insert("Restaurants".to_string(), 98.30);
                data
            },
        };
    
        let destination = "src/data/final_test";

        // Step 3: Call the function
        let result = xls_perform_workbook_update(month_expenses, destination);
    
        // Check if the function returned true (success)
        assert!(result, "xls_perform_workbook_update should return true on success");
    }
}