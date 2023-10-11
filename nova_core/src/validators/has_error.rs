use crate::errors::ServerError;

/// Check for errors in `Self`
pub trait ValidateHasError
where
    Self: Sized,
{
    /// Verify if `Self` contains error
    fn has_error(&self) -> Result<Self, ServerError>;
}

impl<T: Clone> ValidateHasError for Vec<Result<T, ServerError>> {
    fn has_error(&self) -> Result<Self, ServerError> {
        self.iter().find(|a| a.is_err()).map_or_else(
            || Ok(self.clone()),
            |error| Err(error.clone().err().unwrap()),
        )
    }
}
