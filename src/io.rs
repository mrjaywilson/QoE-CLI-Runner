use std::fs::File;
use std::io::{Write};
use csv::Writer;
use serde_json;

use crate::types::ScoreRecord;

pub fn write_csv(path: &str, records: &[ScoreRecord]) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = Writer::from_path(path)?;
    writer.write_record(&["ABR", "Score", "AvgBitrateKbps", "StallCount"])?;

    for record in records {
        writer.serialize(record)?;
    }

    writer.flush()?;

    Ok(())
}

pub fn write_json(path: &str, records: &[ScoreRecord]) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    let json = serde_json::to_string_pretty(records)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}