use std::{ffi::NulError, num::TryFromIntError, str::Utf8Error};
pub type Result<T> = std::result::Result<T, DssError>;

#[derive(Debug)]
pub enum DssError {
    InvalidUtf8(Utf8Error),
    NullCPtr,
    CallFail,
    TryFromInt,
}

impl From<Utf8Error> for DssError {
    fn from(err: Utf8Error) -> Self {
        DssError::InvalidUtf8(err)
    }
}

impl From<NulError> for DssError {
    fn from(_: NulError) -> Self {
        DssError::NullCPtr
    }
}

impl From<TryFromIntError> for DssError {
    fn from(_: TryFromIntError) -> Self {
        DssError::TryFromInt
    }
}
