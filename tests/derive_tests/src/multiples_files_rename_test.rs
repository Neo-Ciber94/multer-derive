use multer_derive::{
    helpers::MultipartFormBuilder, multer::Multipart, FormFile, FromMultipart, MultipartForm,
};

#[derive(Debug, FromMultipart)]
struct FormWithMultipleFiles {
    name: String,
    #[multer(rename = "files[]")]
    files: Vec<FormFile>,
}

fn get_form_data() -> String {
    MultipartFormBuilder::new()
        .text("name", "collections")
        .raw_file(
            "files[]",
            b"[contents of file1]",
            "file1.txt",
            multer_derive::mime::TEXT_PLAIN,
        )
        .raw_file(
            "files[]",
            b"[contents of file2]",
            "file2.txt",
            multer_derive::mime::TEXT_PLAIN,
        )
        .raw_file(
            "files[]",
            b"[contents of file3]",
            "file3.txt",
            multer_derive::mime::TEXT_PLAIN,
        )
        .build("my_boundary")
}

#[tokio::test]
async fn multiple_files_rename_test() {
    let form_data = get_form_data();
    let multipart = Multipart::with_reader(form_data.as_bytes(), "my_boundary");

    let form = MultipartForm::with_multipart(multipart).await.unwrap();
    let result = FormWithMultipleFiles::from_multipart(&form, Default::default()).unwrap();

    assert_eq!(result.files.len(), 3, "{result:#?}, actual:\n{form_data}");
    assert_eq!(result.name, "collections");

    let files = result.files;

    //
    let file1 = String::from_utf8(files[0].bytes().to_vec()).unwrap();
    assert_eq!(files[0].name(), "files[]");
    assert_eq!(files[0].file_name(), "file1.txt");
    assert_eq!(file1, "[contents of file1]");

    let file2 = String::from_utf8(files[1].bytes().to_vec()).unwrap();
    assert_eq!(files[1].name(), "files[]");
    assert_eq!(files[1].file_name(), "file2.txt");
    assert_eq!(file2, "[contents of file2]");

    let file3 = String::from_utf8(files[2].bytes().to_vec()).unwrap();
    assert_eq!(files[2].name(), "files[]");
    assert_eq!(files[2].file_name(), "file3.txt");
    assert_eq!(file3, "[contents of file3]");
}
