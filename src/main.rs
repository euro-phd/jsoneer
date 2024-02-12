use json::{object::Object, JsonValue};
use std::env;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version,about, long_about = "jsoneer allows to create valid json input to test your app")]
struct Args{
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands{
    Show{value: String},
    Set{
        jsonfilepath: std::path::PathBuf,
        #[arg(short, long)]
        mode: String,
        #[arg(default_value="default")]
        value: String,
    }
}

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
            if option=="n"{
                *value = JsonValue::Null;
            } else if option=="t" {
                *value = JsonValue::Boolean(true);
            }else if option=="f"{
                *value = JsonValue::Boolean(false);
            }else if option=="a"{
                *value = JsonValue::Array(vec![]);
            }else if option=="o"{
                *value = JsonValue::Object(json::object::Object::new());
            }else if option=="u"{
                *value = JsonValue::String(replacement.to_string());
            }
        }
    }
}

fn show_smth(jsonfilepath: &str){
    // Extract JSON string from argument
    let json_str = std::fs::read_to_string(jsonfilepath).expect("file does not exist");
    // Parse the JSON string
    let mut parsed_json = json::parse(&json_str).expect("Failed to parse JSON");
    
    match parsed_json {
        JsonValue::Object(map) => {
            print_json_value(&JsonValue::Object(map.clone()), "");
        },
        JsonValue::Array(map) => {
            print_json_value(&JsonValue::Array(map.clone()), "");
        },
        _ => println!("JSON is not an object"),
    }
}


fn replace_on_mode(jsonfilepath: std::path::PathBuf, mode: &str, value: &str){
    println!("{}", mode);
    match mode{ 
    "u" => {
        // Extract arguments
        let replacement = &value;
        let json_file = jsonfilepath;
        let json_str = std::fs::read_to_string(&json_file).expect("file does not exist");

        // Parse the JSON string
        let mut parsed_json = json::parse(&json_str).expect("Failed to parse JSON");
        // Replace values in the JSON
        replace_json_values(&mut parsed_json, replacement, &mode);

        // Print the modified JSON
        println!("{}", parsed_json.dump());
    }
    "n" => {
        // Extract arguments
        let replacement = &value;
        let json_file = jsonfilepath;
        let json_str = std::fs::read_to_string(&json_file).expect("file does not exist");

        // Parse the JSON string
        let mut parsed_json = json::parse(&json_str).expect("Failed to parse JSON");
        // Replace values in the JSON
        replace_json_values(&mut parsed_json, replacement, &mode);

        // Print the modified JSON
        println!("{}", parsed_json.dump());
    }
    "t" => {
        // Extract arguments
        let replacement = &value;
        let json_file = jsonfilepath;
        let json_str = std::fs::read_to_string(&json_file).expect("file does not exist");

        // Parse the JSON string
        let mut parsed_json = json::parse(&json_str).expect("Failed to parse JSON");
        // Replace values in the JSON
        replace_json_values(&mut parsed_json, replacement, &mode);

        // Print the modified JSON
        println!("{}", parsed_json.dump());
    }
    "f" => {
        // Extract arguments
        let replacement = &value;
        let json_file = jsonfilepath;
        let json_str = std::fs::read_to_string(&json_file).expect("file does not exist");

        // Parse the JSON string
        let mut parsed_json = json::parse(&json_str).expect("Failed to parse JSON");
        // Replace values in the JSON
        replace_json_values(&mut parsed_json, replacement, &mode);

        // Print the modified JSON
        println!("{}", parsed_json.dump());
    }
    "a" => {
        // Extract arguments
        let replacement = &value;
        let json_file = jsonfilepath;
        let json_str = std::fs::read_to_string(&json_file).expect("file does not exist");

        // Parse the JSON string
        let mut parsed_json = json::parse(&json_str).expect("Failed to parse JSON");
        // Replace values in the JSON
        replace_json_values(&mut parsed_json, replacement, &mode);

        // Print the modified JSON
        println!("{}", parsed_json.dump());
    }
    "o" => {
        // Extract arguments
        let replacement = &value;
       let json_file = jsonfilepath;
        let json_str = std::fs::read_to_string(&json_file).expect("file does not exist");

        // Parse the JSON string
        let mut parsed_json = json::parse(&json_str).expect("Failed to parse JSON");
        // Replace values in the JSON
        replace_json_values(&mut parsed_json, replacement, &mode);

        // Print the modified JSON
        println!("{}", parsed_json.dump());
    }
    _ => {
        println!("YOU MADE WRONG CHOISE");
    }
}
}


fn main() {
    // Fetch command-line arguments
    let args = Args::parse();

    // Ensure correct number of arguments provided
    match args.cmd {
        Commands::Show{value} => show_smth(&value),
        Commands::Set{jsonfilepath, mode, value} => replace_on_mode(jsonfilepath, &mode, &value),
        _ => {        println!("Usage:");
        println!("For default mode (printing keys and values): command <json_string>");
        println!("For replacement mode: command -u <replacement_string> <json_string>");}
    }
}