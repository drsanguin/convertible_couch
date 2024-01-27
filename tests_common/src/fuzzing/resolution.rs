use std::fmt::Display;

use rand::{rngs::StdRng, seq::SliceRandom};

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
    pub const XGA: FuzzedResolution = FuzzedResolution {
        width: 1024,
        height: 768,
    };
    pub const SXGA: FuzzedResolution = FuzzedResolution {
        width: 1280,
        height: 1024,
    };
    pub const WXGA_HD: FuzzedResolution = FuzzedResolution {
        width: 1366,
        height: 768,
    };
    pub const WXGA_PLUS: FuzzedResolution = FuzzedResolution {
        width: 1440,
        height: 900,
    };
    pub const HD_PLUS: FuzzedResolution = FuzzedResolution {
        width: 1600,
        height: 900,
    };
    pub const UXGA: FuzzedResolution = FuzzedResolution {
        width: 1600,
        height: 1200,
    };
    pub const WSXGA_PLUS: FuzzedResolution = FuzzedResolution {
        width: 1680,
        height: 1050,
    };
    pub const FULL_HD: FuzzedResolution = FuzzedResolution {
        width: 1920,
        height: 1080,
    };
    pub const WUXGA: FuzzedResolution = FuzzedResolution {
        width: 1920,
        height: 1200,
    };
    pub const UW_UXGA: FuzzedResolution = FuzzedResolution {
        width: 2560,
        height: 1080,
    };
    pub const WQHD: FuzzedResolution = FuzzedResolution {
        width: 2560,
        height: 1440,
    };
    pub const WQXGA: FuzzedResolution = FuzzedResolution {
        width: 2560,
        height: 1600,
    };
    pub const SDQHD: FuzzedResolution = FuzzedResolution {
        width: 2560,
        height: 2880,
    };
    pub const _3440_1440: FuzzedResolution = FuzzedResolution {
        width: 3440,
        height: 1440,
    };
    pub const _3840_1080: FuzzedResolution = FuzzedResolution {
        width: 3840,
        height: 1080,
    };
    pub const _3840_1200: FuzzedResolution = FuzzedResolution {
        width: 3840,
        height: 1200,
    };
    pub const _3840_1600: FuzzedResolution = FuzzedResolution {
        width: 3840,
        height: 1600,
    };
    pub const UHD_4K: FuzzedResolution = FuzzedResolution {
        width: 3840,
        height: 2160,
    };
    pub const WQUXGA: FuzzedResolution = FuzzedResolution {
        width: 3840,
        height: 2400,
    };
    pub const DCI_4K: FuzzedResolution = FuzzedResolution {
        width: 4096,
        height: 2160,
    };
    pub const _5120_1440: FuzzedResolution = FuzzedResolution {
        width: 5120,
        height: 1440,
    };
    pub const _5K_5120_2160: FuzzedResolution = FuzzedResolution {
        width: 5120,
        height: 2160,
    };
    pub const _5K_5120_2880: FuzzedResolution = FuzzedResolution {
        width: 5120,
        height: 2880,
    };
    pub const _6016_3384: FuzzedResolution = FuzzedResolution {
        width: 6016,
        height: 3384,
    };
    pub const _6K: FuzzedResolution = FuzzedResolution {
        width: 6144,
        height: 3456,
    };
    pub const UHD_8K: FuzzedResolution = FuzzedResolution {
        width: 7680,
        height: 4320,
    };
    pub const ALL: [FuzzedResolution; 26] = [
        FuzzedResolution::XGA,
        FuzzedResolution::SXGA,
        FuzzedResolution::WXGA_HD,
        FuzzedResolution::WXGA_PLUS,
        FuzzedResolution::HD_PLUS,
        FuzzedResolution::UXGA,
        FuzzedResolution::WSXGA_PLUS,
        FuzzedResolution::FULL_HD,
        FuzzedResolution::WUXGA,
        FuzzedResolution::UW_UXGA,
        FuzzedResolution::WQHD,
        FuzzedResolution::WQXGA,
        FuzzedResolution::SDQHD,
        FuzzedResolution::_3440_1440,
        FuzzedResolution::_3840_1080,
        FuzzedResolution::_3840_1200,
        FuzzedResolution::_3840_1600,
        FuzzedResolution::UHD_4K,
        FuzzedResolution::WQUXGA,
        FuzzedResolution::DCI_4K,
        FuzzedResolution::_5120_1440,
        FuzzedResolution::_5K_5120_2160,
        FuzzedResolution::_5K_5120_2880,
        FuzzedResolution::_6016_3384,
        FuzzedResolution::_6K,
        FuzzedResolution::UHD_8K,
    ];
}

pub struct ResolutionFuzzer {
    rand: StdRng,
}

impl ResolutionFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> FuzzedResolution {
        FuzzedResolution::ALL
            .choose(&mut self.rand)
            .unwrap()
            .clone()
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<FuzzedResolution> {
        (0..count).map(|_| self.generate_one()).collect()
    }
}
