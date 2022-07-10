//! Various IDs and wrappers

use std::fmt::Formatter;

use derive_more::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

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
    let instance: usize = s.next().ok_or(err("expected instance"))?.parse().map_err(|_| err("instance is not a number"))?;

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

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Display, Deref, Hash, Constructor)]
pub struct ParameterId(String);

impl AsRef<str> for ParameterId {
  fn as_ref(&self) -> &str {
    self.0.as_ref()
  }
}

impl From<&str> for ParameterId {
  fn from(v: &str) -> Self {
    Self::new(v.to_owned())
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Display, Deref, Hash, Constructor)]
pub struct ReportId(String);

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

impl From<&str> for ModelId {
  fn from(value: &str) -> Self {
    Self::from(value.to_owned())
  }
}

impl From<String> for ModelId {
  fn from(s: String) -> Self {
    serde_json::from_value(Value::String(s)).expect("key correctly formed")
  }
}

impl<'de> Deserialize<'de> for ModelId {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
  {
    deserializer.deserialize_str(ModelIdVisitor)
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

struct ModelIdVisitor;

impl<'de> serde::de::Visitor<'de> for ModelIdVisitor {
  type Value = ModelId;

  fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
    formatter.write_str("Expected string of format manufacturer/name")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where E: serde::de::Error
  {
    let mut split = v.split('/');
    let manufacturer = split.next().ok_or(E::custom("could not extract manufacturer"))?;
    let name = split.next().ok_or(E::custom("could not extract name"))?;

    Ok(ModelId { manufacturer: manufacturer.to_string(),
                 name:         name.to_string(), })
  }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct TrackId(String);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct MediaId(String);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct MixerId(String);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct InputId(String);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct DynamicId(String);

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug, From, Into, Hash, Display, Constructor)]
#[repr(transparent)]
pub struct FixedId(String);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From)]
pub struct AppId(String);

impl Default for AppId {
  fn default() -> Self {
    Self::new("<unknown>".to_owned())
  }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From, FromStr)]
pub struct SessionId(String);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From)]
pub struct MediaObjectId(String);

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Display, Deref, Constructor, Hash, From)]
pub struct SecureKey(String);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Deref, Display, Hash, Constructor, From, FromStr)]
pub struct DomainId(String);

impl From<&str> for DomainId {
  fn from(value: &str) -> Self {
    Self::new(value.to_owned())
  }
}
