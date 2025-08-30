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

    /// # Safety
    /// This function is unsafe because it writes into a raw pointer:
    /// - `out_audio_endpoints` must be non-null and point to valid, writable memory
    ///   large enough to hold at least `audio_endpoints_count` elements of type
    ///   [`AudioEndpoint`].
    /// - The memory must be properly aligned for [`AudioEndpoint`].
    /// - The caller must ensure that `audio_endpoints_count` accurately reflects the
    ///   size of the allocated buffer. Supplying a smaller buffer causes undefined
    ///   behavior (memory corruption, crashes).
    /// - The buffer must remain valid for the entire duration of the call.
    /// - Returned values are only valid if the function indicates success.
    ///
    /// The caller is responsible for ensuring these conditions are upheld.
    unsafe fn get_all_audio_endpoints(
        &self,
        out_audio_endpoints: *mut AudioEndpoint,
        audio_endpoints_count: c_int,
    ) -> c_int;

    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer:
    /// - `id` must be non-null and point to a valid, readable, properly aligned
    ///   UTF-16 (wide-character) null-terminated string.
    /// - The pointed-to string must remain valid for the duration of the call.
    /// - The caller must ensure the string is correctly null-terminated; otherwise,
    ///   the function may read past allocated memory, causing undefined behavior
    ///   (memory corruption, crashes).
    /// - The pointer must not be mutated by another thread while the function is
    ///   executing, to avoid race conditions.
    /// - Passing an invalid or incorrectly formed string leads to undefined
    ///   behavior.
    ///
    /// The caller is responsible for ensuring all of these conditions are upheld.
    unsafe fn set_default_audio_endpoint(&mut self, id: *mut c_ushort) -> c_int;
}
