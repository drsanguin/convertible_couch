use std::fmt::Display;

use rand::{rngs::StdRng, seq::SliceRandom, Rng};

use super::resolution::FuzzedResolution;

#[derive(Clone, Copy)]
pub struct FuzzedMonitorPosition {
    pub x: i32,
    pub y: i32,
}

impl FuzzedMonitorPosition {
    pub fn is_positioned_at_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

#[derive(Clone, Copy)]
pub struct FuzzedMonitorPositionedResolution {
    pub resolution: FuzzedResolution,
    pub position: FuzzedMonitorPosition,
}

impl Display for FuzzedMonitorPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct MonitorPositionFuzzer {
    rand: StdRng,
}

impl MonitorPositionFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_positions(
        &mut self,
        resolutions: &Vec<FuzzedResolution>,
        has_an_internal_display: bool,
    ) -> Vec<FuzzedMonitorPositionedResolution> {
        let n_monitor = resolutions.len();

        let primary_monitor_index = if has_an_internal_display {
            0
        } else {
            self.rand.gen_range(0..n_monitor)
        };
        let primary_monitor_resolution = resolutions[primary_monitor_index];
        let primary_monitor_position = FuzzedMonitorPosition { x: 0, y: 0 };

        let mut n_mon_res_pos_up = 0;
        let mut n_mon_res_pos_right_up = 0;
        let mut n_mon_res_pos_right = 0;
        let mut n_mon_res_pos_right_down = 0;
        let mut n_mon_res_pos_down = 0;
        let mut n_mon_res_pos_left_down = 0;
        let mut n_mon_res_pos_left = 0;
        let mut n_mon_res_pos_left_up = 0;

        (0..n_monitor).for_each(|i| match i {
            i if i % 7 == 0 => n_mon_res_pos_left_up += 1,
            i if i % 6 == 0 => n_mon_res_pos_left += 1,
            i if i % 5 == 0 => n_mon_res_pos_left_down += 1,
            i if i % 4 == 0 => n_mon_res_pos_down += 1,
            i if i % 3 == 0 => n_mon_res_pos_right_down += 1,
            i if i % 2 == 0 => n_mon_res_pos_right += 1,
            i if i % 1 == 0 => n_mon_res_pos_right_up += 1,
            _ => n_mon_res_pos_up += 1,
        });

        let mon_res_pos_up_lim = n_mon_res_pos_up;
        let mon_res_pos_right_up_lim = mon_res_pos_up_lim + n_mon_res_pos_right_up;
        let mon_res_pos_right_lim = mon_res_pos_right_up_lim + n_mon_res_pos_right;
        let mon_res_pos_right_down_lim = mon_res_pos_right_lim + n_mon_res_pos_right_down;
        let mon_res_pos_down_lim = mon_res_pos_right_down_lim + n_mon_res_pos_down;
        let mon_res_pos_left_down_lim = mon_res_pos_down_lim + n_mon_res_pos_left_down;
        let mon_res_pos_left_lim = mon_res_pos_left_down_lim + n_mon_res_pos_left;
        let mon_res_pos_left_up_lim = mon_res_pos_left_lim + n_mon_res_pos_left_up;

        let mut mon_res_pos_up = Vec::with_capacity(n_mon_res_pos_up);
        let mut mon_res_pos_right_up = Vec::with_capacity(n_mon_res_pos_right_up);
        let mut mon_res_pos_right = Vec::with_capacity(n_mon_res_pos_right);
        let mut mon_res_pos_right_down = Vec::with_capacity(n_mon_res_pos_right_down);
        let mut mon_res_pos_down = Vec::with_capacity(n_mon_res_pos_down);
        let mut mon_res_pos_left_down = Vec::with_capacity(n_mon_res_pos_left_down);
        let mut mon_res_pos_left = Vec::with_capacity(n_mon_res_pos_left);
        let mut mon_res_pos_left_up = Vec::with_capacity(n_mon_res_pos_left_up);

        let primary_monitor_positioned = FuzzedMonitorPositionedResolution {
            position: primary_monitor_position,
            resolution: primary_monitor_resolution,
        };

        let mut positions = Vec::with_capacity(n_monitor);

        resolutions
            .iter()
            .enumerate()
            .filter(|(resolution_index, _resolution)| *resolution_index != primary_monitor_index)
            .enumerate()
            .map(|(resolution_index, (_, resolution))| (resolution_index, resolution))
            .for_each(|(resolution_index, resolution)| {
                Self::position_resolution(
                    resolution_index,
                    mon_res_pos_up_lim,
                    mon_res_pos_right_up_lim,
                    mon_res_pos_right_lim,
                    mon_res_pos_right_down_lim,
                    mon_res_pos_down_lim,
                    mon_res_pos_left_down_lim,
                    mon_res_pos_left_lim,
                    mon_res_pos_left_up_lim,
                    &mut mon_res_pos_up,
                    &mut mon_res_pos_right_up,
                    &mut mon_res_pos_right,
                    &mut mon_res_pos_right_down,
                    &mut mon_res_pos_down,
                    &mut mon_res_pos_left_down,
                    &mut mon_res_pos_left,
                    &mut mon_res_pos_left_up,
                    resolution,
                    primary_monitor_positioned,
                )
            });

        positions.append(&mut mon_res_pos_up);
        positions.append(&mut mon_res_pos_right_up);
        positions.append(&mut mon_res_pos_right);
        positions.append(&mut mon_res_pos_right_down);
        positions.append(&mut mon_res_pos_down);
        positions.append(&mut mon_res_pos_left_down);
        positions.append(&mut mon_res_pos_left);
        positions.append(&mut mon_res_pos_left_up);

        if has_an_internal_display {
            positions.shuffle(&mut self.rand);
            positions.insert(0, primary_monitor_positioned);
        } else {
            positions.push(primary_monitor_positioned);
            positions.shuffle(&mut self.rand);
        }

        assert_eq!(
            positions.len(),
            resolutions.len(),
            "Not all resolutions have been positioned"
        );

        positions
    }

    fn position_resolution(
        resolution_index: usize,
        mon_res_pos_up_lim: usize,
        mon_res_pos_right_up_lim: usize,
        mon_res_pos_right_lim: usize,
        mon_res_pos_right_down_lim: usize,
        mon_res_pos_down_lim: usize,
        mon_res_pos_left_down_lim: usize,
        mon_res_pos_left_lim: usize,
        mon_res_pos_left_up_lim: usize,
        mon_res_pos_up: &mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_right_up: &mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_right: &mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_right_down: &mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_down: &mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_left_down: &mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_left: &mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_left_up: &mut Vec<FuzzedMonitorPositionedResolution>,
        resolution: &FuzzedResolution,
        primary_monitor_positioned: FuzzedMonitorPositionedResolution,
    ) {
        let position_resolution_parameters = Self::get_position_resolution_parameters(
            resolution_index,
            mon_res_pos_up_lim,
            mon_res_pos_right_up_lim,
            mon_res_pos_right_lim,
            mon_res_pos_right_down_lim,
            mon_res_pos_down_lim,
            mon_res_pos_left_down_lim,
            mon_res_pos_left_lim,
            mon_res_pos_left_up_lim,
            mon_res_pos_up,
            mon_res_pos_right_up,
            mon_res_pos_right,
            mon_res_pos_right_down,
            mon_res_pos_down,
            mon_res_pos_left_down,
            mon_res_pos_left,
            mon_res_pos_left_up,
        );

        Self::position_resolution_with_parameters(
            resolution,
            position_resolution_parameters.0,
            &primary_monitor_positioned,
            position_resolution_parameters.1,
            position_resolution_parameters.2,
        );
    }

    fn position_resolution_with_parameters(
        resolution: &FuzzedResolution,
        mon_pos_res: &mut Vec<FuzzedMonitorPositionedResolution>,
        primary_monitor_positioned: &FuzzedMonitorPositionedResolution,
        move_horizontally: MoveHorizontally,
        move_vertically: MoveVertically,
    ) {
        let previous_resolution_position =
            mon_pos_res.last().unwrap_or(&primary_monitor_positioned);

        let x = match move_horizontally {
            MoveHorizontally::None => 0,
            MoveHorizontally::Left => {
                previous_resolution_position.position.x + i32::try_from(resolution.width).unwrap()
            }
            MoveHorizontally::Right => {
                previous_resolution_position.position.x - i32::try_from(resolution.width).unwrap()
            }
        };

        let y = match move_vertically {
            MoveVertically::None => 0,
            MoveVertically::Up => {
                previous_resolution_position.position.y + i32::try_from(resolution.height).unwrap()
            }
            MoveVertically::Down => {
                previous_resolution_position.position.y - i32::try_from(resolution.height).unwrap()
            }
        };

        mon_pos_res.push(FuzzedMonitorPositionedResolution {
            resolution: *resolution,
            position: FuzzedMonitorPosition { x, y },
        });
    }

    fn get_position_resolution_parameters<'a>(
        resolution_index: usize,
        mon_res_pos_up_lim: usize,
        mon_res_pos_right_up_lim: usize,
        mon_res_pos_right_lim: usize,
        mon_res_pos_right_down_lim: usize,
        mon_res_pos_down_lim: usize,
        mon_res_pos_left_down_lim: usize,
        mon_res_pos_left_lim: usize,
        mon_res_pos_left_up_lim: usize,
        mon_res_pos_up: &'a mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_right_up: &'a mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_right: &'a mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_right_down: &'a mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_down: &'a mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_left_down: &'a mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_left: &'a mut Vec<FuzzedMonitorPositionedResolution>,
        mon_res_pos_left_up: &'a mut Vec<FuzzedMonitorPositionedResolution>,
    ) -> (
        &'a mut Vec<FuzzedMonitorPositionedResolution>,
        MoveHorizontally,
        MoveVertically,
    ) {
        match resolution_index {
            resolution_index if resolution_index < mon_res_pos_up_lim => {
                (mon_res_pos_up, MoveHorizontally::None, MoveVertically::Up)
            }
            resolution_index if resolution_index < mon_res_pos_right_up_lim => (
                mon_res_pos_right_up,
                MoveHorizontally::Right,
                MoveVertically::Up,
            ),
            resolution_index if resolution_index < mon_res_pos_right_lim => (
                mon_res_pos_right,
                MoveHorizontally::Right,
                MoveVertically::None,
            ),
            resolution_index if resolution_index < mon_res_pos_right_down_lim => (
                mon_res_pos_right_down,
                MoveHorizontally::Right,
                MoveVertically::Down,
            ),
            resolution_index if resolution_index < mon_res_pos_down_lim => (
                mon_res_pos_down,
                MoveHorizontally::None,
                MoveVertically::Down,
            ),
            resolution_index if resolution_index < mon_res_pos_left_down_lim => (
                mon_res_pos_left_down,
                MoveHorizontally::Left,
                MoveVertically::None,
            ),
            resolution_index if resolution_index < mon_res_pos_left_lim => (
                mon_res_pos_left,
                MoveHorizontally::Left,
                MoveVertically::None,
            ),
            resolution_index if resolution_index < mon_res_pos_left_up_lim => (
                mon_res_pos_left_up,
                MoveHorizontally::Left,
                MoveVertically::Up,
            ),
            _ => panic!("Positioning resolutions failed, index went out of the expected value"),
        }
    }
}

enum MoveHorizontally {
    None,
    Left,
    Right,
}

enum MoveVertically {
    None,
    Up,
    Down,
}
