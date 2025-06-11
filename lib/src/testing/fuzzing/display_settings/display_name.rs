use std::collections::HashSet;

use rand::{
    distr::{Alphanumeric, SampleString},
    rngs::StdRng,
    seq::IndexedRandom,
    Rng,
};

pub struct DisplayNameFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> DisplayNameFuzzer<'a> {
    const BRANDS: [&'static str; 64] = [
        "ACER",
        "AG NEOVO",
        "ALIENWARE",
        "AOC",
        "AORUS",
        "APPLE",
        "APPROX",
        "ASROCK",
        "ASUS",
        "BENQ",
        "CONTINENTAL EDISON",
        "COOLER MASTER",
        "CORSAIR",
        "DAHUA",
        "DELL",
        "EIZO",
        "ELO",
        "ESSENTIELB",
        "FAYTECH",
        "FOX SPIRIT",
        "FUJITSU",
        "GIGABYTE",
        "GIGACRYSTA",
        "HANNSG",
        "HANNSPREE",
        "HEWLETT PACKARD HP",
        "HUAWEI",
        "HYPERX",
        "IGGUAL",
        "IIYAMA",
        "INOVU",
        "ITEK",
        "JOY-IT",
        "KEEP OUT",
        "KONIX",
        "KOORUI",
        "LC-POWER",
        "LENOVO",
        "LG",
        "MEDION",
        "MILLENIUM",
        "MONOPRICE",
        "MSI",
        "NEC",
        "NEWLINE",
        "NEWSKILL",
        "NILOX",
        "OVERSTEEL",
        "OZONE",
        "PHILIPS",
        "RAZER",
        "SAMSUNG",
        "SCHNEIDER",
        "SHARKGAMING",
        "SKILLKORP",
        "SMART-TECH",
        "SONY",
        "TARGUS",
        "THOMSON",
        "VIDEOSEVEN V7",
        "VIEWSONIC",
        "WORTMANN",
        "XIAOMI",
        "YASHI",
    ];

    pub fn new(rand: &'a mut StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self) -> String {
        let brand = Self::BRANDS.choose(self.rand).unwrap();
        let model_id_max_len = 62 - brand.len();
        let model_id_len = self.rand.random_range(8..model_id_max_len);
        let model_id_part_1 = Alphanumeric.sample_string(self.rand, model_id_len);

        format!("{brand} {model_id_part_1}")
    }

    pub fn generate_two(&mut self) -> (String, String) {
        let mut several = self.generate_several(2, &HashSet::new());

        (several.remove(0), several.remove(0))
    }

    pub fn generate_several(
        &mut self,
        count: usize,
        forbidden_display_names: &HashSet<&str>,
    ) -> Vec<String> {
        let mut names = HashSet::with_capacity(count);

        while names.len() != count {
            let name = self.generate_one();

            if forbidden_display_names.contains(&name.as_str()) {
                continue;
            }

            names.insert(name);
        }

        Vec::from_iter(names)
    }
}
