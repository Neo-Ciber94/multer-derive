use multer_derive::{FromMultipart, FormFile};

#[derive(FromMultipart)]
struct MyStruct {
    #[multer(rename = "number")]
    text: String,

    #[multer(rename = "file[]")]
    files: Vec<FormFile>,
}

fn main() {}
