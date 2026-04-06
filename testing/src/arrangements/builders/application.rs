use convertible_couch::application::Application;

use crate::arrangements::fuzzing::computer::FuzzedComputer;

pub struct ApplicationBuilder {
    computer: FuzzedComputer,
}

impl ApplicationBuilder {
    pub fn new(computer: FuzzedComputer) -> Self {
        Self { computer }
    }

    pub fn build(self) -> Application {
        let displays_settings_api = Box::new(self.computer.displays_settings_api);
        let speakers_settings_api = Box::new(self.computer.speakers_settings_api);

        Application::bootstrap(displays_settings_api, speakers_settings_api)
    }
}
