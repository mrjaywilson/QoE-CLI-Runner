use clap::{Parser, ValueEnum};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ScoreRecord {
     pub abr: String,
     pub score: f32,
     pub average_bitrate: f32,
     pub stall_count: u32,
}

#[derive(Parser, Debug)]
#[command(name = "QOE Batch Runner")]
#[command(about = "Run batch simulations for different ABR algorithms", long_about = None)]
pub struct Args {
     // Choose which ABR to run
     #[arg(value_enum, default_value_t = ABRSet::All)]
     pub abr: ABRSet,

     // Output file for the results
     #[arg(short, long, default_value = "scores.csv")]
     pub output: String,

     // Throughput window size for throughput-based ABR
     #[arg(short = 'w', long, default_value_t = 3)]
     pub window_size: usize,

     #[arg(long)]
     pub json: Option<String>
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum ABRSet {
     #[clap(name = "all", alias = "a")]
     All,
     #[clap(name = "fixed", alias = "f")]
     Fixed,
     #[clap(name = "buffer", alias = "b")]
     BufferBased,
     #[clap(name = "throughput", alias = "t")]
     ThroughputBased,
}