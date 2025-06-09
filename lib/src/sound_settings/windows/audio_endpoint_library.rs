use std::os::raw::{c_int, c_ushort};

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

#[link(
    name = ".\\AudioEndPointLibrary\\AudioEndPointLibrary\\bin\\x64\\Release\\AudioEndPointLibrary"
)]
unsafe extern "C" {
    unsafe fn get_all_audio_endpoints_count() -> c_int;

    unsafe fn get_all_audio_endpoints(
        out_audio_endpoints: *mut AudioEndpoint,
        audio_endpoints_count: c_int,
    ) -> c_int;

    unsafe fn set_default_audio_endpoint(id: *mut c_ushort) -> c_int;
}

pub struct DllBasedAudioEndpointLibrary;

impl AudioEndpointLibrary for DllBasedAudioEndpointLibrary {
    fn get_all_audio_endpoints_count(&self) -> c_int {
        unsafe { get_all_audio_endpoints_count() }
    }

    fn get_all_audio_endpoints(
        &self,
        out_audio_endpoints: *mut AudioEndpoint,
        audio_endpoints_count: c_int,
    ) -> c_int {
        unsafe { get_all_audio_endpoints(out_audio_endpoints, audio_endpoints_count) }
    }

    fn set_default_audio_endpoint(&mut self, id: *mut c_ushort) -> c_int {
        unsafe { set_default_audio_endpoint(id) }
    }
}
