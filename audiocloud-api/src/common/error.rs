use anyhow::anyhow;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SerializableResult<T = ()> {
    Ok(T),
    Err { code: usize, message: String },
}

impl<T> Into<anyhow::Result<T>> for SerializableResult<T> {
    fn into(self) -> anyhow::Result<T> {
        match self {
            SerializableResult::Ok(t) => Ok(t),
            SerializableResult::Err { code, message } => Err(anyhow!("Error code {code}: {message}")),
        }
    }
}

impl<T> From<anyhow::Result<T>> for SerializableResult<T> {
    fn from(res: anyhow::Result<T>) -> Self {
        match res {
            Ok(ok) => Self::Ok(ok),
            Err(err) => Self::Err { code:    500,
                                    message: err.to_string(), },
        }
    }
}