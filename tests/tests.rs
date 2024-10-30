use mobasserul_haque_individual_project2::{extract, load, query};
use std::fs;

#[test]
fn test_extract() {
    // Define the URL, file path, and directory
    let url = "https://raw.githubusercontent.com/fivethirtyeight/data/master/airline-safety/airline-safety.csv";
    let file_path = "data/airline-safety.csv";
    let directory = "data";

    // Run the extract function
    extract(url, file_path, directory);

    // Assert the file was created successfully
    assert!(fs::metadata(file_path).is_ok(), "Extracted file does not exist");
}

#[test]
fn test_load() {
    // Define the dataset path
    let dataset = "data/airline-safety.csv";

    // Run the load function
    let result = load(dataset);

    // Assert the database was created successfully
    assert!(
        result.is_ok(),
        "Failed to load data into the database: {:?}",
        result.err()
    );

    // Assert the database file exists
    assert!(fs::metadata("AirlineSafetyDB.db").is_ok(), "Database file does not exist");
}

#[test]
fn test_create_record() {
    let query = "INSERT INTO AirlineSafety (
        airline, avail_seat_km_per_week, incidents_85_99, fatal_accidents_85_99, fatalities_85_99,
        incidents_00_14, fatal_accidents_00_14, fatalities_00_14
    ) VALUES (
        'Test Airline', 500000, 2, 0, 0, 1, 0, 0
    );";

    // Run the query
    let result = query(query);

    // Assert the query executed successfully
    assert!(
        result.is_ok(),
        "Failed to create record in the database: {:?}",
        result.err()
    );
}

#[test]
fn test_query() {
    let query = "SELECT * FROM AirlineSafety WHERE airline = 'Aeroflot*';";

    // Run the query
    let result = query(query);

    // Assert the query executed successfully
    assert!(
        result.is_ok(),
        "Failed to execute general query: {:?}",
        result.err()
    );
}

#[test]
fn test_read_data() {
    let query = "SELECT * FROM AirlineSafety LIMIT 10;";

    // Run the query
    let result = query(query);

    // Assert the query executed successfully
    assert!(
        result.is_ok(),
        "Failed to read data from the database: {:?}",
        result.err()
    );
}
