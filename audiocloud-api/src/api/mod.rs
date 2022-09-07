use schemars::schema::RootSchema;
use serde_json::{json, Value};
use utoipa::openapi::OpenApi;

pub use codec::*;

pub mod codec;

pub fn merge_schemas(x: impl Iterator<Item = RootSchema>) -> RootSchema {
    let mut root = RootSchema::default();
    for schema in x {
        let RootSchema { schema, definitions, .. } = schema;
        let title = schema.metadata.as_ref().unwrap().title.clone().unwrap();

        let title = if title.starts_with("Array_of_") {
            format!("{}List", &title[9..])
        } else {
            title
        };

        root.definitions.extend(definitions.into_iter());
        root.definitions.insert(title, schema.into());
    }

    root
}

pub fn openapi_with_schemas_to_json(api: OpenApi, merged: RootSchema, title: &str) -> anyhow::Result<String> {
    let mut api: serde_json::Value = serde_json::from_str(&api.to_json()?)?;

    let schemas = serde_json::to_value(&merged.definitions)?;

    api.as_object_mut().expect("as object").insert("components".to_string(),
                                                   json!({
                                                       "schemas": schemas,
                                                   }));

    api.as_object_mut()
       .expect("as object")
       .get_mut("info")
       .expect("info")
       .as_object_mut()
       .expect("as_object")
       .insert("title".to_owned(), Value::String(title.to_owned()));

    Ok(serde_json::to_string_pretty(&api)?.replace("#/definitions/", "#/components/schemas/"))
}
