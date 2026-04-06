use windows::Win32::Foundation::WIN32_ERROR;

use crate::arrangements::fuzzing::speakers::settings_api::behaviour::FuzzedSpeakersSettingsApiBehaviour;

#[derive(Clone, Default)]
pub struct FuzzedWindowsSpeakersSettingsApiBehaviour {
    pub co_initialize_ex_error: Option<WIN32_ERROR>,
    pub co_create_immdevice_enumerator_error: Option<WIN32_ERROR>,
    pub immdevice_enumerator_get_default_audio_endpoint_error: Option<WIN32_ERROR>,
    pub immdevice_enumerator_enum_audio_endpoints_error: Option<WIN32_ERROR>,
    pub immdevice_get_id_error: Option<WIN32_ERROR>,
    pub immdevice_collection_get_count_error: Option<WIN32_ERROR>,
    pub immdevice_collection_item_error: Option<WIN32_ERROR>,
    pub immdevice_open_property_store_error: Option<WIN32_ERROR>,
    pub property_store_get_value_error: Option<WIN32_ERROR>,
    pub co_create_ipolicy_config_vista_error: Option<WIN32_ERROR>,
    pub ipolicy_config_vista_set_default_endpoint_error: Option<WIN32_ERROR>,
}

impl FuzzedSpeakersSettingsApiBehaviour for FuzzedWindowsSpeakersSettingsApiBehaviour {}
