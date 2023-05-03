use crate::MultipartForm;
use crate::{error::Error, multipart_form::MultipartField};
use std::collections::{BinaryHeap, LinkedList};
use std::hash::Hash;
use std::str::FromStr;
use std::{
    collections::{HashSet, VecDeque},
    ffi::OsString,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6},
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroU128, NonZeroU16,
        NonZeroU32, NonZeroU64, NonZeroU8,
    },
    path::PathBuf,
};

/// Allows to create a field from a [`multer::Field`].
pub trait FromMultipartField: Sized {
    /// Parses the value of the given form field.
    /// 
    /// # Params
    /// - `field` the field to parse
    /// - `form` the form being parsed
    fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error>;
}

impl<T: FromMultipartField> FromMultipartField for Option<T> {
    fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        match T::from_field(field, form) {
            Ok(x) => Ok(Some(x)),
            Err(_) => Ok(None),
        }
    }
}

impl<T> FromMultipartField for Result<T, Error>
where
    T: FromMultipartField,
{
    fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        match T::from_field(field, form) {
            Ok(x) => Ok(Ok(x)),
            Err(err) => Ok(Err(err)),
        }
    }
}

impl<K, V> FromMultipartField for (K, V)
where
    K: FromStr,
    V: FromMultipartField,
    K::Err: std::error::Error + Send + Sync + 'static,
{
    fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        let name = field
            .name()
            .ok_or_else(|| Error::new("field does not had a name"))?;
        let key = K::from_str(name).map_err(Error::new)?;
        let value = V::from_field(field, form)?;
        Ok((key, value))
    }
}

impl<T: FromMultipartField> FromMultipartField for Vec<T> {
    fn from_field(_: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        let mut matches = vec![];

        // We get all the files that can be converted into `T`
        for field in form.fields() {
            if let Ok(x) = T::from_field(field, form) {
                matches.push(x);
            }
        }

        Ok(matches)
    }
}

impl<T: FromMultipartField> FromMultipartField for VecDeque<T> {
    fn from_field(_: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        let mut matches = VecDeque::new();

        // We get all the files that can be converted into `T`
        for field in form.fields() {
            if let Ok(x) = T::from_field(field, form) {
                matches.push_front(x);
            }
        }

        Ok(matches)
    }
}

impl<T: FromMultipartField> FromMultipartField for LinkedList<T> {
    fn from_field(_: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        let mut matches = LinkedList::new();

        // We get all the files that can be converted into `T`
        for field in form.fields() {
            if let Ok(x) = T::from_field(field, form) {
                matches.push_front(x);
            }
        }

        Ok(matches)
    }
}

impl<T> FromMultipartField for BinaryHeap<T>
where
    T: FromMultipartField + Ord,
{
    fn from_field(_: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        let mut matches = BinaryHeap::new();

        // We get all the files that can be converted into `T`
        for field in form.fields() {
            if let Ok(x) = T::from_field(field, form) {
                matches.push(x);
            }
        }

        Ok(matches)
    }
}

impl<T> FromMultipartField for HashSet<T>
where
    T: FromMultipartField + Hash + Eq,
{
    fn from_field(_: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
        let mut matches = HashSet::new();

        // We get all the files that can be converted into `T`
        for field in form.fields() {
            if let Ok(x) = T::from_field(field, form) {
                matches.insert(x);
            }
        }

        Ok(matches)
    }
}

#[macro_export]
macro_rules! from_field_impls {
    ($($t:ty),*) => {
        $(
            impl FromMultipartField for $t {
                fn from_field(field: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
                    let text = field.text();
                    text.parse().map_err(Error::new)
                }
            }
        )*
    };
}

from_field_impls!(
    bool, char, f32, f64, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, OsString,
    String
);

from_field_impls!(
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI128,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128
);

from_field_impls!(
    PathBuf,
    Ipv4Addr,
    Ipv6Addr,
    SocketAddrV4,
    SocketAddrV6,
    IpAddr,
    SocketAddr
);

#[cfg(feature = "time")]
mod time {
    use super::FromMultipartField;
    use crate::{error::Error, multipart_form::MultipartField, MultipartForm};

    impl FromMultipartField for time::Time {
        fn from_field(field: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
            let format =
                time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:9]");
            let text = field.text();
            let time = time::Time::parse(&text, format).map_err(Error::new)?;
            Ok(time)
        }
    }

    impl FromMultipartField for time::Date {
        fn from_field(field: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
            let format = time::macros::format_description!("[year]-[month]-[day]");
            let text = field.text();
            let date = time::Date::parse(&text, format).map_err(Error::new)?;
            Ok(date)
        }
    }

    impl FromMultipartField for time::PrimitiveDateTime {
        fn from_field(field: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
            let format = time::macros::format_description!(
                "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:9]"
            );
            let text = field.text();
            let date = time::PrimitiveDateTime::parse(&text, format).map_err(Error::new)?;
            Ok(date)
        }
    }
}

#[cfg(feature = "uuid")]
mod uuid {
    use super::FromMultipartField;
    use crate::{error::Error, multipart_form::MultipartField, MultipartForm};
    use uuid::Uuid;

    impl FromMultipartField for Uuid {
        fn from_field(field: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
            let text = field.text();
            text.parse().map_err(Error::new)
        }
    }
}

#[cfg(feature = "json")]
mod json {
    use super::FromMultipartField;
    use crate::{error::Error, multipart_form::MultipartField, MultipartForm};

    impl FromMultipartField for serde_json::Value {
        fn from_field(field: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
            let text = field.text();
            serde_json::to_value(&text).map_err(Error::new)
        }
    }
}

mod misc {
    use std::{
        cell::{Cell, RefCell},
        marker::PhantomData,
        num::Wrapping,
        rc::Rc,
        sync::{Arc, Mutex, RwLock},
    };

    use crate::{Error, FromMultipartField, MultipartField, MultipartForm};

    impl FromMultipartField for () {
        fn from_field(_: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
            Ok(())
        }
    }

    impl<T> FromMultipartField for PhantomData<T> {
        fn from_field(_: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
            Ok(PhantomData)
        }
    }

    impl<T: FromMultipartField> FromMultipartField for Box<T> {
        fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
            T::from_field(field, form).map(Box::new)
        }
    }

    impl<T: FromMultipartField> FromMultipartField for Rc<T> {
        fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
            T::from_field(field, form).map(Rc::new)
        }
    }

    impl<T: FromMultipartField> FromMultipartField for Arc<T> {
        fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
            T::from_field(field, form).map(Arc::new)
        }
    }

    impl<T: FromMultipartField> FromMultipartField for Cell<T> {
        fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
            T::from_field(field, form).map(Cell::new)
        }
    }

    impl<T: FromMultipartField> FromMultipartField for RefCell<T> {
        fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
            T::from_field(field, form).map(RefCell::new)
        }
    }

    impl<T: FromMultipartField> FromMultipartField for Mutex<T> {
        fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
            T::from_field(field, form).map(Mutex::new)
        }
    }

    impl<T: FromMultipartField> FromMultipartField for RwLock<T> {
        fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
            T::from_field(field, form).map(RwLock::new)
        }
    }

    impl<T: FromMultipartField> FromMultipartField for Wrapping<T> {
        fn from_field(field: &MultipartField, form: &MultipartForm) -> Result<Self, Error> {
            T::from_field(field, form).map(Wrapping)
        }
    }
}

mod atomics {
    use crate::{Error, FromMultipartField, MultipartField, MultipartForm};
    use std::sync::atomic::*;

    macro_rules! impl_atomic_field {
        ($($atomic:ident),*) => {
            $(
                impl FromMultipartField for $atomic {
                    fn from_field(field: &MultipartField, _form: &MultipartForm) -> Result<Self, Error> {
                        let text = field.text();
                        let value = text.parse().map_err(Error::new)?;
                        Ok($atomic::new(value))
                    }
                }
            )*
        };
    }

    // Call macro with all atomic types
    impl_atomic_field!(
        AtomicBool,
        AtomicI8,
        AtomicI16,
        AtomicI32,
        AtomicI64,
        AtomicIsize,
        AtomicU8,
        AtomicU16,
        AtomicU32,
        AtomicU64,
        AtomicUsize
    );
}
