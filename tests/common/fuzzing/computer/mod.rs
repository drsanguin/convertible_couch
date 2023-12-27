use rand::{rngs::StdRng, RngCore, SeedableRng};

use super::{
    video_output::{FuzzedVideoOutput, VideoOutputFuzzer},
    win32_devices_display::FuzzedWin32DevicesDisplay,
    win32_graphics_gdi::FuzzedWin32GraphicsGdi,
};

pub struct FuzzedComputer {
    pub win32_devices_display: FuzzedWin32DevicesDisplay,
    pub win32_graphics_gdi: FuzzedWin32GraphicsGdi,
    pub primary_monitor: String,
    pub secondary_monitor: String,
}
pub struct ComputerFuzzer {
    pub video_outputs: Vec<FuzzedVideoOutput>,
    pub reboot_required: bool,
    video_output_fuzzer: VideoOutputFuzzer,
}

impl ComputerFuzzer {
    pub fn new(mut rand: StdRng) -> Self {
        let seed = rand.next_u64();

        Self {
            video_outputs: vec![],
            reboot_required: false,
            video_output_fuzzer: VideoOutputFuzzer::new(StdRng::seed_from_u64(seed)),
        }
    }

    pub fn with_two_monitors_or_more(&mut self) -> &mut ComputerFuzzer {
        self.video_outputs = self.video_output_fuzzer.generate_video_outputs(2);

        self
    }

    pub fn build_computer(&self) -> FuzzedComputer {
        let secondary_monitor = self
            .video_outputs
            .iter()
            .filter(|x| x.monitor.is_some())
            .map(|x| x.monitor.as_ref().unwrap())
            .find(|x| !x.primary)
            .unwrap()
            .name
            .clone();

        let primary_monitor = self
            .video_outputs
            .iter()
            .filter(|x| x.monitor.is_some())
            .map(|x| x.monitor.as_ref().unwrap())
            .find(|x| x.primary)
            .unwrap()
            .name
            .clone();

        let win32_devices_display = FuzzedWin32DevicesDisplay {
            video_outputs: self.video_outputs.clone(),
        };

        let win32_graphics_gdi = FuzzedWin32GraphicsGdi {
            video_outputs: self.video_outputs.clone(),
            reboot_required: self.reboot_required,
        };

        assert_ne!(
            secondary_monitor, primary_monitor,
            "Error during fuzzing ! Primary and secondary monitors are the same"
        );

        FuzzedComputer {
            secondary_monitor,
            primary_monitor,
            win32_devices_display,
            win32_graphics_gdi,
        }
    }

    pub fn which_requires_reboot(&mut self) -> &mut ComputerFuzzer {
        self.reboot_required = true;

        self
    }
}
