use multer_derive::FromMultipart;

#[derive(FromMultipart)]
struct MyStruct {
    text: String,
    other: OtherType
}

struct OtherType {
    value: String
}

fn main() {}
