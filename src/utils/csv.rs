use std::path::Path;
use std::fs::File;
use serde::{Deserialize, Serialize};
use csv::{Reader, Writer};

#[derive(Debug, Deserialize, Clone)]
pub struct EMGSample {
    pub time: f32,
    pub emg: f32,
}

#[derive(Debug, Serialize)]
pub struct EMGOutput {
    pub time: f32,
    pub emg: f32,
    pub left: f32,
    pub right: f32,
}

pub struct EMGReader {
    pub samples: Vec<EMGSample>,
    pub index: usize,
}


impl EMGReader {
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Self {
        let mut rdr = csv::Reader::from_path(path).expect("Cannot open CSV file");
        let mut samples = Vec::new();
        for result in rdr.deserialize() {
            let record: EMGSample = result.expect("Error reading row");
            samples.push(record);
        }
        Self { samples, index: 0 }
    }

    pub fn next_sample(&mut self) -> Option<EMGSample> {
        if self.index < self.samples.len() {
            let sample = self.samples[self.index].clone();
            self.index += 1;
            Some(sample)
        } else {
            None
        }
    }
}

pub fn write_emg_to_csv(path: &str, data: &[EMGOutput]) {
    let file = File::create(path).expect("Cannot create file");
    let mut wtr = Writer::from_writer(file);

    for row in data {
        wtr.serialize(row).expect("Failed to write row");
    }

    wtr.flush().expect("Failed to flush writer");
}
