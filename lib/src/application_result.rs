use crate::application_error::ApplicationError;

pub type ApplicationResult<T> = Result<T, ApplicationError>;
