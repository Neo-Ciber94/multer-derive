use indexmap::IndexMap;
use mime::Mime;
use std::borrow::Cow;
use std::fmt::Write;
use std::io::Read;
use std::path::Path;

/// An input file
#[derive(Debug, Clone)]
pub struct InputFile {
    pub contents: Vec<u8>,
    pub file_name: String,
    pub content_type: Mime,
}

/// Represents a form input.
#[derive(Debug, Clone)]
pub enum InputValue {
    /// A text input.
    Text(String),

    /// A binary file input.
    Files(Vec<InputFile>),
}

impl InputValue {
    /// Returns `true` if is text.
    pub fn is_text(&self) -> bool {
        matches!(self, InputValue::Text(_))
    }

    /// Returns `true` if is a file.
    pub fn is_file(&self) -> bool {
        matches!(self, InputValue::Files(_))
    }
}

/// A builder for multipart form data strings.
#[derive(Debug, Clone)]
pub struct MultipartFormBuilder {
    fields: IndexMap<String, InputValue>,
}

impl MultipartFormBuilder {
    /// Creates a new MultipartFormBuilder instance.
    pub fn new() -> Self {
        Self {
            fields: IndexMap::new(),
        }
    }

    /// Adds a text input field to the form.
    ///
    /// # Example
    ///
    /// ```
    /// use multer_derive::helpers::{MultipartFormBuilder, InputValue};
    ///
    /// let mut builder = MultipartFormBuilder::new();
    /// builder.text("username", "john_doe");
    /// ```
    pub fn text(&mut self, name: &str, value: &str) -> &mut Self {
        self.fields
            .insert(name.to_owned(), InputValue::Text(value.to_owned()));
        self
    }

    /// Adds a file input field to the form.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use multer_derive::helpers::{MultipartFormBuilder, InputValue};
    /// use std::fs::File;
    /// use std::io::Read;
    /// use multer_derive::mime::APPLICATION_OCTET_STREAM;
    ///
    /// let mut contents = Vec::new();
    /// let mut file = File::open("example.png").unwrap();
    /// file.read_to_end(&mut contents).unwrap();
    ///
    /// let mut builder = MultipartFormBuilder::new();
    /// builder.raw_file("image", &contents, "example.png", mime::IMAGE_PNG);
    /// ```
    pub fn raw_file(
        &mut self,
        name: &str,
        contents: impl ToBytes,
        file_name: &str,
        content_type: Mime,
    ) -> &mut Self {
        let file = InputFile {
            contents: contents.to_bytes(),
            file_name: file_name.to_owned(),
            content_type,
        };

        let cur: Option<&mut InputValue> = self.fields.get_mut(name);

        if cur.is_none() {
            self.fields
                .insert(name.to_owned(), InputValue::Files(vec![file]));
            return self;
        }

        let cur = cur.unwrap();

        if cur.is_file() {
            let InputValue::Files(files) = cur else {
                unreachable!()
            };

            files.push(file);
        } else {
            *cur = InputValue::Files(vec![file])
        }

        self
    }

    /// Adds a file from the given path, returns an error if fails to read the file.
    pub fn file_from_path(
        &mut self,
        name: &str,
        path: impl AsRef<Path>,
    ) -> std::io::Result<&mut Self> {
        // First, we open the file at the specified path.
        let mut file = std::fs::File::open(path.as_ref())?;

        // Next, we read the contents of the file into a `Vec` of bytes.
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let content_type = mime_guess::from_path(path.as_ref()).first_or_octet_stream();

        let file_name = path
            .as_ref()
            .file_name() // SAFETY: If null we fill fail to open above
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        Ok(self.raw_file(name, contents, &file_name, content_type))
    }

    /// Builds the multipart form data string.
    ///
    /// # Example
    ///
    /// ```
    /// use multer_derive::helpers::{MultipartFormBuilder, InputValue};
    ///
    /// let mut builder = MultipartFormBuilder::new();
    /// builder.text("username", "john_doe");
    ///
    /// let data = builder.build("my_boundary");
    /// ```
    pub fn build(&mut self, boundary: &str) -> String {
        let mut body = String::new();

        for (name, value) in &self.fields {
            match value {
                InputValue::Text(text) => {
                    body += &format!("--{}\r\n", boundary);
                    body += &format!(
                        "Content-Disposition: form-data; name=\"{}\"\r\n\r\n{}\r\n",
                        name, text
                    );
                }
                InputValue::Files(files) => {
                    for InputFile {
                        file_name,
                        contents,
                        content_type,
                    } in files
                    {
                        body += &format!("--{}\r\n", boundary);
                        let content_disposition = format!(
                            "Content-Disposition: form-data; name=\"{name}\"; filename=\"{file_name}\"\r\n"
                        );

                        body += &content_disposition;
                        body += &format!("Content-Type: {}\r\n\r\n", content_type);

                        let mut buf = String::new();
                        for byte in contents {
                            write!(&mut buf, "{}", *byte as char).unwrap();
                        }

                        body += &buf;
                        body += "\r\n";
                    }
                }
            }
        }

        body += &format!("--{}--\r\n", boundary);

        body
    }
}

/// Convert a type into a `Vec<u8>`
pub trait ToBytes {
    fn to_bytes(self) -> Vec<u8>;
}

impl ToBytes for Vec<u8> {
    fn to_bytes(self) -> Vec<u8> {
        self
    }
}

impl ToBytes for &'_ Vec<u8> {
    fn to_bytes(self) -> Vec<u8> {
        self.clone()
    }
}

impl<const N: usize> ToBytes for [u8; N] {
    fn to_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl<const N: usize> ToBytes for &'_ [u8; N] {
    fn to_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ToBytes for &'_ [u8] {
    fn to_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ToBytes for &'_ str {
    fn to_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for String {
    fn to_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for Cow<'_, [u8]> {
    fn to_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ToBytes for Cow<'_, str> {
    fn to_bytes(self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl ToBytes for crate::multer::bytes::Bytes {
    fn to_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ToBytes for crate::multer::bytes::BytesMut {
    fn to_bytes(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl ToBytes for std::str::Bytes<'_> {
    fn to_bytes(self) -> Vec<u8> {
        self.collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_is_text() {
        let text_input = InputValue::Text("hello".to_owned());
        let file_input = InputValue::Files(vec![InputFile {
            contents: vec![],
            file_name: "".to_owned(),
            content_type: mime::APPLICATION_OCTET_STREAM,
        }]);

        assert!(text_input.is_text());
        assert!(!file_input.is_text());
    }

    #[test]
    fn test_is_file() {
        let text_input = InputValue::Text("hello".to_owned());
        let file_input = InputValue::Files(vec![InputFile {
            contents: vec![],
            file_name: "".to_owned(),
            content_type: mime::APPLICATION_OCTET_STREAM,
        }]);

        assert!(!text_input.is_file());
        assert!(file_input.is_file());
    }

    #[test]
    fn test_text() {
        let mut builder = MultipartFormBuilder::new();
        builder.text("username", "john_doe");

        let form_data = builder.build("my_boundary");

        assert!(form_data
            .contains("Content-Disposition: form-data; name=\"username\"\r\n\r\njohn_doe\r\n"));
    }

    #[test]
    fn test_raw_file() {
        let mut builder = MultipartFormBuilder::new();
        builder.raw_file(
            "file",
            vec![0, 1, 2],
            "file.bin",
            mime::APPLICATION_OCTET_STREAM,
        );

        let form_data = builder.build("my_boundary");

        assert!(form_data
            .contains("Content-Disposition: form-data; name=\"file\"; filename=\"file.bin\"\r\n"));
        assert!(
            form_data.contains("Content-Type: application/octet-stream\r\n\r\n\u{0}\u{1}\u{2}\r\n")
        );
    }

    #[test]
    fn test_file_from_path() {
        use std::io::Write;

        let mut builder = MultipartFormBuilder::new();
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let temp_dir = tempfile::tempdir_in(dir).unwrap();
        let file_path = temp_dir.path().join("example.txt");
        let mut file = std::fs::File::create(&file_path).unwrap();
        write!(file, "Hello World!").unwrap();

        builder.file_from_path("example_file", &file_path).unwrap();

        let form_data = builder.build("my_boundary");

        assert!(form_data.contains(
            "Content-Disposition: form-data; name=\"example_file\"; filename=\"example.txt\"\r\n"
        ), "actual:\n{form_data}");
        assert!(
            form_data.contains("Content-Type: text/plain\r\n\r\nHello World!"),
            "actual:\n{form_data}"
        );
    }

    #[test]
    fn test_build() {
        let mut builder = MultipartFormBuilder::new();
        builder.text("username", "john_doe");
        builder.raw_file(
            "file",
            vec![0, 1, 2],
            "file.bin",
            mime::APPLICATION_OCTET_STREAM,
        );

        let form_data = builder.build("my_boundary");

        assert_eq!(
            form_data,
            format!(
                "--my_boundary\r\nContent-Disposition: form-data; name=\"username\"\r\n\r\njohn_doe\r\n--my_boundary\r\nContent-Disposition: form-data; name=\"file\"; filename=\"file.bin\"\r\nContent-Type: application/octet-stream\r\n\r\n\u{0}\u{1}\u{2}\r\n--my_boundary--\r\n"
            )
        );
    }
}
