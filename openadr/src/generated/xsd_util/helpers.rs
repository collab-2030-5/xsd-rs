use crate::xsd_util::ReadError;

pub(crate) struct SetOnce<T> {
    inner: Option<T>,
}

impl<T> Default for SetOnce<T> {
    fn default() -> Self {
        Self { inner: None }
    }
}

impl<T> SetOnce<T> {
    pub(crate) fn set(&mut self, value: T) -> core::result::Result<(), ReadError> {
        if self.inner.is_some() {
            return Err(ReadError::DuplicateField);
        }
        self.inner = Some(value);
        Ok(())
    }

    pub(crate) fn require(self) -> core::result::Result<T, ReadError> {
        match self.inner {
            Some(x) => Ok(x),
            None => Err(ReadError::MissingMandatoryField),
        }
    }

    pub(crate) fn get(self) -> Option<T> {
        self.inner
    }
}
