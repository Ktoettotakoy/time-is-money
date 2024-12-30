use std::{collections::HashMap, fs::File, io::Read, path::Path};
use crate::utils::structs::MonthExpenses;

pub fn show_debug_data_from_file(filepath: &str) -> String {
    if let Some(data) = transform_file_into_me_struct(filepath) {
        let sum: f64 = data.expenses_data.values().sum();

        let result = format!(
            "Total is {:.2}\nMetaData is {} {}\nBy category:\n{:?}", sum,
            data.year, data.month,
            serde_json::to_string_pretty(&data.expenses_data)
        );

        return result;
    }
    "".to_string() // return nothing
}

// receives filepath as input and transforms data from the file into format I want
pub fn transform_file_into_me_struct(filepath: &str) -> Option<MonthExpenses> {
    if let Some(data) = read_txt_file_to_string(filepath) {
        let mut lines = data.lines();

        // Assume the first line contains the month and year in the format "Month Year"
        if let Some(meta_data) = lines.next() {
            let parts: Vec<&str> = meta_data.split_whitespace().collect();
            if parts.len() < 2 {
                return None; // If there's not enough data, return None
            }

            let month = parts[0].to_string();
            let year = parts[1].parse::<i64>().ok()?;

            if let Some(expenses_data) = put_data_into_hashmap(&data){
                return Some(MonthExpenses {
                    year,
                    month,
                    expenses_data,
                });
            }
        }
    }

    None
}

// receives string slice with expected structure:
//  first line meta_data (month, year) not important
//  other lines should have format:
//      string (category) or float (expenses) or empty line (delimiter)
// there can be multiple entries of expenses followed by delimiter and next category after it
fn put_data_into_hashmap(data: &str) -> Option<HashMap<String, f64>> {
    let mut expenses_by_category: HashMap<String, f64> = HashMap::new();
    let mut lines = data.lines();

    // Skip the first line (month and year)
    let _meta_data = lines.next();

    let mut current_category = String::new();
    for line in lines {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            continue; // Skip empty lines
        }

        if let Ok(amount) = trimmed_line.parse::<f64>() {
            // If the line is a number, add it to the current category total
            if !current_category.is_empty() {
                *expenses_by_category.entry(current_category.clone()).or_insert(0.0) += amount;
            }
        } else {
            // If the line is not a number, it’s a new category
            current_category = trimmed_line.to_string();
        }
    }

    // Round each total to two decimal places
    for value in expenses_by_category.values_mut() {
        *value = (*value * 100.0).round() / 100.0;
    }

    if expenses_by_category.is_empty() {
        None
    } else {
        Some(expenses_by_category)
    }
}


// reads .txt file, converts it to huge string which is then passed
fn read_txt_file_to_string(filepath: &str) -> Option<String> {
    let mut buffer = String::new();

    if let Ok(mut file) = File::open(filepath) {
        if Path::new(filepath).extension().and_then(|s| s.to_str()) != Some("txt") {
            println!("File is not a .txt file");
            return None
        }
        if file.read_to_string(&mut buffer).is_ok() {
            return Some(buffer);
        }
    }
    println!("Error reading or opening file.");
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn test_read_txt_file_to_string_success() {
        // Create a temporary file path
        let test_file_path = "test1.txt";

        // Write some test data to the file
        {
            File::create(test_file_path).expect("Failed to create test file");
            let contents = "July\n\nGroceries";
            write(test_file_path, contents).expect("File write is failed");
        }

        // Test the function
        let result = read_txt_file_to_string(test_file_path);

        // Check if the function returned the correct string
        assert!(result.is_some());
        let content = result.unwrap();
        assert!(content.contains("July"));
        assert!(content.contains("Groceries"));

        // Clean up the test file
        std::fs::remove_file(test_file_path).expect("Failed to delete test file");
    }

    #[test]
    fn test_read_txt_file_to_string_file_not_found() {
        // Ensure the file doesn't exist
        let test_file_path = "non_existent_file.txt";
        // Test the function
        let result = read_txt_file_to_string(test_file_path);
        // Check if the function returned None because the file does not exist
        assert!(result.is_none());
    }

    #[test]
    fn test_read_txt_file_to_string_file_is_not_txt(){
        // Ensure the file doesn't exist
        let test_file_path = "non_existent_file.png";
        // Test the function
        let result = read_txt_file_to_string(test_file_path);
        // Check if the function returned None because the file does not exist
        assert!(result.is_none());
    }

    #[test]
    fn test_process_data_from_file() {

        let test_file_path = "test2.txt";

        // Write some test data to the file
        {
            File::create(test_file_path).expect("Failed to create test file");
            let contents = "July\n\nGroceries\n12.5\n12\n3.33\n\n\n\nSweets\n2.5\n7\n4.8\n\nRestaurant\n";
            write(test_file_path, contents).expect("File write is failed");
        }

        let json_string = show_debug_data_from_file(test_file_path);
        // Expected JSON strings (order may vary)
        let expected_json1 = "Total is 42.13\nBy category:\n{\n  \"Groceries\": 27.83,\n  \"Sweets\": 14.3\n}";
        let expected_json2 = "Total is 42.13\nBy category:\n{\n  \"Sweets\": 14.3,\n  \"Groceries\": 27.83\n}";

        // Assertions: Check if the result matches either of the expected JSON strings
        assert!(
            json_string.trim() == expected_json1 || json_string.trim() == expected_json2,
            "The output did not match either expected JSON string."
        );

        // Clean up the test file
        std::fs::remove_file(test_file_path).expect("Failed to delete test file");

        assert_eq!(show_debug_data_from_file(test_file_path), "")
    }
}
