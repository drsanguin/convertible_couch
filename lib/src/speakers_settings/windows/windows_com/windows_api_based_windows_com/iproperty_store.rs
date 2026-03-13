use crate::speakers_settings::windows::windows_com::IPropertyStore as IPropertyStoreTrait;
use windows::Win32::{
    Foundation::PROPERTYKEY, System::Com::StructuredStorage::PROPVARIANT,
    UI::Shell::PropertiesSystem::IPropertyStore,
};
use windows_core::Result;

pub struct WindowsApiBasedIPropertyStore {
    iproperty_store: IPropertyStore,
}

impl WindowsApiBasedIPropertyStore {
    pub fn new(iproperty_store: IPropertyStore) -> Self {
        Self { iproperty_store }
    }
}

impl IPropertyStoreTrait for WindowsApiBasedIPropertyStore {
    unsafe fn get_value(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        unsafe { self.iproperty_store.GetValue(key) }
    }
}
