use multer_derive::{FormFile, FromMultipart, Multipart, MultipartForm};

#[derive(FromMultipart)]
struct Person {
    name: String,
    email: String,
    age: u8,
    married: bool,
    photo: FormFile,
}

#[tokio::test]
async fn person_from_multipart_test() {
    const FORM_DATA : &str = "--boundary_string\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nJohn Smith\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\njohn@example.com\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"age\"\r\n\r\n25\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"married\"\r\n\r\ntrue\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"photo\"; filename=\"example.jpg\"\r\nContent-Type: image/jpeg\r\n\r\n[Binary data]\r\n--boundary_string--\r\n";

    let reader = FORM_DATA.as_bytes();
    let multipart = Multipart::with_reader(reader, "boundary_string");

    let form = MultipartForm::with_multipart(multipart).await.unwrap();
    let person = Person::from_multipart(form).unwrap();

    assert_eq!(person.name, "John Smith");
    assert_eq!(person.email, "john@example.com");
    assert_eq!(person.age, 25);
    assert_eq!(person.married, true);

    let str = String::from_utf8(person.photo.bytes().to_vec()).unwrap();
    assert_eq!(str, "[Binary data]");
}
