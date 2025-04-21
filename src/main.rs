mod batch;

use qoe_core::models::{ABRType, SimulationConfig};
use batch::run_test;
use std::fs::File;
use std::path::Path;
use csv::Writer;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(name = "QOE Batch Runner")]
#[command(about = "Run batch simulations for different ABR algorithms", long_about = None)]
struct Args {
    // Choose which ABR to run
    #[arg(value_enum, default_value_t = ABRSet::All)]
    abr: ABRSet,

    // Output file for the results
    #[arg(short, long, default_value = "scores.csv")]
    output: String,

    // Throughput window size for throughput-based ABR
    #[arg(short = 'w', long, default_value_t = 3)]
    window_size: usize,

    #[arg(long)]
    json: Option<String>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ABRSet {
    #[clap(name = "all", alias = "a")]
    All,
    #[clap(name = "fixed", alias = "f")]
    Fixed,
    #[clap(name = "buffer", alias = "b")]
    BufferBased,
    #[clap(name = "throughput", alias = "t")]
    ThroughputBased,
}

fn main() {
    let args = Args::parse();
    let mut configs = vec![];

    match args.abr {
        ABRSet::All => {
            configs.push(SimulationConfig {
                abr_type: ABRType::Fixed,
                ..Default::default()
            });
            configs.push(SimulationConfig {
                abr_type: ABRType::BufferBased,
                ..Default::default()
            });
            configs.push(SimulationConfig {
                abr_type: ABRType::ThroughputBased { window_size: args.window_size },
                ..Default::default()
            });
        }
        ABRSet::Fixed => configs.push((SimulationConfig {
            abr_type: ABRType::Fixed,
            ..Default::default()
        })),
        ABRSet::BufferBased => configs.push((SimulationConfig {
            abr_type: ABRType::BufferBased,
            ..Default::default()
        })),
        ABRSet::ThroughputBased => configs.push((SimulationConfig {
            abr_type: ABRType::ThroughputBased { window_size: args.window_size },
            ..Default::default()
        })),
    }

    let path = Path::new(&args.output);

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent).expect("Failed to create output directory");
        }
    }

    let mut writer = Writer::from_path(&args.output).expect("Could not create CSV file.");
    writer.write_record(&["ABR", "Score", "AvgBitrateKbps", "Stalls"]).unwrap();

    let mut records = vec![];

    for config in configs {
        let record = run_test(&config);

        records.push(record.clone());
        println!(
            "ABR: {:<15} | Score: {:>5.1} | Bitrate: {:>5} kbps | Stalls: {}",
            record.abr, record.score, record.average_bitrate, record.stall_count
        );
        writer.serialize(record).expect("CSV write failed.");
    }

    writer.flush().expect("CSV flush failed.");
    println!("Scores written to scores.csv");

    if let Some(json_path) = args.json {
        let json_file = File::create(&json_path).expect("Failed to create JSON file");
        serde_json::to_writer_pretty(json_file, &records).expect("Failed to write JSON");
        println!("JSON written to {}", json_path)
    }
}