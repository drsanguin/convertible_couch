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
    /// Retrieves information about a display configuration device.
    ///
    /// This is a thin wrapper around the underlying Win32 API and performs no
    /// validation of pointers or memory.
    ///
    /// # Safety
    /// - `requestpacket` must be a **valid, non-null, properly aligned** pointer to a
    ///   `DISPLAYCONFIG_DEVICE_INFO_HEADER`.
    /// - The memory pointed to by `requestpacket` must be **fully initialized** and
    ///   writable for the entire duration of the call.
    /// - The pointed structure must have its `size` and `type` fields correctly set
    ///   according to the Win32 API contract.
    /// - The memory must not be aliased mutably elsewhere while this function executes.
    /// - The caller must ensure that the pointer remains valid for the duration of the call.
    /// - Violating any of these requirements results in **undefined behavior**.
    unsafe fn display_config_get_device_info(
        &self,
        requestpacket: *mut DISPLAYCONFIG_DEVICE_INFO_HEADER,
    ) -> i32;

    /// Retrieves the required buffer sizes for display configuration queries.
    ///
    /// This function writes to the provided output pointers.
    ///
    /// # Safety
    /// - `numpatharrayelements` must be a **valid, non-null, properly aligned** pointer to a `u32`.
    /// - `nummodeinfoarrayelements` must be a **valid, non-null, properly aligned** pointer to a `u32`.
    /// - Both pointers must be **writable** and point to initialized memory.
    /// - The memory must not be aliased mutably elsewhere during the call.
    /// - The pointers must remain valid for the full duration of the call.
    /// - Passing invalid pointers or violating aliasing rules results in **undefined behavior**.
    unsafe fn get_display_config_buffer_sizes(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        nummodeinfoarrayelements: *mut u32,
    ) -> WIN32_ERROR;

    /// Queries the current display configuration.
    ///
    /// Populates user-provided buffers with path and mode information.
    ///
    /// # Safety
    /// - `numpatharrayelements` and `nummodeinfoarrayelements` must be **valid, non-null, aligned**
    ///   pointers to `u32` values.
    /// - `patharray` must point to a **valid writable buffer** of at least
    ///   `*numpatharrayelements` elements of `DISPLAYCONFIG_PATH_INFO`.
    /// - `modeinfoarray` must point to a **valid writable buffer** of at least
    ///   `*nummodeinfoarrayelements` elements of `DISPLAYCONFIG_MODE_INFO`.
    /// - If `currenttopologyid` is `Some(ptr)`, `ptr` must be a **valid, writable, aligned**
    ///   pointer to a `DISPLAYCONFIG_TOPOLOGY_ID`.
    /// - All provided memory must be properly aligned, initialized where required,
    ///   and remain valid for the duration of the call.
    /// - No provided memory may be mutably aliased elsewhere during the call.
    /// - Buffer sizes must match the counts provided, otherwise **undefined behavior** may occur.
    unsafe fn query_display_config(
        &self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: ::core::option::Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> WIN32_ERROR;

    /// Changes display settings for a device.
    ///
    /// Directly forwards parameters to the Win32 API.
    ///
    /// # Safety
    /// - `lpszdevicename` must be a **valid, properly aligned UTF-16 null-terminated string pointer**
    ///   or `PCWSTR::null()` if allowed by the API contract.
    /// - If `lpdevmode` is `Some(ptr)`, `ptr` must be a **valid, aligned pointer** to a `DEVMODEW`
    ///   structure that remains valid for the duration of the call.
    /// - If `lparam` is `Some(ptr)`, it must be a **valid pointer** for the expected Win32 usage.
    /// - All pointers must obey alignment and lifetime requirements.
    /// - The provided pointers must not be mutably aliased elsewhere.
    /// - The caller must uphold all Win32 API invariants for `ChangeDisplaySettingsExW`.
    /// - Violating any of these conditions results in **undefined behavior**.
    unsafe fn change_display_settings_ex_w(
        &mut self,
        lpszdevicename: PCWSTR,
        lpdevmode: ::core::option::Option<*const DEVMODEW>,
        hwnd: Option<HWND>,
        dwflags: CDS_TYPE,
        lparam: ::core::option::Option<*const ::core::ffi::c_void>,
    ) -> DISP_CHANGE;

    /// Enumerates display devices.
    ///
    /// Writes device information into the provided structure.
    ///
    /// # Safety
    /// - `lpdevice` must be a **valid UTF-16 null-terminated string pointer** or null if permitted.
    /// - `lpdisplaydevice` must be a **valid, non-null, aligned, writable** pointer to a
    ///   `DISPLAY_DEVICEW` structure.
    /// - The structure must be properly initialized as required by the Win32 API.
    /// - The pointer must remain valid for the duration of the call.
    /// - The memory must not be mutably aliased elsewhere.
    /// - Any violation of these requirements results in **undefined behavior**.
    unsafe fn enum_display_devices_w(
        &self,
        lpdevice: PCWSTR,
        idevnum: u32,
        lpdisplaydevice: *mut DISPLAY_DEVICEW,
        dwflags: u32,
    ) -> BOOL;

    /// Enumerates display settings for a device.
    ///
    /// Writes the display mode into the provided `DEVMODEW` structure.
    ///
    /// # Safety
    /// - `lpszdevicename` must be a **valid UTF-16 null-terminated string pointer** or null if allowed.
    /// - `lpdevmode` must be a **valid, non-null, aligned, writable** pointer to a `DEVMODEW`.
    /// - The `DEVMODEW` structure must be properly initialized as required by the Win32 API.
    /// - The memory must remain valid for the entire call.
    /// - No mutable aliasing of the pointed memory is allowed.
    /// - Failing to uphold these invariants results in **undefined behavior**.
    unsafe fn enum_display_settings_w(
        &self,
        lpszdevicename: PCWSTR,
        imodenum: ENUM_DISPLAY_SETTINGS_MODE,
        lpdevmode: *mut DEVMODEW,
    ) -> BOOL;
}
