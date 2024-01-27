use std::collections::HashSet;

use rand::{
    distributions::{Alphanumeric, DistString},
    rngs::StdRng,
    seq::SliceRandom,
    Rng,
};

pub struct FuzzedMonitorBrand(String);

impl FuzzedMonitorBrand {
    pub const ACER: &'static str = "ACER";
    pub const AG_NEOVO: &'static str = "AG NEOVO";
    pub const ALIENWARE: &'static str = "ALIENWARE";
    pub const AOC: &'static str = "AOC";
    pub const AORUS: &'static str = "AORUS";
    pub const APPLE: &'static str = "APPLE";
    pub const APPROX: &'static str = "APPROX";
    pub const ASROCK: &'static str = "ASROCK";
    pub const ASUS: &'static str = "ASUS";
    pub const BENQ: &'static str = "BENQ";
    pub const CONTINENTAL_EDISON: &'static str = "CONTINENTAL EDISON";
    pub const COOLER_MASTER: &'static str = "COOLER MASTER";
    pub const CORSAIR: &'static str = "CORSAIR";
    pub const DAHUA: &'static str = "DAHUA";
    pub const DELL: &'static str = "DELL";
    pub const EIZO: &'static str = "EIZO";
    pub const ELO: &'static str = "ELO";
    pub const ESSENTIELB: &'static str = "ESSENTIELB";
    pub const FAYTECH: &'static str = "FAYTECH";
    pub const FOX_SPIRIT: &'static str = "FOX SPIRIT";
    pub const FUJITSU: &'static str = "FUJITSU";
    pub const GIGABYTE: &'static str = "GIGABYTE";
    pub const GIGACRYSTA: &'static str = "GIGACRYSTA";
    pub const HANNSG: &'static str = "HANNSG";
    pub const HANNSPREE: &'static str = "HANNSPREE";
    pub const HEWLETT_PACKARD_HP: &'static str = "HEWLETT PACKARD HP";
    pub const HUAWEI: &'static str = "HUAWEI";
    pub const HYPERX: &'static str = "HYPERX";
    pub const IGGUAL: &'static str = "IGGUAL";
    pub const IIYAMA: &'static str = "IIYAMA";
    pub const INOVU: &'static str = "INOVU";
    pub const ITEK: &'static str = "ITEK";
    pub const JOY_IT: &'static str = "JOY-IT";
    pub const KEEP_OUT: &'static str = "KEEP OUT";
    pub const KONIX: &'static str = "KONIX";
    pub const KOORUI: &'static str = "KOORUI";
    pub const LC_POWER: &'static str = "LC-POWER";
    pub const LENOVO: &'static str = "LENOVO";
    pub const LG: &'static str = "LG";
    pub const MEDION: &'static str = "MEDION";
    pub const MILLENIUM: &'static str = "MILLENIUM";
    pub const MONOPRICE: &'static str = "MONOPRICE";
    pub const MSI: &'static str = "MSI";
    pub const NEC: &'static str = "NEC";
    pub const NEWLINE: &'static str = "NEWLINE";
    pub const NEWSKILL: &'static str = "NEWSKILL";
    pub const NILOX: &'static str = "NILOX";
    pub const OVERSTEEL: &'static str = "OVERSTEEL";
    pub const OZONE: &'static str = "OZONE";
    pub const PHILIPS: &'static str = "PHILIPS";
    pub const RAZER: &'static str = "RAZER";
    pub const SAMSUNG: &'static str = "SAMSUNG";
    pub const SCHNEIDER: &'static str = "SCHNEIDER";
    pub const SHARKGAMING: &'static str = "SHARKGAMING";
    pub const SKILLKORP: &'static str = "SKILLKORP";
    pub const SMART_TECH: &'static str = "SMART-TECH";
    pub const SONY: &'static str = "SONY";
    pub const TARGUS: &'static str = "TARGUS";
    pub const THOMSON: &'static str = "THOMSON";
    pub const VIDEOSEVEN_V7: &'static str = "VIDEOSEVEN V7";
    pub const VIEWSONIC: &'static str = "VIEWSONIC";
    pub const WORTMANN: &'static str = "WORTMANN";
    pub const XIAOMI: &'static str = "XIAOMI";
    pub const YASHI: &'static str = "YASHI";
    pub const ALL: [&'static str; 64] = [
        FuzzedMonitorBrand::ACER,
        FuzzedMonitorBrand::AG_NEOVO,
        FuzzedMonitorBrand::ALIENWARE,
        FuzzedMonitorBrand::AOC,
        FuzzedMonitorBrand::AORUS,
        FuzzedMonitorBrand::APPLE,
        FuzzedMonitorBrand::APPROX,
        FuzzedMonitorBrand::ASROCK,
        FuzzedMonitorBrand::ASUS,
        FuzzedMonitorBrand::BENQ,
        FuzzedMonitorBrand::CONTINENTAL_EDISON,
        FuzzedMonitorBrand::COOLER_MASTER,
        FuzzedMonitorBrand::CORSAIR,
        FuzzedMonitorBrand::DAHUA,
        FuzzedMonitorBrand::DELL,
        FuzzedMonitorBrand::EIZO,
        FuzzedMonitorBrand::ELO,
        FuzzedMonitorBrand::ESSENTIELB,
        FuzzedMonitorBrand::FAYTECH,
        FuzzedMonitorBrand::FOX_SPIRIT,
        FuzzedMonitorBrand::FUJITSU,
        FuzzedMonitorBrand::GIGABYTE,
        FuzzedMonitorBrand::GIGACRYSTA,
        FuzzedMonitorBrand::HANNSG,
        FuzzedMonitorBrand::HANNSPREE,
        FuzzedMonitorBrand::HEWLETT_PACKARD_HP,
        FuzzedMonitorBrand::HUAWEI,
        FuzzedMonitorBrand::HYPERX,
        FuzzedMonitorBrand::IGGUAL,
        FuzzedMonitorBrand::IIYAMA,
        FuzzedMonitorBrand::INOVU,
        FuzzedMonitorBrand::ITEK,
        FuzzedMonitorBrand::JOY_IT,
        FuzzedMonitorBrand::KEEP_OUT,
        FuzzedMonitorBrand::KONIX,
        FuzzedMonitorBrand::KOORUI,
        FuzzedMonitorBrand::LC_POWER,
        FuzzedMonitorBrand::LENOVO,
        FuzzedMonitorBrand::LG,
        FuzzedMonitorBrand::MEDION,
        FuzzedMonitorBrand::MILLENIUM,
        FuzzedMonitorBrand::MONOPRICE,
        FuzzedMonitorBrand::MSI,
        FuzzedMonitorBrand::NEC,
        FuzzedMonitorBrand::NEWLINE,
        FuzzedMonitorBrand::NEWSKILL,
        FuzzedMonitorBrand::NILOX,
        FuzzedMonitorBrand::OVERSTEEL,
        FuzzedMonitorBrand::OZONE,
        FuzzedMonitorBrand::PHILIPS,
        FuzzedMonitorBrand::RAZER,
        FuzzedMonitorBrand::SAMSUNG,
        FuzzedMonitorBrand::SCHNEIDER,
        FuzzedMonitorBrand::SHARKGAMING,
        FuzzedMonitorBrand::SKILLKORP,
        FuzzedMonitorBrand::SMART_TECH,
        FuzzedMonitorBrand::SONY,
        FuzzedMonitorBrand::TARGUS,
        FuzzedMonitorBrand::THOMSON,
        FuzzedMonitorBrand::VIDEOSEVEN_V7,
        FuzzedMonitorBrand::VIEWSONIC,
        FuzzedMonitorBrand::WORTMANN,
        FuzzedMonitorBrand::XIAOMI,
        FuzzedMonitorBrand::YASHI,
    ];
}

pub struct MonitorNameFuzzer {
    rand: StdRng,
}

impl MonitorNameFuzzer {
    pub fn new(rand: StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> String {
        let brand = FuzzedMonitorBrand::ALL.choose(&mut self.rand).unwrap();
        let model_id_max_len = 62 - brand.len();
        let model_id_len = self.rand.gen_range(8..model_id_max_len);
        let model_id_part_1 = Alphanumeric.sample_string(&mut self.rand, model_id_len);

        format!("{brand} {model_id_part_1}")
    }

    pub fn generate_two(&mut self) -> (String, String) {
        let several = self.generate_several(2);

        (several[0].clone(), several[1].clone())
    }

    pub fn generate_several(&mut self, count: usize) -> Vec<String> {
        let mut names = HashSet::with_capacity(count);

        while names.len() != count {
            let name = self.generate_one();
            names.insert(name);
        }

        Vec::from_iter(names)
    }
}
