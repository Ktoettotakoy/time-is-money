use std::{any::Any, collections::HashMap};

use calamine::{open_workbook, DataType, Reader, Xlsx};

use crate::file_handlers::txt_handlers::MonthExpenses;

const YEAR_MONTH_COLUMN: u32 = 2; // index of column "C" (A = 0 B = 1)
const STARTING_ROW: u32 = 1; // starting position of a table. (row 1 = pos 0, row 2 = pos 1)
const WORKBOOK_PATH: &str = "src/data/test_file.xlsx"; // hardcoded for now (?)


// function which executes all logic of this file 
// returns true if file was modified successfully
pub fn xls_insert_monthly_expense(me: MonthExpenses) -> bool{
    !unimplemented!()
}


// To improve the performance I could do indexing first to avoid String comparisons

// stores data from expenses hashmap into the correct columns by category
// (order in Excel file may vary and hashmap don't care about ordering as well)
fn xls_put_expenses(expenses_data: HashMap<String, f64>){
    !unimplemented!()
}


// returns the ordered vector of categories present in expenses tracking xls file
fn xls_categories_to_vec(row: i32) -> Option<Vec<String>>{
    !unimplemented!()
}


// There exists column C which contains 
// year (int) followed by 12 month (String) = (13 rows).
// Each year is separated with 2 blank lines.
// So starting from base row and column I can reach any next year 
// shifting row number by 15

// returns row number of correct year entry
fn xls_find_year_entry_row_number(column: u32, year_to_find: i64) -> Option<u32> {
   
   // opens a new workbook
   let mut workbook: Xlsx<_> = open_workbook(WORKBOOK_PATH).expect("Cannot open file");

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
    
    None  
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xls_find_year_entry_row_number(){

        // this test assumes that we have a file src/data/test_file.xlsx
        // and starting position of tables is (1,2) - (row 2 column C)

        // here is the test table I use
        //2023	    Groceries 	Sweets	Restaurants (2023 is located at row 2 column C)
        //January	202.70	    40.45	98.30
        //February	219.82	    52.80	8.80
        //March	    273.33	    69.58	134.25
        //April	    391.32	    66.12	28.75
        //May	    363.26	    65.20	15.00
        //June	    245.18	    59.88	168.38
        //July	    168.49	    54.40	252.00
        //August	243.58	    54.58	77.98
        //September			
        //October			
        //November			
        //December			
        // + number 2026 at row 32 column C and number 2024 at row 17 column C

        // check the present year in a test_file
        let mut year_to_find = 2026;
        
        // use function
        let result = xls_find_year_entry_row_number(YEAR_MONTH_COLUMN, year_to_find);
        // expect Some u32
        assert!(result.is_some());

        // test year which is not present
        year_to_find = 2022;
        let result = xls_find_year_entry_row_number(YEAR_MONTH_COLUMN, year_to_find);
        // expect None
        assert!(result.is_none());
    }
}