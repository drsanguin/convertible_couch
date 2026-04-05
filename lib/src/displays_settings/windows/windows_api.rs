use windows::Win32::{
    Devices::Display::{
        DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_MODE_INFO, DISPLAYCONFIG_PATH_INFO,
        DISPLAYCONFIG_TOPOLOGY_ID, QUERY_DISPLAY_CONFIG_FLAGS, SET_DISPLAY_CONFIG_FLAGS,
    },
    Foundation::WIN32_ERROR,
};

pub trait WindowsApi {
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
        &mut self,
        flags: QUERY_DISPLAY_CONFIG_FLAGS,
        numpatharrayelements: *mut u32,
        patharray: *mut DISPLAYCONFIG_PATH_INFO,
        nummodeinfoarrayelements: *mut u32,
        modeinfoarray: *mut DISPLAYCONFIG_MODE_INFO,
        currenttopologyid: ::core::option::Option<*mut DISPLAYCONFIG_TOPOLOGY_ID>,
    ) -> WIN32_ERROR;

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

    /// Sets the display configuration for the system.
    ///
    /// This is a thin wrapper around the Win32 `SetDisplayConfig` API and directly
    /// forwards the provided parameters without additional validation.
    ///
    /// The function applies a new display topology, source/target paths, and mode
    /// information depending on the provided flags and buffers.
    ///
    /// # Safety
    /// - If `patharray` is `Some(slice)`, the slice must reference a **valid, properly aligned**
    ///   contiguous array of `DISPLAYCONFIG_PATH_INFO` elements.
    /// - If `modeinfoarray` is `Some(slice)`, the slice must reference a **valid, properly aligned**
    ///   contiguous array of `DISPLAYCONFIG_MODE_INFO` elements.
    /// - The memory backing both slices must remain **valid and immutable** for the duration of the call.
    /// - The lengths of the provided slices must match the expectations implied by `flags`,
    ///   as required by the underlying Win32 API.
    /// - If either slice is `None`, the caller must ensure that this is valid according to the
    ///   `SetDisplayConfig` contract and the provided `flags`.
    /// - The caller must uphold all invariants required by the Win32 `SetDisplayConfig` function,
    ///   including correct relationships between path and mode entries.
    /// - No mutable aliasing of the underlying memory may occur during the call.
    /// - Violating any of these requirements results in **undefined behavior**.
    unsafe fn set_display_config(
        &mut self,
        patharray: Option<&[DISPLAYCONFIG_PATH_INFO]>,
        modeinfoarray: Option<&[DISPLAYCONFIG_MODE_INFO]>,
        flags: SET_DISPLAY_CONFIG_FLAGS,
    ) -> i32;
}
