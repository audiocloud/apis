//! Various IDs and wrappers

use std::fmt::Formatter;
use std::marker::PhantomData;
use std::str::FromStr;

use crate::cloud::CloudError;
use derive_more::*;
use once_cell::sync::OnceCell;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Display, Constructor)]
#[display(fmt = "{manufacturer}/{name}/{instance}")]
pub struct FixedInstanceId {
    pub manufacturer: String,
    pub name:         String,
    pub instance:     String,
}

impl FixedInstanceId {
    pub fn model_id(&self) -> ModelId {
        ModelId { manufacturer: self.manufacturer.to_string(),
                  name:         self.name.to_string(), }
    }

    pub fn from_model_id(model_id: ModelId, instance: String) -> Self {
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
        let instance = s.next().ok_or(err("expected instance"))?;

        Ok(Self { manufacturer: manufacturer.to_string(),
                  name:         name.to_string(),
                  instance:     instance.to_string(), })
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
    pub fn instance(self, instance: String) -> FixedInstanceId {
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

/// Dynamic instance in a session
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct DynamicId(String);

/// Fixed instance in a session
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct FixedId(String);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct ConnectionId(String);

/// App
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct AppId(String);

impl AppId {
    pub fn is_admin(&self) -> bool {
        self.0 == "admin"
    }

    pub fn admin() -> AppId {
        AppId("admin".to_string())
    }
}

/// Session of an App on a Domain
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct SessionId(String);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct SocketId(String);

impl SessionId {
    pub fn validate(self) -> Result<Self, CloudError> {
        static VALIDATION: OnceCell<Regex> = OnceCell::new();

        VALIDATION.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap())
                  .find(&self.0)
                  .ok_or_else(|| CloudError::InvalidSessionId(self.to_string()))?;

        Ok(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Constructor, Hash, From)]
#[display(fmt = "{app_id}/{session_id}")]
pub struct AppSessionId {
    pub app_id:     AppId,
    pub session_id: SessionId,
}

impl FromStr for AppSessionId {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_string()))
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

impl MediaObjectId {
    pub fn validate(self) -> Result<Self, CloudError> {
        static VALIDATION: OnceCell<Regex> = OnceCell::new();

        VALIDATION.get_or_init(|| Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap())
                  .find(&self.0)
                  .ok_or_else(|| CloudError::InvalidMediaId(self.to_string()))?;

        Ok(self)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Display, Constructor, Hash)]
#[display(fmt = "{app_id}/{media_id}")]
pub struct AppMediaObjectId {
    pub app_id:   AppId,
    pub media_id: MediaObjectId,
}

impl From<(AppId, MediaObjectId)> for AppMediaObjectId {
    fn from((app_id, media_id): (AppId, MediaObjectId)) -> Self {
        Self::new(app_id, media_id)
    }
}

impl FromStr for AppMediaObjectId {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_value(Value::String(s.to_string()))
    }
}

impl Serialize for AppMediaObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&format!("{}/{}", &self.app_id, &self.media_id))
    }
}

impl<'de> Deserialize<'de> for AppMediaObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        deserializer.deserialize_str(Tuple2Visitor::new())
    }
}
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

impl From<&str> for ParameterId {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}

/// Report of a model
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
#[repr(transparent)]
pub struct ReportId(String);

impl From<&str> for ReportId {
    fn from(s: &str) -> Self {
        Self::new(s.to_string())
    }
}
