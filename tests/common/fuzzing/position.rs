use std::fmt::Display;

use super::resolution::FuzzedResolution;

#[derive(Clone, Copy)]
pub struct FuzzedMonitorPosition {
    pub x: i32,
    pub y: i32,
}

impl Display for FuzzedMonitorPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct MonitorPositionFuzzer {}

impl MonitorPositionFuzzer {
    pub fn generate_positions(
        resolutions: &Vec<FuzzedResolution>,
        primary_monitor_number: usize,
    ) -> Vec<FuzzedMonitorPosition> {
        let n_monitor = resolutions.len();
        let mut monitors_positions = vec![FuzzedMonitorPosition { x: 0, y: 0 }; n_monitor];

        if primary_monitor_number != 1 {
            (0..=primary_monitor_number - 2)
                .rev()
                .for_each(|monitor_resolution_index| {
                    let monitor_resolution = resolutions[monitor_resolution_index];
                    let monitor_on_right_position =
                        monitors_positions[monitor_resolution_index + 1];
                    let monitor_resolution_width = i32::try_from(monitor_resolution.width).unwrap();

                    monitors_positions[monitor_resolution_index] = FuzzedMonitorPosition {
                        x: monitor_on_right_position.x - monitor_resolution_width,
                        y: 0,
                    };
                });
        }

        (primary_monitor_number..n_monitor).for_each(|monitor_resolution_index| {
            let monitor_resolution = resolutions[monitor_resolution_index];
            let monitor_on_left_position = monitors_positions[monitor_resolution_index - 1];
            let monitor_resolution_width = i32::try_from(monitor_resolution.width).unwrap();

            monitors_positions[monitor_resolution_index] = FuzzedMonitorPosition {
                x: monitor_on_left_position.x + monitor_resolution_width,
                y: 0,
            };
        });

        monitors_positions
    }
}
