use crate::{
    speakers_settings::windows::windows_api::IPropertyStore as IPropertyStoreTrait, trace_fn,
};
use windows::Win32::{
    Foundation::PROPERTYKEY, System::Com::StructuredStorage::PROPVARIANT,
    UI::Shell::PropertiesSystem::IPropertyStore,
};
use windows_core::Result;

pub struct Win32BasedIPropertyStore {
    iproperty_store: IPropertyStore,
}

impl Win32BasedIPropertyStore {
    pub fn new(iproperty_store: IPropertyStore) -> Self {
        trace_fn!();

        Self { iproperty_store }
    }
}

impl IPropertyStoreTrait for Win32BasedIPropertyStore {
    unsafe fn get_value(&self, key: *const PROPERTYKEY) -> Result<PROPVARIANT> {
        trace_fn!();

        unsafe { self.iproperty_store.GetValue(key) }
    }
}
