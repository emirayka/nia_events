#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    UInputConfigurationError,
    UInputSendError,
    XorgError,
    KeyParseError,
}

#[derive(Debug, Clone)]
pub struct Error {
    error_kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(error_kind: ErrorKind, message: &str) -> Error {
        Error {
            error_kind,
            message: String::from(message),
        }
    }

    pub fn get_message(&self) -> &String {
        &self.message
    }

    pub fn uinput_configuration_error(message: &str) -> Error {
        Error::new(ErrorKind::UInputConfigurationError, message)
    }

    pub fn uinput_error(message: &str) -> Error {
        Error::new(ErrorKind::UInputSendError, message)
    }

    pub fn xorg_error(message: &str) -> Error {
        Error::new(ErrorKind::XorgError, message)
    }

    pub fn key_parse_error(message: &str) -> Error {
        Error::new(ErrorKind::KeyParseError, message)
    }
}
