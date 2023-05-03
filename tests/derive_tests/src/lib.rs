#![cfg(test)]

use multer_derive::{FormFile, FromMultipart};

#[derive(FromMultipart)]
struct Person {
    name: String,
    email: String,
    age: u8,
    married: bool,
    photo: FormFile,
}
