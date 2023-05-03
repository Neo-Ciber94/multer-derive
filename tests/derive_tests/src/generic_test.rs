use std::borrow::Cow;

use multer_derive::{
    multer::Multipart, FormFile, FromMultipart, FromMultipartField, MultipartForm,
};

#[derive(FromMultipart)]
struct Person<S, N, B> {
    name: S,
    email: S,
    age: N,
    married: B,
    photo: FormFile,
}

#[derive(Debug, PartialEq)]
enum Truthy {
    On,
    Off,
}

impl FromMultipartField for Truthy {
    fn from_field(field: &multer_derive::MultipartField) -> Result<Self, multer_derive::Error> {
        match bool::from_field(field)? {
            true => Ok(Truthy::On),
            false => Ok(Truthy::Off),
        }
    }
}

#[tokio::test]
async fn generic_person_from_multipart_test() {
    const FORM_DATA : &str = "--boundary_string\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nJohn Smith\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\njohn@example.com\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"age\"\r\n\r\n25\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"married\"\r\n\r\ntrue\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"photo\"; filename=\"example.jpg\"\r\nContent-Type: image/jpeg\r\n\r\n[Binary data]\r\n--boundary_string--\r\n";

    let reader = FORM_DATA.as_bytes();
    let multipart = Multipart::with_reader(reader, "boundary_string");

    let form = MultipartForm::with_multipart(multipart).await.unwrap();
    let person: Person<Cow<String>, f32, Truthy> =
        Person::from_multipart(&form, Default::default()).unwrap();

    assert_eq!(person.name.as_str(), "John Smith");
    assert_eq!(person.email.as_str(), "john@example.com");
    assert_eq!(person.age, 25.0);
    assert_eq!(person.married, Truthy::On);

    let str = String::from_utf8(person.photo.bytes().to_vec()).unwrap();
    assert_eq!(str, "[Binary data]");
}
