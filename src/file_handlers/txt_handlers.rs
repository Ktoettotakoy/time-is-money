use std::{collections::HashMap, fs::File, io::Read};
use serde_json::json;

pub fn process_data_from_file(filepath: &str) -> String {
    
    let mut expenses = put_data_into_hashmap(filepath);
    let mut sum = 0.0;

    for (_category, total) in expenses.iter_mut() {
        sum += *total;
    }

    println!("Total is {:.2}\nBy category:", (sum * 100.0).round() / 100.0);
    
    let json_expenses = json!(expenses);  // Serialize the HashMap to JSON format
    let result = serde_json::to_string_pretty(&json_expenses).unwrap();

    result
}

fn put_data_into_hashmap(filepath: &str) -> HashMap<String, f64> {
    let mut expenses_by_category: HashMap<String, f64> = HashMap::new();
    
    if let Some(data) = read_file_to_string(filepath) {
        let mut lines = data.lines();
        
        // Skip the first line (month name)
        let _month = lines.next();
        
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
    }

    expenses_by_category
}

fn read_file_to_string(filepath: &str) -> Option<String> {
    let mut buffer = String::new();

    if let Ok(mut file) = File::open(filepath) {
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
    fn test_read_file_to_string_success() {
        // Create a temporary file path
        let test_file_path = "test1.txt";

        // Write some test data to the file
        {
            File::create(test_file_path).expect("Failed to create test file");
            let contents = "July\n\nGroceries";
            write(test_file_path, contents).expect("File write is failed");
        }

        // Test the function
        let result = read_file_to_string(test_file_path);

        // Check if the function returned the correct string
        assert!(result.is_some());
        let content = result.unwrap();
        assert!(content.contains("July"));
        assert!(content.contains("Groceries"));

        // Clean up the test file
        std::fs::remove_file(test_file_path).expect("Failed to delete test file");
    }

    #[test]
    fn test_read_file_to_string_file_not_found() {
        // Ensure the file doesn't exist
        let test_file_path = "non_existent_file.txt";
        // Test the function
        let result = read_file_to_string(test_file_path);
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

        let json_string = process_data_from_file(test_file_path);

        // Assertions
        let expected_json = r#"{
  "Groceries": 27.83,
  "Sweets": 14.3
}"#;
        assert_eq!(json_string.trim(), expected_json);

        // Clean up the test file
        std::fs::remove_file(test_file_path).expect("Failed to delete test file");
    }
}