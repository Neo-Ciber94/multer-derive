/// An error that ocurred while processing a multipart.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error ocurred in `multer`.
    #[error(transparent)]
    MultipartError(multer::Error),

    /// Other error that ocurred.
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync>),
}

impl Error {
    /// Constructs a new error.
    pub fn new(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        Error::Other(error.into())
    }

    /// Constructs an error from multer.
    pub fn from_multer(error: multer::Error) -> Self {
        Error::MultipartError(error)
    }
}

impl From<multer::Error> for Error {
    fn from(error: multer::Error) -> Self {
        Error::from_multer(error)
    }
}
