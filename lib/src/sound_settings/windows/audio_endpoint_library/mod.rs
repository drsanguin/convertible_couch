use std::os::raw::{c_int, c_ushort};

pub mod dll_based_audio_endpoint_library;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioEndpoint {
    pub id: *mut c_ushort,
    pub name: *mut c_ushort,
    pub is_default: c_int,
}

impl Default for AudioEndpoint {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

pub trait AudioEndpointLibrary {
    fn get_all_audio_endpoints_count(&self) -> c_int;

    fn get_all_audio_endpoints(
        &self,
        out_audio_endpoints: *mut AudioEndpoint,
        audio_endpoints_count: c_int,
    ) -> c_int;

    fn set_default_audio_endpoint(&mut self, id: *mut c_ushort) -> c_int;
}
