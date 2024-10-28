use csv::ReaderBuilder;
use polars::prelude::*;
use std::error::Error;

pub fn read() -> Result<(), Box<dyn Error>> {
    // Specify the path to the CSV file
    let file_path = "medallists.csv";

    let df = CsvReader::from_path("medallists.csv")?
        .infer_schema(Some(100))
        .has_header(true)
        .finish()?;

    // Display the first 10 rows
    println!(
        "DataFrame shape: {} rows, {} columns",
        df.height(),
        df.width()
    );
    println!("Sample 10 rows have been displayed below:");
    println!("{:?}", df.head(Some(10)));

    Ok(())
}
