use multer_derive::FromMultipart;

#[derive(FromMultipart)]
struct MyStruct {
    #[multer(rename = "number")]
    text: String,

    #[multer(rename = "text")]
    number: u32,
}

fn main() {}
