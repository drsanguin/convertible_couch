use super::settings_api::FuzzedDisplay;

#[derive(Clone)]
pub struct FuzzedVideoOutput {
    pub device_name: String,
    pub display: Option<FuzzedDisplay>,
    index: usize,
}

impl FuzzedVideoOutput {
    pub fn new(index: usize, display: Option<FuzzedDisplay>) -> Self {
        let device_name = match display {
            Some(_) => format!(r"\\.\DISPLAY{index}\Monitor0"),
            None => format!(r"\\.\DISPLAY{index}"),
        };

        Self {
            device_name,
            display,
            index,
        }
    }

    pub fn plug_display(&self, display: FuzzedDisplay) -> Self {
        Self {
            device_name: format!(r"\\.\DISPLAY{}\Monitor0", self.index),
            display: Some(display),
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
