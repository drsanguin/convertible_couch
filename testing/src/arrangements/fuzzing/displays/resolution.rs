use std::fmt::Display;

use rand::{rngs::StdRng, seq::IndexedRandom};

#[derive(Clone, Copy)]
pub struct FuzzedResolution {
    pub width: u32,
    pub height: u32,
}

impl Display for FuzzedResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl FuzzedResolution {
    pub const ALL: [FuzzedResolution; 26] = [
        FuzzedResolution {
            width: 1024,
            height: 768,
        },
        FuzzedResolution {
            width: 1280,
            height: 1024,
        },
        FuzzedResolution {
            width: 1366,
            height: 768,
        },
        FuzzedResolution {
            width: 1440,
            height: 900,
        },
        FuzzedResolution {
            width: 1600,
            height: 900,
        },
        FuzzedResolution {
            width: 1600,
            height: 1200,
        },
        FuzzedResolution {
            width: 1680,
            height: 1050,
        },
        FuzzedResolution {
            width: 1920,
            height: 1080,
        },
        FuzzedResolution {
            width: 1920,
            height: 1200,
        },
        FuzzedResolution {
            width: 2560,
            height: 1080,
        },
        FuzzedResolution {
            width: 2560,
            height: 1440,
        },
        FuzzedResolution {
            width: 2560,
            height: 1600,
        },
        FuzzedResolution {
            width: 2560,
            height: 2880,
        },
        FuzzedResolution {
            width: 3440,
            height: 1440,
        },
        FuzzedResolution {
            width: 3840,
            height: 1080,
        },
        FuzzedResolution {
            width: 3840,
            height: 1200,
        },
        FuzzedResolution {
            width: 3840,
            height: 1600,
        },
        FuzzedResolution {
            width: 3840,
            height: 2160,
        },
        FuzzedResolution {
            width: 3840,
            height: 2400,
        },
        FuzzedResolution {
            width: 4096,
            height: 2160,
        },
        FuzzedResolution {
            width: 5120,
            height: 1440,
        },
        FuzzedResolution {
            width: 5120,
            height: 2160,
        },
        FuzzedResolution {
            width: 5120,
            height: 2880,
        },
        FuzzedResolution {
            width: 6016,
            height: 3384,
        },
        FuzzedResolution {
            width: 6144,
            height: 3456,
        },
        FuzzedResolution {
            width: 7680,
            height: 4320,
        },
    ];
}

pub struct ResolutionFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> ResolutionFuzzer<'a> {
    pub fn new(rand: &'a mut StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> FuzzedResolution {
        *FuzzedResolution::ALL.choose(self.rand).unwrap()
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<FuzzedResolution> {
        (0..count).map(|_| self.generate_one()).collect()
    }
}
