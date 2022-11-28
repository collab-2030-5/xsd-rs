use xsd_api::ReadError;

pub struct SetOnce<T> {
    inner: Option<T>,
}

impl<T> Default for SetOnce<T> {
    fn default() -> Self {
        Self { inner: None }
    }
}

impl<T> SetOnce<T> {
    pub fn set(&mut self, value: T) -> Result<(), ReadError> {
        if self.inner.is_some() {
            return Err(ReadError::DuplicateField);
        }
        self.inner = Some(value);
        Ok(())
    }

    pub fn require(self) -> Result<T, ReadError> {
        match self.inner {
            Some(x) => Ok(x),
            None => Err(ReadError::MissingMandatoryField),
        }
    }

    pub fn get(self) -> Option<T> {
        self.inner
    }
}
