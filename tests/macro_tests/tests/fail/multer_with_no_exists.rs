use multer_derive::FromMultipart;

#[derive(FromMultipart)]
struct MyStruct {
    #[multer(with = "not_exists")]
    text: Text,
    number: u32,
}

struct Text(String);

fn main() {}
