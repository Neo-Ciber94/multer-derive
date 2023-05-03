use multer_derive::{FromMultipart, FormFile};

#[derive(FromMultipart)]
pub struct MyStruct {
    text: String,
    number: u32,
    boolean: bool,
    character: char,
    photo: FormFile,
    all_files: Vec<FormFile>
}

fn main() {}