use multer_derive::{Error, FromMultipart, MultipartForm, FormContext};

#[derive(FromMultipart)]
struct MyStruct {
    #[multer(with = "text_from_multipart")]
    text: Text,
    number: u32,
}

struct Text(String);

fn text_from_multipart(_multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<Text, ()> {
    todo!()
}

fn main() {}
