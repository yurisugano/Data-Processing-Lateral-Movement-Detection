use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufReader, Write, BufRead};
use bzip2::read::BzDecoder;
use serde_json::Value;
use std::time::Instant;

fn main() {
    let output_file = "filtered_events32.txt";
    for i in 33..=90 {
        let filename = format!("wls_day-{:02}.bz2", i);

        // Record the start time
        let start_time = Instant::now();

        // Open the compressed file
        let file = match File::open(&filename) {
            Ok(file) => file,
            Err(e) => {
                println!("An error occurred while opening the compressed file: {:?}", e);
                continue; // Skip to the next iteration
            },
        };
        let decompressor = BzDecoder::new(file);
        let reader = BufReader::new(decompressor);

        // Open the output file in append mode, create it if it does not exist
        let mut out_file = match OpenOptions::new().append(true).create(true).open(output_file) {
            Ok(file) => file,
            Err(e) => {
                println!("An error occurred while opening the output file: {:?}", e);
                continue; // Skip to the next iteration
            },
        };

        // Process the file line by line to avoid loading everything into memory
        for line in reader.lines() {
            // ... Rest of the code remains the same ...
        }

        // Record the end time
        let elapsed_time = start_time.elapsed().as_secs_f64();
        println!("Time taken to process the file {}: {:.2} seconds", i, elapsed_time);
    }
}
