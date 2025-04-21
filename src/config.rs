use qoe_core::models::{SimulationConfig, ABRType};
use crate::types::{Args, ABRSet};

pub fn build_configs(args: &Args) -> Vec<SimulationConfig> {
    match args.abr {
        ABRSet::All => vec![
            SimulationConfig {
                abr_type: ABRType::Fixed,
                ..Default::default()
            },
            SimulationConfig {
                abr_type: ABRType::BufferBased,
                ..Default::default()
            },
            SimulationConfig {
                abr_type: ABRType::ThroughputBased { window_size: args.window_size },
                ..Default::default()
            },
        ],
        ABRSet::Fixed => vec![SimulationConfig {
            abr_type: ABRType::Fixed,
            ..Default::default()
        }],
        ABRSet::BufferBased => vec![SimulationConfig {
            abr_type: ABRType::BufferBased,
            ..Default::default()
        }],
        ABRSet::ThroughputBased => vec![SimulationConfig {
            abr_type: ABRType::ThroughputBased { window_size: args.window_size },
            ..Default::default()
        }],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Args, ABRSet};

    #[test]
    fn test_all_configs_created()
    {
        let args = Args {
            abr: ABRSet::All,
            output: "test.csv".into(),
            window_size: 3,
            json: None,
        };

        let configs = build_configs(&args);
        assert_eq!(configs.len(), 3);
    }

    #[test]
    fn test_single_config() {
        let args = Args {
            abr: ABRSet::Fixed,
            output: "test.csv".into(),
            window_size: 3,
            json: None,
        };

        let configs = build_configs(&args);
        assert_eq!(configs.len(), 1);
    }
}