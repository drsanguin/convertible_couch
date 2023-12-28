use super::monitor::FuzzedMonitor;

#[derive(Clone)]
pub struct FuzzedVideoOutput {
    pub id: String,
    pub monitor: Option<FuzzedMonitor>,
    index: usize,
}

impl FuzzedVideoOutput {
    pub fn new(index: usize, monitor: Option<FuzzedMonitor>) -> Self {
        let id = match monitor {
            Some(_) => format!(r"\\.\DISPLAY{index}\Monitor0"),
            None => format!(r"\\.\DISPLAY{index}"),
        };

        Self { id, monitor, index }
    }

    pub fn plug_monitor(&self, monitor: FuzzedMonitor) -> Self {
        Self {
            id: format!(r"\\.\DISPLAY{}\Monitor0", self.index),
            monitor: Some(monitor),
            index: self.index,
        }
    }
}

pub struct VideoOutputFuzzer {}

impl VideoOutputFuzzer {
    pub fn generate_video_outputs(count: usize) -> Vec<FuzzedVideoOutput> {
        (1..=count)
            .map(|video_output_number| FuzzedVideoOutput::new(video_output_number, None))
            .collect()
    }
}
