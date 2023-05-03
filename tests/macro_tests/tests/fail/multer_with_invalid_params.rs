use multer_derive::{Error, FromMultipart, MultipartForm};

#[derive(FromMultipart)]
struct MyStruct {
    #[multer(with = "text_from_multipart")]
    text: Text,
    number: u32,
}

struct Text(String);

fn text_from_multipart(_multipart: &MultipartForm, _ctx: u32) -> Result<Text, Error> {
    todo!()
}

fn main() {}
