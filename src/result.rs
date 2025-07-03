// use errors in errors by default and define Result that automatically attach Error
use crate::errors::ProtocolError;

pub type Result<T> = std::result::Result<T, ProtocolError>;
