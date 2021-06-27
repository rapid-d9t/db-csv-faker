use std::fmt;
use std::io;

#[derive(fmt::Debug)]
pub struct FieldError {
    error_type: FieldErrorType,
    message: String,
}

#[derive(fmt::Debug, Clone, PartialEq)]
pub enum FieldErrorType {
    InvalidInputArgs,
}

impl fmt::Display for FieldErrorType {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldErrorType::InvalidInputArgs => write!(formatter, "InvalidInputArgs"),
        }
    }
}

impl FieldError {
    pub fn new(message: String, error_type: FieldErrorType) -> FieldError {
        FieldError {
            message,
            error_type,
        }
    }

    pub fn get_message(&self) -> String {
        self.message.clone()
    }

    pub fn get_error_type(&self) -> FieldErrorType {
        self.error_type.clone()
    }
}

impl fmt::Display for FieldError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "Field Error with type: {}, reason: {}",
            self.error_type, self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use super::FieldError;
    use super::FieldErrorType;

    #[test]
    fn it_inits() {
        let invalid_args_error =
            FieldError::new("field error".to_string(), FieldErrorType::InvalidInputArgs);

        assert_eq!(invalid_args_error.get_message(), "field error");
        assert_eq!(
            invalid_args_error.get_error_type(),
            FieldErrorType::InvalidInputArgs
        );
    }

    #[test]
    fn it_displayable() {
        let invalid_args_error =
            FieldError::new("field error".to_string(), FieldErrorType::InvalidInputArgs);

        assert_eq!(
            format!("{}", invalid_args_error),
            "Field Error reason: InvalidInputArgs, reason: file error"
        );
    }
}
