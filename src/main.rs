use std::env;
use mobasserul_haque_individual_project2::{extract, load, query as query_fn};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} [action]", args[0]);
        return;
    }

    let action = &args[1];

    match action.as_str() {
        "extract" => {
            let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/airline-safety/airline-safety.csv";
            let file_path = "data/airline-safety.csv";
            let directory = "data";

            println!("Extracting data...");
            extract(url, file_path, directory);
            println!("Extraction completed.");
        }
        "load" => {
            let file_path = "data/airline-safety.csv";
            println!("Loading data into database...");
            if let Err(e) = load(file_path) {
                eprintln!("Failed to load data: {:?}", e);
            }
        }
        "query" => {
            if let Some(query) = args.get(2) {
                if let Err(err) = query_fn(query) {
                    eprintln!("Query failed: {:?}", err);
                }
            } else {
                println!("Usage: {} query [SQL query]", args[0]);
            }
        }
        _ => {
            println!("Invalid action. Use 'extract', 'load', or 'query'.");
        }
    }
}
