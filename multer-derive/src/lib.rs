#[doc = include_str!("../../README.md")]
mod error;
pub use error::Error;

mod file_collection;
pub use file_collection::FileCollection;

mod form_file;
pub use form_file::FormFile;

mod from_multipart;
pub use from_multipart::{FormContext, FromMultipart};

mod from_multipart_field;
pub use from_multipart_field::FromMultipartField;

mod multipart_form;
pub use multipart_form::{MultipartField, MultipartForm};

// Macro
pub use multer_derive_macros::FromMultipart;

// Re-exports
pub use http::header;
pub use mime;
pub use multer;
