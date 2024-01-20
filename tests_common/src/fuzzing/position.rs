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
        let primary_monitor_positioned = FuzzedMonitorPositionedResolution {
            position: primary_monitor_position,
            resolution: primary_monitor_resolution,
        };

        let n_monitor_by_axis = (n_monitor / 8) + 1;

        let mut mon_res_pos_up = Vec::with_capacity(n_monitor_by_axis);
        let mut mon_res_pos_right_up = Vec::with_capacity(n_monitor_by_axis);
        let mut mon_res_pos_right = Vec::with_capacity(n_monitor_by_axis);
        let mut mon_res_pos_right_down = Vec::with_capacity(n_monitor_by_axis);
        let mut mon_res_pos_down = Vec::with_capacity(n_monitor_by_axis);
        let mut mon_res_pos_left_down = Vec::with_capacity(n_monitor_by_axis);
        let mut mon_res_pos_left = Vec::with_capacity(n_monitor_by_axis);
        let mut mon_res_pos_left_up = Vec::with_capacity(n_monitor_by_axis);

        resolutions
            .iter()
            .enumerate()
            .for_each(|(i, resolution)| match i {
                i if i == primary_monitor_index => {}
                i if i & 7 == 7 => Self::position_resolution(
                    resolution,
                    &mut mon_res_pos_left_up,
                    &primary_monitor_positioned,
                    MoveHorizontally::Left,
                    MoveVertically::Up,
                ),
                i if i & 7 == 6 => Self::position_resolution(
                    resolution,
                    &mut mon_res_pos_left,
                    &primary_monitor_positioned,
                    MoveHorizontally::Left,
                    MoveVertically::None,
                ),
                i if i & 7 == 5 => Self::position_resolution(
                    resolution,
                    &mut mon_res_pos_left_down,
                    &primary_monitor_positioned,
                    MoveHorizontally::Left,
                    MoveVertically::Down,
                ),
                i if i & 7 == 4 => Self::position_resolution(
                    resolution,
                    &mut mon_res_pos_down,
                    &primary_monitor_positioned,
                    MoveHorizontally::None,
                    MoveVertically::Down,
                ),
                i if i & 7 == 3 => Self::position_resolution(
                    resolution,
                    &mut mon_res_pos_right_down,
                    &primary_monitor_positioned,
                    MoveHorizontally::Right,
                    MoveVertically::Down,
                ),
                i if i & 7 == 2 => Self::position_resolution(
                    resolution,
                    &mut mon_res_pos_right,
                    &primary_monitor_positioned,
                    MoveHorizontally::Right,
                    MoveVertically::None,
                ),
                i if i & 7 == 1 => Self::position_resolution(
                    resolution,
                    &mut mon_res_pos_right_up,
                    &primary_monitor_positioned,
                    MoveHorizontally::Right,
                    MoveVertically::Up,
                ),
                _ => Self::position_resolution(
                    resolution,
                    &mut mon_res_pos_up,
                    &primary_monitor_positioned,
                    MoveHorizontally::None,
                    MoveVertically::Up,
                ),
            });

        let mut positions = Vec::with_capacity(n_monitor);

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
