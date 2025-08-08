use std::os::raw::{c_int, c_ushort};

use crate::speakers_settings::windows::audio_endpoint_library::{
    AudioEndpoint, AudioEndpointLibrary,
};

#[link(
    name = ".\\lib\\AudioEndPointLibrary\\AudioEndPointLibrary\\bin\\x64\\Release\\AudioEndPointLibrary"
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
