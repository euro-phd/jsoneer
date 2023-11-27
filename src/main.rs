use json::{object::Object, JsonValue};
use std::env;

fn print_json_value(value: &JsonValue, prefix: &str) {
    match value {
        JsonValue::Object(obj) => {
            for (key, val) in obj.iter() {
                match val {
                    JsonValue::Object(_) | JsonValue::Array(_) => {
                        println!("{}{}:", prefix, key);
                        print_json_value(val, &format!("{}    ", prefix));
                    }
                    _ => {
                        println!("{}{}: {}", prefix, key, val);
                    }
                }
            }
        }
        JsonValue::Array(arr) => {
            for (i, val) in arr.iter().enumerate() {
                println!("{}[{}] {}", prefix, i, val);
            }
        }
        _ => {
            println!("{}{}", prefix, value);
        }
    }
}

fn replace_json_values(value: &mut JsonValue, replacement: &str, option: &str) {
    match value {
        JsonValue::Object(obj) => {
            for (_, val) in obj.iter_mut() {
                replace_json_values(val, replacement, option);
            }
        }
        JsonValue::Array(arr) => {
            for val in arr.iter_mut() {
                replace_json_values(val, replacement, option);
            }
        }
        _ => {
            if option=="-n"{
                *value = JsonValue::Null;
            } else if option=="-t" {
                *value = JsonValue::Boolean(true);
            }else if option=="-f"{
                *value = JsonValue::Boolean(false);
            }else if option=="-a"{
                *value = JsonValue::Array(vec![]);
            }else if option=="-o"{
                *value = JsonValue::Object(json::object::Object::new());
            }else if option=="-u"{
                *value = JsonValue::String(replacement.to_string());
            }
        }
    }
}

fn main() {
    // Fetch command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure correct number of arguments provided
    match args.len() {
        1 => {
            println!("Please provide JSON string as an argument");
        }
        2 => {
            // Extract JSON string from argument
            let json_str = &args[1];

            // Parse the JSON string
            let parsed_json = json::parse(json_str).expect("Failed to parse JSON");

            match parsed_json {
                JsonValue::Object(map) => {
                    print_json_value(&JsonValue::Object(map.clone()), "");
                }
                _ => println!("JSON is not an object"),
            }
        }
        4 if args[1] == "-u" => {
            // Extract arguments
            let replacement = &args[2];
            let json_str = &args[3];

            // Parse the JSON string
            let mut parsed_json = json::parse(json_str).expect("Failed to parse JSON");

            // Replace values in the JSON
            replace_json_values(&mut parsed_json, replacement, &args[1]);

            // Print the modified JSON
            println!("{}", parsed_json.dump());
        }
        4 if args[1] == "-n" => {
            // Extract arguments
            let replacement = &args[2];
            let json_str = &args[3];

            // Parse the JSON string
            let mut parsed_json = json::parse(json_str).expect("Failed to parse JSON");

            // Replace values in the JSON
            replace_json_values(&mut parsed_json, replacement, &args[1]);

            // Print the modified JSON
            println!("{}", parsed_json.dump());
        }
        4 if args[1] == "-t" => {
            // Extract arguments
            let replacement = &args[2];
            let json_str = &args[3];

            // Parse the JSON string
            let mut parsed_json = json::parse(json_str).expect("Failed to parse JSON");

            // Replace values in the JSON
            replace_json_values(&mut parsed_json, replacement, &args[1]);

            // Print the modified JSON
            println!("{}", parsed_json.dump());
        }
        4 if args[1] == "-f" => {
            // Extract arguments
            let replacement = &args[2];
            let json_str = &args[3];

            // Parse the JSON string
            let mut parsed_json = json::parse(json_str).expect("Failed to parse JSON");

            // Replace values in the JSON
            replace_json_values(&mut parsed_json, replacement, &args[1]);

            // Print the modified JSON
            println!("{}", parsed_json.dump());
        }
        4 if args[1] == "-a" => {
            // Extract arguments
            let replacement = &args[2];
            let json_str = &args[3];

            // Parse the JSON string
            let mut parsed_json = json::parse(json_str).expect("Failed to parse JSON");

            // Replace values in the JSON
            replace_json_values(&mut parsed_json, replacement, &args[1]);

            // Print the modified JSON
            println!("{}", parsed_json.dump());
        }
        4 if args[1] == "-o" => {
            // Extract arguments
            let replacement = &args[2];
            let json_str = &args[3];

            // Parse the JSON string
            let mut parsed_json = json::parse(json_str).expect("Failed to parse JSON");

            // Replace values in the JSON
            replace_json_values(&mut parsed_json, replacement, &args[1]);

            // Print the modified JSON
            println!("{}", parsed_json.dump());
        }
        _ => {
            println!("Usage:");
            println!("For default mode (printing keys and values): command <json_string>");
            println!("For replacement mode: command -u <replacement_string> <json_string>");
        }
    }
}
