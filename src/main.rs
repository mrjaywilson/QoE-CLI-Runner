mod batch;

use qoe_core::models::{ABRType, SimulationConfig};
use batch::run_test;
use std::fs::File;
use csv::Writer;

fn main() {
    let configs = vec![
        SimulationConfig {
            abr_type: ABRType::Fixed,
            ..Default::default()
        },
        SimulationConfig {
            abr_type: ABRType::BufferBased,
            ..Default::default()
        },
        SimulationConfig {
            abr_type: ABRType::ThroughputBased { window_size: 3 },
            ..Default::default()
        }
    ];

    let mut writer = Writer::from_path("scores.csv").expect("Could not create CSV file.");
    writer.write_record(&["ABR", "Score", "AvgBitrateKbps", "Stalls"]).unwrap();

    for config in configs {
        let record = run_test(&config);
        println!(
            "ABR: {:<15} | Score: {:>5.1} | Bitrate: {:>5} kbps | Stalls: {}",
            record.abr, record.score, record.average_bitrate, record.stall_count
        );
        writer.serialize(record).expect("CSV write failed.");
    }

    writer.flush().expect("CSV flush failed.");
    println!("Scores written to scores.csv");
}