use crate::{SmartString, SmartStringMode};
use std::{fmt, marker::PhantomData};

use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

impl<T: SmartStringMode> Serialize for SmartString<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self)
    }
}

impl<'de, T: SmartStringMode> Deserialize<'de> for SmartString<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_string(SmartStringVisitor(PhantomData))
            .map(SmartString::from)
    }
}

struct SmartStringVisitor<T: SmartStringMode>(PhantomData<*const T>);

impl<'de, T: SmartStringMode> Visitor<'de> for SmartStringVisitor<T> {
    type Value = SmartString<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a string")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(SmartString::from(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(SmartString::from(v))
    }
}