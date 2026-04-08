use convertible_couch::application::CommandResult;
use convertible_couch_lib::{
    application_error::ApplicationError, application_result::ApplicationResult,
};

use crate::arrangements::builders::command_result::{
    change::ChangeResultBuilder, info::InfoResultBuilder,
};

pub mod change;
pub mod info;

pub struct CommandResultBuilder;

impl CommandResultBuilder {
    pub fn change() -> ChangeResultBuilder {
        ChangeResultBuilder
    }

    pub fn info() -> InfoResultBuilder {
        InfoResultBuilder
    }

    pub fn custom_error(expected_message: String) -> ApplicationResult<CommandResult> {
        let application_error = ApplicationError::Custom(expected_message);

        Err(application_error)
    }
}
