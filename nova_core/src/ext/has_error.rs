use crate::errors::ServerError;

pub(crate) trait ValidateHasError
where
    Self: Sized,
{
    fn has_error(&self) -> Result<Self, ServerError>;
}

impl<T: Clone> ValidateHasError for Vec<Result<T, ServerError>> {
    fn has_error(&self) -> Result<Self, ServerError> {
        let has_error = self.iter().find(|a| a.is_err());
        if let Some(error) = has_error {
            Err(error.clone().err().unwrap())
        } else {
            Ok(self.clone())
        }
    }
}
