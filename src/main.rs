mod batch;
mod io;
mod types;
mod runner;
mod config;

use batch::run_test;
use std::path::Path;
use clap::{Parser, ValueEnum};
use io::{write_csv, write_json};
use types::{ABRSet, Args};
use crate::config::build_configs;

fn main() {
    let args = Args::parse();
    let mut configs = build_configs(&args);
    let mut records = vec![];

    let path = Path::new(&args.output);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Failed to create output directory");
        }
    }

    for config in configs {
        let record = run_test(&config);

        records.push(record.clone());
        println!(
            "ABR: {:<15} | Score: {:>5.1} | Bitrate: {:>5} kbps | Stalls: {}",
            record.abr, record.score, record.average_bitrate, record.stall_count
        );
    }

    write_csv(&args.output, &records).expect("Failed to write CSV");
    println!("Scores written to scores.csv");

    if let Some(json_path) = args.json.as_ref() {
        write_json(json_path, &records).expect("Failed to write JSON");
        println!("Scores written to {}", json_path);
    }
}