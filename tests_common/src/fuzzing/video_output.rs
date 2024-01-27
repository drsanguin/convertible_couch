use super::monitor::FuzzedMonitor;

#[derive(Clone)]
pub struct FuzzedVideoOutput {
    pub device_name: String,
    pub monitor: Option<FuzzedMonitor>,
    index: usize,
}

impl FuzzedVideoOutput {
    pub fn new(index: usize, monitor: Option<FuzzedMonitor>) -> Self {
        let device_name = match monitor {
            Some(_) => format!(r"\\.\DISPLAY{index}\Monitor0"),
            None => format!(r"\\.\DISPLAY{index}"),
        };

        Self {
            device_name,
            monitor,
            index,
        }
    }

    pub fn plug_monitor(&self, monitor: FuzzedMonitor) -> Self {
        Self {
            device_name: format!(r"\\.\DISPLAY{}\Monitor0", self.index),
            monitor: Some(monitor),
            index: self.index,
        }
    }
}

pub struct VideoOutputFuzzer {}

impl VideoOutputFuzzer {
    pub fn generate_several(count: usize) -> Vec<FuzzedVideoOutput> {
        (1..=count)
            .map(|video_output_number| FuzzedVideoOutput::new(video_output_number, None))
            .collect()
    }
}
