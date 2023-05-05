use crate::{
    error::Error, from_multipart_field::FromMultipartField, multipart_form::MultipartField,
};
use http::HeaderMap;
use mime::Mime;
use multer::bytes::Bytes;

/// Represents a file sent in a form.
#[derive(Debug, Clone)]
pub struct FormFile {
    bytes: Bytes,
    headers: HeaderMap,
    content_type: Mime,
    name: String,
    file_name: String,
}

impl FormFile {
    /// Returns the bytes of this file.
    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Return the name of the form field
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the file name.
    pub fn file_name(&self) -> &str {
        &self.file_name
    }

    /// Returns the headers of the file.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns the content type of this file.
    pub fn content_type(&self) -> &Mime {
        &self.content_type
    }
}

impl FromMultipartField for FormFile {
    fn from_field(field: &MultipartField) -> Result<Self, Error> {
        let name = field
            .name()
            .ok_or_else(|| Error::new("field does not have a name"))?
            .to_owned();

        let file_name = field
            .file_name()
            .ok_or_else(|| Error::new("file does not have a name"))?
            .to_owned();

        let headers = field.headers().clone();
        let bytes = field.bytes().clone();
        let content_type = field
            .content_type()
            .cloned()
            .unwrap_or(mime::APPLICATION_OCTET_STREAM);

        Ok(FormFile {
            bytes,
            headers,
            name,
            file_name,
            content_type,
        })
    }
}
