use encoding_rs::Encoding;
use http::HeaderMap;
use mime::Mime;
use multer::{bytes::Bytes, Multipart};
use std::{borrow::Cow, ops::Index};

/// A field in a multipart form.
#[derive(Clone)]
pub struct MultipartField {
    name: Option<String>,
    file_name: Option<String>,
    content_type: Option<Mime>,
    headers: HeaderMap,
    bytes: Bytes,
    index: usize,
}

impl MultipartField {
    /// Returns the index of this field in the form.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns the name of this field.
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the file name of this field.
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
    }

    /// Returns the content type of this field.
    pub fn content_type(&self) -> Option<&Mime> {
        self.content_type.as_ref()
    }

    /// Returns the headers of this field.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Returns the bytes content of this field.
    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Converts the bytes of this field to a `utf-8` string.
    pub fn text(&self) -> String {
        self.text_with_charset("utf-8")
    }

    /// Converts this field to a string using the given encoding.
    ///
    /// Checkout: <https://docs.rs/encoding_rs/latest/encoding_rs/struct.Encoding.html>
    pub fn text_with_charset(&self, default_encoding: &str) -> String {
        let encoding_name = self
            .content_type()
            .and_then(|mime| mime.get_param(mime::CHARSET))
            .map(|charset| charset.as_str())
            .unwrap_or(default_encoding);

        let encoding = Encoding::for_label(encoding_name.as_bytes()).unwrap_or(encoding_rs::UTF_8);
        let bytes = self.bytes();
        let (text, ..) = encoding.decode(bytes);

        match text {
            Cow::Owned(s) => s,
            Cow::Borrowed(s) => String::from(s),
        }
    }
}

/// A multipart form.
#[derive(Clone)]
pub struct MultipartForm {
    fields: Vec<MultipartField>,
}

impl MultipartForm {
    /// Creates a multipart form by caching all the fields in the [`multer::Multipart`].
    pub async fn with_multipart(mut multipart: Multipart<'_>) -> multer::Result<MultipartForm> {
        let mut fields = vec![];

        while let Some((index, field)) = multipart.next_field_with_idx().await? {
            let name = field.name().map(|s| s.to_owned());
            let file_name = field.file_name().map(|s| s.to_owned());
            let content_type = field.content_type().cloned();
            let headers = field.headers().clone();
            let bytes = field.bytes().await?;

            fields.push(MultipartField {
                name,
                file_name,
                content_type,
                headers,
                bytes,
                index,
            })
        }

        Ok(MultipartForm { fields })
    }

    /// Returns the field in the given index.
    pub fn get(&self, index: usize) -> Option<&MultipartField> {
        self.fields.get(index)
    }

    /// Returns the field with the given name
    pub fn get_by_name(&self, name: &str) -> Option<&MultipartField> {
        self.fields.iter().find(|x| x.name() == Some(name))
    }

    /// Returns all the fields.
    pub fn fields(&self) -> &[MultipartField] {
        self.fields.as_slice()
    }

    /// Returns the number of fields.
    pub fn len(&self) -> usize {
        self.fields.len()
    }

    /// Returns true if this form had no fields.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Index<usize> for MultipartForm {
    type Output = MultipartField;

    fn index(&self, index: usize) -> &Self::Output {
        &self.fields[index]
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderValue;
    use multer::Multipart;

    use crate::multipart_form::MultipartForm;

    const MULTI_PART_STR: &str = "--MyBoundary\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nJohn Doe\r\n--MyBoundary\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\njohndoe@example.com\r\n--MyBoundary\r\nContent-Disposition: form-data; name=\"age\"\r\n\r\n25\r\n--MyBoundary\r\nContent-Disposition: form-data; name=\"file\"; filename=\"example.txt\"\r\nContent-Type: text/plain\r\n\r\nThis is an example file.\r\n--MyBoundary--\r\n";

    #[tokio::test]
    async fn with_multipart_test() {
        let reader = MULTI_PART_STR.as_bytes();
        let multipart = Multipart::with_reader(reader, "MyBoundary");

        let form = MultipartForm::with_multipart(multipart).await.unwrap();

        assert_eq!(form.len(), 4);
        assert_eq!(form[0].name(), Some("name"));
        assert_eq!(form[1].name(), Some("email"));
        assert_eq!(form[2].name(), Some("age"));
        assert_eq!(form[3].name(), Some("file"));
    }

    #[tokio::test]
    async fn with_multipart_get_test() {
        let reader = MULTI_PART_STR.as_bytes();
        let multipart = Multipart::with_reader(reader, "MyBoundary");

        let form = MultipartForm::with_multipart(multipart).await.unwrap();

        assert_eq!(form.get(0).unwrap().name(), Some("name"));
        assert_eq!(form.get(1).unwrap().name(), Some("email"));
        assert_eq!(form.get(2).unwrap().name(), Some("age"));
        assert_eq!(form.get(3).unwrap().name(), Some("file"));
        assert!(form.get(4).is_none());
    }

    #[tokio::test]
    async fn form_field_text_test() {
        let reader = MULTI_PART_STR.as_bytes();
        let multipart = Multipart::with_reader(reader, "MyBoundary");

        let form = MultipartForm::with_multipart(multipart).await.unwrap();

        assert_eq!(form.get(0).unwrap().text(), String::from("John Doe"));
        assert_eq!(
            form.get(1).unwrap().text(),
            String::from("johndoe@example.com")
        );
        assert_eq!(form.get(2).unwrap().text(), String::from("25"));
        assert_eq!(
            form.get(3).unwrap().text(),
            String::from("This is an example file.")
        );
        assert!(form.get(4).is_none());
    }

    #[tokio::test]
    async fn form_field_file_test() {
        let reader = MULTI_PART_STR.as_bytes();
        let multipart = Multipart::with_reader(reader, "MyBoundary");

        let form = MultipartForm::with_multipart(multipart).await.unwrap();

        let file = &form[3];
        assert_eq!(file.text(), "This is an example file.");
        assert_eq!(file.file_name(), Some("example.txt"));
        assert_eq!(file.content_type(), Some(&mime::TEXT_PLAIN));

        let headers = file.headers();
        assert_eq!(headers.len(), 2);

        assert_eq!(
            headers.get("content-disposition"),
            Some(&HeaderValue::from_static(
                "form-data; name=\"file\"; filename=\"example.txt\""
            ))
        );

        assert_eq!(
            headers.get("content-type"),
            Some(&HeaderValue::from_static("text/plain"))
        );
    }
}
