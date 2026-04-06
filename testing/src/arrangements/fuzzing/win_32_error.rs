use rand::{rngs::StdRng, seq::IndexedRandom};
use windows::Win32::Foundation::{
    ERROR_ACCESS_DENIED, ERROR_BAD_CONFIGURATION, ERROR_GEN_FAILURE, ERROR_INSUFFICIENT_BUFFER,
    ERROR_INVALID_PARAMETER, ERROR_NOT_SUPPORTED, WIN32_ERROR,
};

pub struct Win32ErrorFuzzer<'a> {
    rand: &'a mut StdRng,
}

impl<'a> Win32ErrorFuzzer<'a> {
    const ERRORS: [WIN32_ERROR; 6] = [
        ERROR_INVALID_PARAMETER,
        ERROR_NOT_SUPPORTED,
        ERROR_ACCESS_DENIED,
        ERROR_GEN_FAILURE,
        ERROR_INSUFFICIENT_BUFFER,
        ERROR_BAD_CONFIGURATION,
    ];

    pub fn new(rand: &'a mut StdRng) -> Self {
        Self { rand }
    }

    pub fn generate_one(&mut self, forbidden_errors: &[WIN32_ERROR]) -> WIN32_ERROR {
        loop {
            let error = Self::ERRORS.choose(self.rand).unwrap().to_owned();

            if forbidden_errors.contains(&error) {
                continue;
            }

            return error;
        }
    }
}
