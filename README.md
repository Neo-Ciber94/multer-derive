# multer-derive

Allow to implement derive for create types from a multipart form.

## Usage

```rs
#[derive(FormMultipart)]
struct Person {
    name: String,
    email: String,
    age: u8,
    married: bool,
    photo: FormFile
}
```