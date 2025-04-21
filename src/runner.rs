use qoe_core::{
    models::{ABRType, SimulationConfig},
    playback::engine::run_simulation,
    metrics::qoe::evaluate_qoe,
};

use crate::types::ScoreRecord;

pub fn run_simulation_for_config(config: &SimulationConfig) -> ScoreRecord {
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

#[cfg(test)]
mod tests {
    use super::*;
    use qoe_core::models::{ABRType, SimulationConfig};

    #[test]
    fn test_fixed_strategy_runs() {
        let config = SimulationConfig {
            abr_type: ABRType::Fixed,
            ..Default::default()
        };

        let result = run_simulation_for_config(&config);
        assert!(result.score > 0.0 && result.score <= 100.0);
    }

    #[test]
    fn test_buffer_based_strategy_runs() {
        let config = SimulationConfig {
            abr_type: ABRType::BufferBased,
            ..Default::default()
        };

        let result = run_simulation_for_config(&config);
        assert!(result.score > 0.0);
    }

    #[test]
    fn test_throughput_based_strategy_runs() {
        let config = SimulationConfig {
            abr_type: ABRType::ThroughputBased { window_size: 3 },
            ..Default::default()
        };

        let result = run_simulation_for_config(&config);
        assert!(result.average_bitrate > 0.0);
    }
}