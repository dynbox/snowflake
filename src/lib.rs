use std::num::ParseIntError;

pub mod internal;
pub mod factory;
pub mod errors;

#[derive(Debug)]
pub struct Snowflake {
    pub id: u64,
}

impl Snowflake {
    pub fn new(id: u64) -> Self {
        Self { id, }
    }

    pub fn from_string<S: Into<String>>(id: S) -> Result<Snowflake, ParseIntError> {
        id.into().parse().map(Snowflake::new)
    }

    pub fn milliseconds(&self) -> u64 {
        self.id >> 22
    }

    pub fn epoch_time(&self, epoch: u64) -> u64 {
        self.milliseconds() + epoch
    }

    pub fn node_id(&self) -> u64 {
        (self.id & 0x3E0000) >> 17
    }

    pub fn worker_id(&self) -> u64 {
        (self.id & 0x1F000) >> 12
    }

    pub fn sequence(&self) -> u64 {
        self.id & 0xFFF
    }
}

#[cfg(feature="derive")]
impl serde::Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(&self.id.to_string())
    }
}

#[cfg(feature="derive")]
struct IDVisitor;

#[cfg(feature="derive")]
impl<'de> serde::de::Visitor<'de> for IDVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a string containing the snowflake id")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value.to_owned())
    }
}

#[cfg(feature="derive")]
impl<'de> serde::Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let id = deserializer.deserialize_string(IDVisitor)?;

        Snowflake::from_string(&id).map_err(|_| {
            serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(&id),
                &"a valid snowflake id in string format",
            )
        })
    }
}