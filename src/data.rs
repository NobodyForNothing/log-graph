use std::collections::HashMap;

/// Data gathered from log file.
pub struct LogData {
    /// Data rows.
    ///
    /// Format:
    /// - key: max value of sequence
    /// - value: sequence of data points
    pub data: HashMap<u64, Vec<Entry>>
}

impl LogData {
    /// Generate the rate of change from data.
    pub fn rate(&self) -> Self {
        let data = self.data.iter().map(|(k, v)| {
            let mut iter = v.iter();
            let mut last = iter.next().unwrap();

            let mut v = Vec::new();
            for e in iter {
                v.push(Entry {
                    time: e.time,
                    value: e.value - last.value,
                });
                last = e;
            }

            (*k, v)
        }).collect();
        LogData {
            data,
        }
    }
}

#[derive(Clone)]
pub struct Entry {
    /// Unix timestamp seconds.
    pub time: i64,
    pub value: i64,
}
