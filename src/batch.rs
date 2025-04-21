use serde::Serialize;
use qoe_core::{
    models::{ABRType, SimulationConfig},
    playback::engine::run_simulation,
    metrics::qoe::evaluate_qoe,
};
use crate::types::ScoreRecord;

pub fn run_test(config: &SimulationConfig) -> ScoreRecord {
    let metrics = run_simulation(config);
    let qoe = evaluate_qoe(&metrics);

    ScoreRecord {
        abr: abr_name(&config.abr_type).to_string(),
        score: qoe.final_score,
        average_bitrate: qoe.average_bitrate,
        stall_count: qoe.stall_count,
    }
}

fn abr_name(abr: &ABRType) -> &'static str {
    match abr {
        ABRType::Fixed => "Fixed",
        ABRType::BufferBased => "BufferBased",
        ABRType::ThroughputBased { .. } => "ThroughputBased",
    }
}