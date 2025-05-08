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

    pub fn generate_several(
        &mut self,
        resolutions: &Vec<FuzzedResolution>,
        has_an_internal_display: bool,
    ) -> Vec<FuzzedMonitorPositionedResolution> {
        let n_monitor = resolutions.len();
        let primary_monitor_index = if has_an_internal_display {
            0
        } else {
            self.rand.random_range(0..n_monitor)
        };
        let primary_monitor_resolution = resolutions[primary_monitor_index];
        let primary_monitor_position = FuzzedMonitorPosition { x: 0, y: 0 };
        let primary_monitor_positioned = FuzzedMonitorPositionedResolution {
            position: primary_monitor_position,
            resolution: primary_monitor_resolution,
        };

        let mut monitors_positions_by_axis = MonitorsPositionsByAxis::new(n_monitor);

        resolutions.iter().enumerate().for_each(|(i, resolution)| {
            Self::position_resolution(
                i,
                primary_monitor_index,
                resolution,
                &mut monitors_positions_by_axis,
                primary_monitor_positioned,
            )
        });

        let mut positions = Vec::with_capacity(n_monitor);

        positions.append(&mut monitors_positions_by_axis.up);
        positions.append(&mut monitors_positions_by_axis.right_up);
        positions.append(&mut monitors_positions_by_axis.right);
        positions.append(&mut monitors_positions_by_axis.right_down);
        positions.append(&mut monitors_positions_by_axis.down);
        positions.append(&mut monitors_positions_by_axis.left_down);
        positions.append(&mut monitors_positions_by_axis.left);
        positions.append(&mut monitors_positions_by_axis.left_up);

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
        i: usize,
        primary_monitor_index: usize,
        resolution: &FuzzedResolution,
        monitors_positions_by_axis: &mut MonitorsPositionsByAxis,
        primary_monitor_positioned: FuzzedMonitorPositionedResolution,
    ) {
        match i {
            i if i == primary_monitor_index => {}
            i if i & 7 == 7 => Self::move_resolution(
                resolution,
                &mut monitors_positions_by_axis.left_up,
                &primary_monitor_positioned,
                HorizontalMove::Left,
                VerticalMove::Up,
            ),
            i if i & 7 == 6 => Self::move_resolution(
                resolution,
                &mut monitors_positions_by_axis.left,
                &primary_monitor_positioned,
                HorizontalMove::Left,
                VerticalMove::None,
            ),
            i if i & 7 == 5 => Self::move_resolution(
                resolution,
                &mut monitors_positions_by_axis.left_down,
                &primary_monitor_positioned,
                HorizontalMove::Left,
                VerticalMove::Down,
            ),
            i if i & 7 == 4 => Self::move_resolution(
                resolution,
                &mut monitors_positions_by_axis.down,
                &primary_monitor_positioned,
                HorizontalMove::None,
                VerticalMove::Down,
            ),
            i if i & 7 == 3 => Self::move_resolution(
                resolution,
                &mut monitors_positions_by_axis.right_down,
                &primary_monitor_positioned,
                HorizontalMove::Right,
                VerticalMove::Down,
            ),
            i if i & 7 == 2 => Self::move_resolution(
                resolution,
                &mut monitors_positions_by_axis.right,
                &primary_monitor_positioned,
                HorizontalMove::Right,
                VerticalMove::None,
            ),
            i if i & 7 == 1 => Self::move_resolution(
                resolution,
                &mut monitors_positions_by_axis.right_up,
                &primary_monitor_positioned,
                HorizontalMove::Right,
                VerticalMove::Up,
            ),
            _ => Self::move_resolution(
                resolution,
                &mut monitors_positions_by_axis.up,
                &primary_monitor_positioned,
                HorizontalMove::None,
                VerticalMove::Up,
            ),
        }
    }

    fn move_resolution(
        resolution: &FuzzedResolution,
        axis_monitors_positions: &mut Vec<FuzzedMonitorPositionedResolution>,
        primary_monitor_positioned: &FuzzedMonitorPositionedResolution,
        horizontal_move: HorizontalMove,
        vertical_move: VerticalMove,
    ) {
        let previous_resolution_position = axis_monitors_positions
            .last()
            .unwrap_or(&primary_monitor_positioned);

        let x = match horizontal_move {
            HorizontalMove::None => 0,
            HorizontalMove::Left => {
                previous_resolution_position.position.x + i32::try_from(resolution.width).unwrap()
            }
            HorizontalMove::Right => {
                previous_resolution_position.position.x - i32::try_from(resolution.width).unwrap()
            }
        };

        let y = match vertical_move {
            VerticalMove::None => 0,
            VerticalMove::Up => {
                previous_resolution_position.position.y + i32::try_from(resolution.height).unwrap()
            }
            VerticalMove::Down => {
                previous_resolution_position.position.y - i32::try_from(resolution.height).unwrap()
            }
        };

        axis_monitors_positions.push(FuzzedMonitorPositionedResolution {
            resolution: *resolution,
            position: FuzzedMonitorPosition { x, y },
        });
    }
}

enum HorizontalMove {
    None,
    Left,
    Right,
}

enum VerticalMove {
    None,
    Up,
    Down,
}

struct MonitorsPositionsByAxis {
    pub up: Vec<FuzzedMonitorPositionedResolution>,
    pub right_up: Vec<FuzzedMonitorPositionedResolution>,
    pub right: Vec<FuzzedMonitorPositionedResolution>,
    pub right_down: Vec<FuzzedMonitorPositionedResolution>,
    pub down: Vec<FuzzedMonitorPositionedResolution>,
    pub left_down: Vec<FuzzedMonitorPositionedResolution>,
    pub left: Vec<FuzzedMonitorPositionedResolution>,
    pub left_up: Vec<FuzzedMonitorPositionedResolution>,
}

impl MonitorsPositionsByAxis {
    fn new(n_monitor: usize) -> Self {
        let n_monitor_by_axis = (n_monitor / 8) + 1;

        Self {
            up: Vec::with_capacity(n_monitor_by_axis),
            right_up: Vec::with_capacity(n_monitor_by_axis),
            right: Vec::with_capacity(n_monitor_by_axis),
            right_down: Vec::with_capacity(n_monitor_by_axis),
            down: Vec::with_capacity(n_monitor_by_axis),
            left_down: Vec::with_capacity(n_monitor_by_axis),
            left: Vec::with_capacity(n_monitor_by_axis),
            left_up: Vec::with_capacity(n_monitor_by_axis),
        }
    }
}
