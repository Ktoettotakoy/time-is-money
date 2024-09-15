use std::collections::HashMap;

use calamine::{open_workbook, DataType, Reader, Xlsx};

use crate::file_handlers::txt_handlers::MonthExpenses;

const YEAR_MONTH_COLUMN: &str = "C";
const WORKBOOK_PATH: &str = "../data/test_file.xlsx";


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
fn xls_find_year_row_number(column: &str, year_to_find: i32) -> Option<i32> {
   
    let mut workbook: Xlsx<_> = open_workbook(WORKBOOK_PATH).expect("Cannot open file");
    // while()
   
    if false {
        return Some(1);
    }
   
    None 
}
