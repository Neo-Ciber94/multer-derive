# multer-derive

[![CI-badge]](ci) [![Latest Version]][crates.io] [![Docs Badge]][docs]

[CI-badge]: https://github.com/Neo-Ciber94/multer-derive/actions/workflows/ci.yml/badge.svg
[ci]: <https://github.com/Neo-Ciber94/multer-derive/actions/workflows/ci.yml>

[Latest Version]: https://img.shields.io/crates/v/multer-derive.svg
[crates.io]: https://crates.io/crates/rust-decimal

[Docs Badge]: https://docs.rs/multer-derive/badge.svg
[docs]: https://docs.rs/multer-derive/latest

Provides a `FromMultipart` derive for construct types from [multer::Multipart](https://docs.rs/multer/2.1.0/multer/struct.Multipart.html).

## Usage

```rs
use multer_derive::{FormFile, FromMultipart, Multipart, MultipartForm};

#[derive(FormMultipart)]
struct Person {
    name: String,
    email: String,
    age: u8,
    married: bool,
    photo: FormFile
}

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
```

## Attributes

`multer-derive` also support the next attributes to decorate your fields:

- To rename the target you can use `#[multer(rename = "new_field_name")]`
  - This will make multer to parse the field using the given name.

Example:

```rs
use multer_derive::FromMultipart;

#[derive(FromMultipart)]
struct MyStruct {
    #[multer(rename = "active")]
    is_active: bool
}
```

- To parse using a function you can use `#[multer(with = "path::to::function")]`

  - For this you should provide a function with the signature:

  ```rs
  fn from_multipart(multipart: &MultipartForm, ctx: FormContext<'_>) -> Result<YourType, Error> {
    todo!()
  }
  ```

Example:

```rs
use multer_derive::{FromMultipart, MultipartForm, FormContext, Error};

#[derive(FromMultipart)]
struct MyStruct {
    #[multer(with = "text_from_multipart")]
    name: Text
}

struct Text(String);

fn text_from_multipart(
    multipart: &MultipartForm,
    ctx: FormContext<'_>,
) -> Result<Text, Error> {
    // This is safe, the `field_name` is always passed
    let field_name = ctx.field_name.unwrap();

    // We search the field in the source multipart
    let field = multipart
        .get_by_name(field_name)
        .ok_or(Error::new(format!(
            "`{field_name}` form field was not found"
        )))?;

    // Parse the value using `String`
    let s = String::from_field(field)?;
    Ok(Text(s))
}
```
