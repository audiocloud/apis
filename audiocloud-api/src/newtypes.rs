//! Various IDs and wrappers

use std::fmt::Formatter;
use std::marker::PhantomData;
use std::str::FromStr;

use derive_more::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Display, Constructor)]
#[display(fmt = "{manufacturer}/{name}/{instance}")]
pub struct FixedInstanceId {
    pub manufacturer: String,
    pub name:         String,
    pub instance:     u64,
}

impl FixedInstanceId {
    pub fn model_id(&self) -> ModelId {
        ModelId { manufacturer: self.manufacturer.to_string(),
                  name:         self.name.to_string(), }
    }

    pub fn from_model_id(model_id: ModelId, instance: u64) -> Self {
        let ModelId { manufacturer, name } = model_id;
        Self::new(manufacturer, name, instance)
    }
}

impl<'de> Deserialize<'de> for FixedInstanceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let err = |msg| serde::de::Error::custom(msg);

        let s = String::deserialize(deserializer)?;
        let mut s = s.split('/');
        let manufacturer = s.next().ok_or(err("expected manufacturer"))?;
        let name = s.next().ok_or(err("expected manufacturer"))?;
        let instance: usize = s.next()
                               .ok_or(err("expected instance"))?
                               .parse()
                               .map_err(|_| err("instance is not a number"))?;

        Ok(Self { manufacturer: manufacturer.to_string(),
                  name:         name.to_string(),
                  instance:     instance as u64, })
    }
}

impl Serialize for FixedInstanceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}/{}/{}", &self.manufacturer, &self.name, &self.instance))
    }
}

#[derive(Clone, Debug, Display, Eq, PartialEq, Hash, Constructor)]
#[display(fmt = "{manufacturer}/{name}")]
pub struct ModelId {
    pub manufacturer: String,
    pub name:         String,
}

impl ModelId {
    pub fn instance(self, instance: u64) -> FixedInstanceId {
        FixedInstanceId::from_model_id(self, instance)
    }
}

impl From<(String, String)> for ModelId {
    fn from((manufacturer, name): (String, String)) -> Self {
        Self::new(manufacturer, name)
    }
}

impl<'de> Deserialize<'de> for ModelId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(Tuple2Visitor::new())
    }
}

impl Serialize for ModelId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}/{}", &self.manufacturer, &self.name))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, IsVariant)]
#[serde(rename_all = "snake_case")]
pub enum FilterId {
    HighPass,
    Low,
    LowMid,
    Mid,
    HighMid,
    High,
    LowPass,
    BandPass,
    Dynamics,
    DeEsser,
}

struct Tuple2Visitor<K, V, T>(PhantomData<K>, PhantomData<V>, PhantomData<T>);

impl<K, V, T> Tuple2Visitor<K, V, T> {
    pub fn new() -> Self {
        Self(PhantomData, PhantomData, PhantomData)
    }
}

impl<'de, K, V, T> serde::de::Visitor<'de> for Tuple2Visitor<K, V, T>
    where T: From<(K, V)>,
          K: From<String>,
          V: From<String>
{
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("Expected string of format string/string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: serde::de::Error
    {
        let mut split = v.split('/');
        let manufacturer = split.next().ok_or(E::custom("could not extract first string"))?;
        let name = split.next().ok_or(E::custom("could not extract second string"))?;

        Ok(T::from((K::from(manufacturer.to_string()), V::from(name.to_string()))))
    }
}

/// Track in a session
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct TrackId(String);

/// Media item on a track
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct MediaId(String);

/// Mixer in a session
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct MixerId(String);

/// Input of a mixer
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct InputId(String);

/// Dynamic instance in a session
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct DynamicId(String);

/// Fixed instance in a session
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct FixedId(String);

/// App
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct AppId(String);

/// Session of an App on a Domain
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct SessionId(String);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Constructor, Hash, From)]
#[display(fmt = "{app_id}/{session_id}")]
pub struct AppSessionId {
    pub app_id:     AppId,
    pub session_id: SessionId,
}

impl FromStr for AppSessionId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_value(serde_json::Value::String(s.to_string()))?)
    }
}

impl<'de> Deserialize<'de> for AppSessionId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(Tuple2Visitor::new())
    }
}

impl Serialize for AppSessionId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}/{}", &self.app_id, &self.session_id))
    }
}

/// Media of an App
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct MediaObjectId(String);

/// A password for session control
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct SecureKey(String);

/// Domain
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct DomainId(String);

/// Parameter of a model
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct ParameterId(String);

/// Report of a model
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct ReportId(String);
