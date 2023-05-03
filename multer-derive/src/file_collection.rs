use crate::{
    error::Error, form_file::FormFile, from_multipart_field::FormMultipartField,
    multipart_form::MultipartField, MultipartForm,
};

/// Provides a way to collect all the files in a `form`.
pub struct FileCollection(Vec<FormFile>);

impl FileCollection {
    /// Returns all the collected files.
    pub fn into_inner(self) -> Vec<FormFile> {
        self.0
    }
}

impl FormMultipartField for FileCollection {
    fn from_field(_: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        let mut files = vec![];

        // We get all the files that can be converted into `T`
        for field in form.fields() {
            if field.file_name().is_none() {
                continue;
            }

            let file = FormFile::from_field(field, form)?;
            files.push(file);
        }

        Ok(FileCollection(files))
    }
}
