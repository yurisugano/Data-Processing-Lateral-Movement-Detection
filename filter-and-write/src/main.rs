use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};
use bzip2::read::BzDecoder;
use std::time::Instant;
use serde_json::Value;
use rayon::prelude::*;

fn process_file(path_to_bz2_files: &str, i: u32, output_file: &str) {
    let filename = format!("{}/wls_day-{:02}.bz2", path_to_bz2_files, i);
    println!("Trying to open file: {}", filename);

    // Record the start time
    let start_time = Instant::now();

    // Open the compressed file
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            println!("An error occurred while opening the compressed file: {:?}", e);
            return;
        },
    };
    let decompressor = BzDecoder::new(file);
    let reader = BufReader::new(decompressor);

    // Open the output file in append mode, create it if it does not exist
    let mut out_file = match OpenOptions::new().append(true).create(true).open(output_file) {
        Ok(file) => file,
        Err(e) => {
            println!("An error occurred while opening the output file: {:?}", e);
            return;
        },
    };

    // Process the file line by line to avoid loading everything into memory
    for line in reader.lines() {
        let line = line.unwrap();
        // Decode and load the JSON data
        if let Ok(entry) = serde_json::from_str::<Value>(&line) {
            // Check the conditions
            if (entry["EventID"] == 4624 || entry["EventID"] == 4634) && entry["LogonType"] == 10 && entry.get("Source").is_some() {
                // Write the values of the entry as a CSV line to the output file, including 'i' as the first value
                let values: Vec<String> = entry.as_object().unwrap().values().map(|v| v.to_string()).collect();
                let csv_line = format!("{},{}", i, values.join(","));
                writeln!(out_file, "{}", csv_line).unwrap();
            }
        } else {
            println!("An error occurred while processing the line");
        }
    }

    // Record the end time
    let elapsed_time = start_time.elapsed().as_secs_f64();
    println!("Time taken to process the file {}: {:.2} seconds", i, elapsed_time);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path_to_bz2_files>", args[0]);
        return;
    }
    let path_to_bz2_files = &args[1];
    let output_file = "filtered_events-rs.txt";

    (1..=84).into_par_iter().for_each(|i| {
        process_file(&path_to_bz2_files, i, &output_file);
    });
}
