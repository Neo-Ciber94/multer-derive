use multer_derive::FromMultipart;

#[derive(FromMultipart)]
struct MyStruct {
    text: String,

    #[multer(with = "utils::other_from_multipart")]
    other: OtherType,
}

pub struct OtherType {
    value: String
}

mod utils {
    use super::OtherType;
    use multer_derive::{MultipartForm, FormContext, Error};

    pub fn other_from_multipart(_multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<OtherType, Error> {
        todo!()
    }
}

fn main() {}
