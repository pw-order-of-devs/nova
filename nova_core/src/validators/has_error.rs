use crate::errors::ServerError;

pub trait ValidateHasError
where
    Self: Sized,
{
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
