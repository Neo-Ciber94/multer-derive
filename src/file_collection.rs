use crate::{
    error::Error, form_file::FormFile, from_multipart_field::FromMultipartField, FromMultipart,
    MultipartForm,
};

/// Provides a way to collect all the files in a `form`.
pub struct FileCollection(Vec<FormFile>);

impl FileCollection {
    /// Returns all the collected files.
    pub fn into_inner(self) -> Vec<FormFile> {
        self.0
    }
}

impl FromMultipart for FileCollection {
    fn from_multipart(
        multipart: &MultipartForm,
        ctx: crate::from_multipart::FormContext<'_>,
    ) -> Result<Self, Error> {
        let mut files = vec![];

        for field in multipart.fields() {
            if field.file_name().is_none() {
                continue;
            }

            if ctx.field_name.is_some() && ctx.field_name != field.name() {
                continue;
            }

            files.push(FormFile::from_field(field)?);
        }

        Ok(FileCollection(files))
    }
}
