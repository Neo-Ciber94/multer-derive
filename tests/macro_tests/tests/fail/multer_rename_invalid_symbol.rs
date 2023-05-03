use multer_derive::FromMultipart;

#[derive(FromMultipart)]
struct MyStruct {
    #[multer(rename = "1")]
    text: String,

    #[multer(rename = "$number")]
    number: u32,
}

fn main() {}
