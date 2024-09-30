use std::collections::HashMap;

// struct (me) that is used to correctly locate and insert data into excel
pub struct MonthExpenses {
    pub year: i64, // because of Microsoft Excel
    pub month: String,
    pub expenses_data: HashMap<String, f64>,
}