use std::fs::File;
use serde::{Deserialize, Serialize};
use csv::{Reader, Writer};
use std::path::{Path, PathBuf};

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
    pub fn from_csv<P: AsRef<Path>>(path: Option<P>) -> Self {
        let csv_path: PathBuf = match path {
            Some(p) => p.as_ref().to_path_buf(),
            None => PathBuf::from("emg_data\\emg.csv"), // defaults to emg_data\\.emg.csv
        };

        let mut rdr = Reader::from_path(csv_path).expect("Cannot open CSV file");
        let mut samples = Vec::new();

        for result in rdr.deserialize() {
            let record: EMGSample = result.expect("Error reading row");
            samples.push(record);
        }

        Self { samples, index: 0 }
    }
    /*
    pub fn from_csv<P: AsRef<Path>>(path: P) -> Self {
        let mut rdr = csv::Reader::from_path(get_out_dir(path).expect("Cannot open CSV file");
        let mut samples = Vec::new();
        for result in rdr.deserialize() {
            let record: EMGSample = result.expect("Error reading row");
            samples.push(record);
        }
        Self { samples, index: 0 }
    }
    */
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;
    use std::fs;

    #[test]
    fn test_from_csv_case1() {
        let mut temp = NamedTempFile::new().expect("failed to create temp file");
        writeln!(temp, "time,emg").unwrap();
        writeln!(temp, "0.0,1.0").unwrap();
        writeln!(temp, "0.1,1.2").unwrap();

        let reader = EMGReader::from_csv(Some(temp.path()));
        assert_eq!(reader.samples.len(), 2);
        assert_eq!(reader.samples[0].time, 0.0);
        assert_eq!(reader.samples[0].emg, 1.0);
        assert_eq!(reader.samples[1].time, 0.1);
        assert_eq!(reader.samples[1].emg, 1.2);
    }

    #[test]
    fn test_next_sample_case1() {
        let samples = vec![
            EMGSample { time: 0.0, emg: 1.0 },
            EMGSample { time: 0.1, emg: 1.2 },
        ];
        let mut reader = EMGReader {
            samples,
            index: 0,
        };

        let sample1 = reader.next_sample().unwrap();
        assert_eq!(sample1.time, 0.0);
        let sample2 = reader.next_sample().unwrap();
        assert_eq!(sample2.time, 0.1);
        assert!(reader.next_sample().is_none());
    }

    #[test]
    fn test_write_emg_to_csv_case1() {
        let data = vec![
            EMGOutput { time: 0.0, emg: 1.0, left: 0.5, right: 0.6 },
            EMGOutput { time: 0.1, emg: 1.2, left: 0.7, right: 0.8 },
        ];

        let temp = NamedTempFile::new().expect("failed to create temp file");
        let path_str = temp.path().to_str().unwrap();

        write_emg_to_csv(path_str, &data);

        let contents = fs::read_to_string(path_str).expect("failed to read back file");
        assert!(contents.contains("time,emg,left,right"));
        assert!(contents.contains("0.0,1.0,0.5,0.6"));
        assert!(contents.contains("0.1,1.2,0.7,0.8"));
    }

    #[test]
    #[should_panic(expected = "Cannot open CSV file")]
    fn test_from_csv_invalid_path_panics() {
        EMGReader::from_csv(Some("this/does/not/exist.csv"));
    }
}
