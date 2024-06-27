
#[derive(Debug)]
pub enum Error {
    ParseIntError(std::num::ParseIntError),
    TryFromSliceError(std::array::TryFromSliceError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


impl std::error::Error for Error {}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<std::array::TryFromSliceError> for Error {
    fn from(err: std::array::TryFromSliceError) -> Self {
        Error::TryFromSliceError(err)
    }
}