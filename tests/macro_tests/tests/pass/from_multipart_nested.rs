use multer_derive::FromMultipart;

#[derive(FromMultipart)]
struct MyStruct {
    text: String,
    nested: Nested
}

#[derive(FromMultipart)]
struct Nested {
    value: String
}

fn main() {}
