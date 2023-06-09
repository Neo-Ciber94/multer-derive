use multer_derive::{multer::Multipart, FormFile, FromMultipart, MultipartForm};

const FORM_DATA :&str = "--boundary\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nfield1_value\r\n--boundary\r\nContent-Disposition: form-data; name=\"files\"; filename=\"filename1\"\r\nContent-Type: application/octet-stream\r\n\r\n[contents of file1]\r\n--boundary\r\nContent-Disposition: form-data; name=\"files\"; filename=\"filename2\"\r\nContent-Type: application/octet-stream\r\n\r\n[contents of file2]\r\n--boundary\r\nContent-Disposition: form-data; name=\"photo\"; filename=\"filename3\"\r\nContent-Type: application/octet-stream\r\n\r\n[contents of file3]\r\n--boundary--";

#[derive(FromMultipart)]
struct InputWithFiles1 {
    name: String,
    photo: FormFile,
    files: Vec<FormFile>,
}

#[tokio::test]
async fn multiple_files_test() {
    let reader = FORM_DATA.as_bytes();
    let multipart = Multipart::with_reader(reader, "boundary");

    let form = MultipartForm::with_multipart(multipart).await.unwrap();
    let result = InputWithFiles1::from_multipart(&form, Default::default()).unwrap();

    assert_eq!(result.files.len(), 2);
    assert_eq!(result.name, "field1_value");

    let files = result.files;

    //
    let file1 = String::from_utf8(files[0].bytes().to_vec()).unwrap();
    assert_eq!(files[0].name(), "files");
    assert_eq!(files[0].file_name(), "filename1");
    assert_eq!(file1, "[contents of file1]");

    let file2 = String::from_utf8(files[1].bytes().to_vec()).unwrap();
    assert_eq!(files[1].name(), "files");
    assert_eq!(files[1].file_name(), "filename2");
    assert_eq!(file2, "[contents of file2]");

    let file3 = String::from_utf8(result.photo.bytes().to_vec()).unwrap();
    assert_eq!(result.photo.name(), "photo");
    assert_eq!(result.photo.file_name(), "filename3");
    assert_eq!(file3, "[contents of file3]");
}
