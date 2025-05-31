use std::fmt::Display;

use rand::{rngs::StdRng, seq::SliceRandom, Rng};

use super::resolution::FuzzedResolution;

#[derive(Clone, Copy)]
pub struct FuzzedDisplayPosition {
    pub x: i32,
    pub y: i32,
}

impl FuzzedDisplayPosition {
    pub fn is_positioned_at_origin(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

#[derive(Clone, Copy)]
pub struct FuzzedDisplayPositionedResolution {
    pub resolution: FuzzedResolution,
    pub position: FuzzedDisplayPosition,
}

impl Display for FuzzedDisplayPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct DisplayPositionFuzzer {
    rand: StdRng,
}

impl DisplayPositionFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_several(
        &mut self,
        resolutions: &Vec<FuzzedResolution>,
        has_an_internal_display: bool,
    ) -> Vec<FuzzedDisplayPositionedResolution> {
        let n_display = resolutions.len();
        let primary_display_index = if has_an_internal_display {
            0
        } else {
            self.rand.random_range(0..n_display)
        };
        let primary_display_resolution = resolutions[primary_display_index];
        let primary_display_position = FuzzedDisplayPosition { x: 0, y: 0 };
        let primary_display_positioned = FuzzedDisplayPositionedResolution {
            position: primary_display_position,
            resolution: primary_display_resolution,
        };

        let mut displays_positions_by_axis = DisplaysPositionsByAxis::new(n_display);

        resolutions.iter().enumerate().for_each(|(i, resolution)| {
            Self::position_resolution(
                i,
                primary_display_index,
                resolution,
                &mut displays_positions_by_axis,
                primary_display_positioned,
            )
        });

        let mut positions = Vec::with_capacity(n_display);

        positions.append(&mut displays_positions_by_axis.up);
        positions.append(&mut displays_positions_by_axis.right_up);
        positions.append(&mut displays_positions_by_axis.right);
        positions.append(&mut displays_positions_by_axis.right_down);
        positions.append(&mut displays_positions_by_axis.down);
        positions.append(&mut displays_positions_by_axis.left_down);
        positions.append(&mut displays_positions_by_axis.left);
        positions.append(&mut displays_positions_by_axis.left_up);

        if has_an_internal_display {
            positions.shuffle(&mut self.rand);
            positions.insert(0, primary_display_positioned);
        } else {
            positions.push(primary_display_positioned);
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
        primary_display_index: usize,
        resolution: &FuzzedResolution,
        displays_positions_by_axis: &mut DisplaysPositionsByAxis,
        primary_display_positioned: FuzzedDisplayPositionedResolution,
    ) {
        match i {
            i if i == primary_display_index => {}
            i if i & 7 == 7 => Self::move_resolution(
                resolution,
                &mut displays_positions_by_axis.left_up,
                &primary_display_positioned,
                HorizontalMove::Left,
                VerticalMove::Up,
            ),
            i if i & 7 == 6 => Self::move_resolution(
                resolution,
                &mut displays_positions_by_axis.left,
                &primary_display_positioned,
                HorizontalMove::Left,
                VerticalMove::None,
            ),
            i if i & 7 == 5 => Self::move_resolution(
                resolution,
                &mut displays_positions_by_axis.left_down,
                &primary_display_positioned,
                HorizontalMove::Left,
                VerticalMove::Down,
            ),
            i if i & 7 == 4 => Self::move_resolution(
                resolution,
                &mut displays_positions_by_axis.down,
                &primary_display_positioned,
                HorizontalMove::None,
                VerticalMove::Down,
            ),
            i if i & 7 == 3 => Self::move_resolution(
                resolution,
                &mut displays_positions_by_axis.right_down,
                &primary_display_positioned,
                HorizontalMove::Right,
                VerticalMove::Down,
            ),
            i if i & 7 == 2 => Self::move_resolution(
                resolution,
                &mut displays_positions_by_axis.right,
                &primary_display_positioned,
                HorizontalMove::Right,
                VerticalMove::None,
            ),
            i if i & 7 == 1 => Self::move_resolution(
                resolution,
                &mut displays_positions_by_axis.right_up,
                &primary_display_positioned,
                HorizontalMove::Right,
                VerticalMove::Up,
            ),
            _ => Self::move_resolution(
                resolution,
                &mut displays_positions_by_axis.up,
                &primary_display_positioned,
                HorizontalMove::None,
                VerticalMove::Up,
            ),
        }
    }

    fn move_resolution(
        resolution: &FuzzedResolution,
        axis_displays_positions: &mut Vec<FuzzedDisplayPositionedResolution>,
        primary_display_positioned: &FuzzedDisplayPositionedResolution,
        horizontal_move: HorizontalMove,
        vertical_move: VerticalMove,
    ) {
        let previous_resolution_position = axis_displays_positions
            .last()
            .unwrap_or(&primary_display_positioned);

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

        axis_displays_positions.push(FuzzedDisplayPositionedResolution {
            resolution: *resolution,
            position: FuzzedDisplayPosition { x, y },
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

struct DisplaysPositionsByAxis {
    pub up: Vec<FuzzedDisplayPositionedResolution>,
    pub right_up: Vec<FuzzedDisplayPositionedResolution>,
    pub right: Vec<FuzzedDisplayPositionedResolution>,
    pub right_down: Vec<FuzzedDisplayPositionedResolution>,
    pub down: Vec<FuzzedDisplayPositionedResolution>,
    pub left_down: Vec<FuzzedDisplayPositionedResolution>,
    pub left: Vec<FuzzedDisplayPositionedResolution>,
    pub left_up: Vec<FuzzedDisplayPositionedResolution>,
}

impl DisplaysPositionsByAxis {
    fn new(n_display: usize) -> Self {
        let n_display_by_axis = (n_display / 8) + 1;

        Self {
            up: Vec::with_capacity(n_display_by_axis),
            right_up: Vec::with_capacity(n_display_by_axis),
            right: Vec::with_capacity(n_display_by_axis),
            right_down: Vec::with_capacity(n_display_by_axis),
            down: Vec::with_capacity(n_display_by_axis),
            left_down: Vec::with_capacity(n_display_by_axis),
            left: Vec::with_capacity(n_display_by_axis),
            left_up: Vec::with_capacity(n_display_by_axis),
        }
    }
}
