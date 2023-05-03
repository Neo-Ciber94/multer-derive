use crate::{
    error::Error, from_multipart_field::FormMultipartField, multipart_form::MultipartForm,
};
use std::{
    collections::{BTreeMap, HashMap},
    hash::Hash,
    str::FromStr,
};

/// Allows to create a type from a `MultipartForm`.
pub trait FromMultipart: Sized {
    fn from_multipart(multipart: MultipartForm) -> Result<Self, Error>;
}

impl<K, V> FromMultipart for HashMap<K, V>
where
    K: FromStr + Hash + Eq + Send,
    V: FormMultipartField + Send,
    K::Err: std::error::Error + Send + Sync + 'static,
{
    fn from_multipart(multipart: MultipartForm) -> Result<Self, Error> {
        let mut map = HashMap::new();

        for field in multipart.fields() {
            let Some(name) = field.name() else {
                continue;
            };

            let key = K::from_str(name).map_err(Error::new)?;
            let value = V::from_field(field, &multipart)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<K, V> FromMultipart for BTreeMap<K, V>
where
    K: FromStr + Ord + Send,
    V: FormMultipartField + Send,
    K::Err: std::error::Error + Send + Sync + 'static,
{
    fn from_multipart(multipart: MultipartForm) -> Result<Self, Error> {
        let mut map = BTreeMap::new();

        for field in multipart.fields() {
            let Some(name) = field.name() else {
                continue;
            };

            let key = K::from_str(name).map_err(Error::new)?;
            let value = V::from_field(field, &multipart)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use multer::Multipart;

    use crate::{FormFile, FromMultipart, MultipartForm};

    struct Person {
        name: String,
        email: String,
        age: u8,
        married: bool,
        photo: FormFile,
    }

    impl FromMultipart for Person {
        fn from_multipart(multipart: crate::MultipartForm) -> Result<Self, crate::Error> {
            let name = multipart
                .get_by_name("name")
                .ok_or_else(|| crate::Error::new(format!("`name` field was not found")))
                .and_then(|f| <String as crate::FormMultipartField>::from_field(f, &multipart))?;

            let email = multipart
                .get_by_name("email")
                .ok_or_else(|| crate::Error::new(format!("`email` form field was not found")))
                .and_then(|f| <String as crate::FormMultipartField>::from_field(f, &multipart))?;

            let age = multipart
                .get_by_name("age")
                .ok_or_else(|| crate::Error::new(format!("`age` form field was not found")))
                .and_then(|f| <u8 as crate::FormMultipartField>::from_field(f, &multipart))?;

            let married = multipart
                .get_by_name("married")
                .ok_or_else(|| crate::Error::new(format!("`married` form field was not found")))
                .and_then(|f| <bool as crate::FormMultipartField>::from_field(f, &multipart))?;

            let photo = multipart
                .get_by_name("photo")
                .ok_or_else(|| crate::Error::new(format!("`photo` form field was not found")))
                .and_then(|f| <FormFile as crate::FormMultipartField>::from_field(f, &multipart))?;

            Ok(Self {
                name,
                email,
                age,
                married,
                photo,
            })
        }
    }

    #[tokio::test]
    async fn parse_struct_test() {
        const FORM_DATA : &str = "--boundary_string\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nJohn Smith\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\njohn@example.com\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"age\"\r\n\r\n25\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"married\"\r\n\r\ntrue\r\n--boundary_string\r\nContent-Disposition: form-data; name=\"photo\"; filename=\"example.jpg\"\r\nContent-Type: image/jpeg\r\n\r\n[Binary data]\r\n--boundary_string--\r\n";

        let reader = FORM_DATA.as_bytes();
        let multipart = Multipart::with_reader(reader, "boundary_string");

        let form = MultipartForm::with_multipart(multipart).await.unwrap();
        let person = Person::from_multipart(form).unwrap();

        assert_eq!(person.name, "John Smith");
        assert_eq!(person.email, "john@example.com");
        assert_eq!(person.age, 25);
        assert_eq!(person.married, true);

        let str = String::from_utf8(person.photo.bytes().to_vec()).unwrap();
        assert_eq!(str, "[Binary data]");
    }
}
