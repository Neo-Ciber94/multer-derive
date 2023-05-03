use crate::{
    error::Error, from_multipart_field::FromMultipartField, multipart_form::MultipartForm,
};
use std::{
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
    str::FromStr,
};

/// Additional information for parsing a multipart form.
#[derive(Default, Debug)]
pub struct FormContext<'a> {
    /// The name of the field being parsed, if any.
    pub field_name: Option<&'a str>,
}

/// Allows to create a type from a [`multer::Multipart`].
pub trait FromMultipart: Sized {
    /// Constructs this type from the given multipart form.
    fn from_multipart(multipart: &MultipartForm, ctx: FormContext<'_>) -> Result<Self, Error>;
}

impl<T: FromMultipartField> FromMultipart for T {
    fn from_multipart(multipart: &MultipartForm, ctx: FormContext<'_>) -> Result<Self, Error> {
        let Some(field_name) = ctx.field_name else {
            return Err(Error::new("FormContext does not specified a field to parse"));
        };

        let field = multipart
            .get_by_name(field_name)
            .ok_or_else(|| Error::new(format!("`{field_name}` form field was not found")))?;

        T::from_field(field)
    }
}

impl<K, V> FromMultipart for HashMap<K, V>
where
    K: FromStr + Hash + Eq + Send,
    V: FromMultipartField + Send,
    K::Err: std::error::Error + Send + Sync + 'static,
{
    fn from_multipart(multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<Self, Error> {
        let mut map = HashMap::new();

        for field in multipart.fields() {
            let Some(name) = field.name() else {
                continue;
            };

            let key = K::from_str(name).map_err(Error::new)?;
            let value = V::from_field(field)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<K, V> FromMultipart for BTreeMap<K, V>
where
    K: FromStr + Ord + Send,
    V: FromMultipartField + Send,
    K::Err: std::error::Error + Send + Sync + 'static,
{
    fn from_multipart(multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<Self, Error> {
        let mut map = BTreeMap::new();

        for field in multipart.fields() {
            let Some(name) = field.name() else {
                continue;
            };

            let key = K::from_str(name).map_err(Error::new)?;
            let value = V::from_field(field)?;
            map.insert(key, value);
        }

        Ok(map)
    }
}

impl<T: FromMultipartField> FromMultipart for Vec<T> {
    fn from_multipart(multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<Self, Error> {
        Ok(multipart
            .fields()
            .iter()
            .filter_map(|f| T::from_field(f).ok())
            .collect())
    }
}

impl<T: FromMultipartField> FromMultipart for VecDeque<T> {
    fn from_multipart(multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<Self, Error> {
        Ok(multipart
            .fields()
            .iter()
            .filter_map(|f| T::from_field(f).ok())
            .collect())
    }
}

impl<T: FromMultipartField> FromMultipart for LinkedList<T> {
    fn from_multipart(multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<Self, Error> {
        Ok(multipart
            .fields()
            .iter()
            .filter_map(|f| T::from_field(f).ok())
            .collect())
    }
}

impl<T: FromMultipartField + Hash + Eq> FromMultipart for HashSet<T> {
    fn from_multipart(multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<Self, Error> {
        Ok(multipart
            .fields()
            .iter()
            .filter_map(|f| T::from_field(f).ok())
            .collect())
    }
}

impl<T: FromMultipartField + Ord> FromMultipart for BinaryHeap<T> {
    fn from_multipart(multipart: &MultipartForm, _ctx: FormContext<'_>) -> Result<Self, Error> {
        Ok(multipart
            .fields()
            .iter()
            .filter_map(|f| T::from_field(f).ok())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use multer::Multipart;

    use crate::{FormFile, FromMultipart, MultipartForm};

    use super::FormContext;

    struct Person {
        name: String,
        email: String,
        age: u8,
        married: bool,
        photo: FormFile,
    }

    impl FromMultipart for Person {
        fn from_multipart(
            multipart: &crate::MultipartForm,
            _ctx: FormContext<'_>,
        ) -> Result<Self, crate::Error> {
            let name = <String as crate::FromMultipart>::from_multipart(
                multipart,
                FormContext {
                    field_name: Some("name"),
                },
            )?;

            let email = <String as crate::FromMultipart>::from_multipart(
                multipart,
                FormContext {
                    field_name: Some("email"),
                },
            )?;

            let age = <u8 as crate::FromMultipart>::from_multipart(
                multipart,
                FormContext {
                    field_name: Some("age"),
                },
            )?;

            let married = <bool as crate::FromMultipart>::from_multipart(
                multipart,
                FormContext {
                    field_name: Some("married"),
                },
            )?;

            let photo = <FormFile as crate::FromMultipart>::from_multipart(
                multipart,
                FormContext {
                    field_name: Some("photo"),
                },
            )?;

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
        let person = Person::from_multipart(&form, Default::default()).unwrap();

        assert_eq!(person.name, "John Smith");
        assert_eq!(person.email, "john@example.com");
        assert_eq!(person.age, 25);
        assert_eq!(person.married, true);

        let str = String::from_utf8(person.photo.bytes().to_vec()).unwrap();
        assert_eq!(str, "[Binary data]");
    }
}
