use windows::{
    core::{BOOL, PCWSTR},
    Win32::{
        Devices::Display::{
            DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
            DISPLAYCONFIG_TOPOLOGY_ID, QUERY_DISPLAY_CONFIG_FLAGS,
        },
        Foundation::{HWND, WIN32_ERROR},
        Graphics::Gdi::{
            CDS_TYPE, DEVMODEW, DISPLAY_DEVICEW, DISP_CHANGE, ENUM_DISPLAY_SETTINGS_MODE,
        },
    },
};

pub mod windows_api_based_win_32;

pub trait Win32 {
    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer:
    /// - `requestpacket` must be non-null and point to valid, writable memory.
    /// - The memory referenced by `requestpacket` must remain valid for the duration
    ///   of the call.
    /// - The caller must ensure that `requestpacket` is correctly sized and aligned
    ///   for a [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure and any expected
    ///   subtype it represents.
    /// - Passing an invalid or incorrectly initialized pointer leads to undefined
    ///   behavior (e.g., crashes, memory corruption).
    ///
    /// The caller is responsible for ensuring all of these conditions are upheld.
    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32;

    /// # Safety
    /// - `numpatharrayelements` and `nummodeinfoarrayelements` must be non-null and
    ///   point to valid, writable `u32` memory locations.
    /// - The caller must ensure these pointers remain valid and properly aligned
    ///   for the duration of the call.
    /// - Passing null or invalid pointers results in undefined behavior.
    unsafe fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> WIN32_ERROR;

    /// # Safety
    /// - All pointer arguments (`numpatharrayelements`, `patharray`,
    ///   `nummodeinfoarrayelements`, `modeinfoarray`, and optionally
    ///   `currenttopologyid`) must be valid, properly aligned, and point to
    ///   sufficiently large writable memory.
    /// - The caller must correctly initialize the input sizes before the call.
    /// - Buffers must remain valid for the duration of the call.
    /// - Passing invalid or undersized buffers leads to undefined behavior.
    unsafe fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: ::core::option::Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> WIN32_ERROR;

    /// # Safety
    /// - `lpszdevicename` must be a valid, null-terminated UTF-16 string (if not null).
    /// - `lpdevmode` must point to a valid, readable [`DEVMODEW`] structure (if provided).
    /// - `lparam` must point to valid, properly aligned memory if used.
    /// - All pointers must remain valid for the duration of the call.
    /// - Passing invalid or incorrectly initialized pointers results in undefined behavior.
    unsafe fn change_display_settings_ex_w(
        &mut self,
        lpszdevicename: PCWSTR,
        lpdevmode: ::core::option::Option<*const DEVMODEW>,
        hwnd: Option<HWND>,
        dwflags: CDS_TYPE,
        lparam: ::core::option::Option<*const ::core::ffi::c_void>,
    ) -> DISP_CHANGE;

    /// # Safety
    /// - `lpdevice`, if non-null, must be a valid, null-terminated UTF-16 string.
    /// - `lpdisplaydevice` must be non-null, valid, and point to a writable
    ///   [`DISPLAY_DEVICEW`] structure of sufficient size. The `cb` field of the
    ///   structure must be set correctly by the caller before the call.
    /// - All pointers must remain valid and properly aligned during the call.
    /// - Passing invalid or incorrectly sized memory leads to undefined behavior.
    unsafe fn enum_display_devices_w(
        &self,
        lpdevice: PCWSTR,
        idevnum: u32,
        lpdisplaydevice: *mut DISPLAY_DEVICEW,
        dwflags: u32,
    ) -> BOOL;

    /// # Safety
    /// - `lpszdevicename`, if non-null, must be a valid, null-terminated UTF-16 string.
    /// - `lpdevmode` must be non-null, valid, and point to a writable
    ///   [`DEVMODEW`] structure of sufficient size. The `dmSize` field must be set
    ///   correctly by the caller before the call.
    /// - All pointers must remain valid and properly aligned during the call.
    /// - Passing invalid or incorrectly sized memory leads to undefined behavior.
    unsafe fn enum_display_settings_w(
        &self,
        lpszdevicename: PCWSTR,
        imodenum: ENUM_DISPLAY_SETTINGS_MODE,
        lpdevmode: *mut DEVMODEW,
    ) -> BOOL;
}
