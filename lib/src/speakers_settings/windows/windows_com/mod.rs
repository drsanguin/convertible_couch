use core::ffi::c_void;

use windows::{
    core::{GUID, HRESULT},
    Win32::System::Com::{CLSCTX, COINIT},
};

use windows_core::{IUnknown, Interface, Param, Result};

pub mod windows_api_based_windows_com;

pub trait WindowsCom {
    /// Initializes the COM library for use by the calling thread.
    ///
    /// This is a low-level binding to `CoInitializeEx`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `pvreserved` is either `None` or a valid pointer as required by the Windows API
    ///   contract (it must not point to invalid or unmapped memory).
    /// - The function is called in accordance with COM initialization rules:
    ///   - Each thread must call `co_initialize_ex` before using COM.
    ///   - Each successful call must be balanced with a matching `co_uninitialize` on the same thread.
    /// - `dwcoinit` specifies a valid COM apartment model.
    ///
    /// Calling this function incorrectly may lead to undefined behavior, resource leaks,
    /// or COM runtime corruption.
    unsafe fn co_initialize_ex(
        &self,
        pvreserved: Option<*const c_void>,
        dwcoinit: COINIT,
    ) -> HRESULT;

    /// Uninitializes the COM library on the calling thread.
    ///
    /// This is a low-level binding to `CoUninitialize`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - This function is only called on a thread that previously successfully called
    ///   `co_initialize_ex`.
    /// - Calls are properly balanced: each successful `co_initialize_ex` call must have
    ///   exactly one corresponding `co_uninitialize`.
    /// - No COM objects are still in use on the thread when this is called.
    ///
    /// Violating these rules may result in undefined behavior, including memory corruption,
    /// resource leaks, or COM runtime instability.
    unsafe fn co_uninitialize(&self);

    /// Creates a COM object instance of the specified CLSID.
    ///
    /// This is a low-level binding to `CoCreateInstance`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that:
    /// - `rclsid` is a valid, non-null pointer to a properly initialized `GUID`.
    /// - The memory pointed to by `rclsid` is readable for the duration of the call.
    /// - `punkouter` is either a valid `IUnknown` pointer for aggregation or represents
    unsafe fn co_create_instance<P1, T>(
        &self,
        rclsid: *const GUID,
        punkouter: P1,
        dwclscontext: CLSCTX,
    ) -> Result<T>
    where
        P1: Param<IUnknown>,
        T: Interface;
}
