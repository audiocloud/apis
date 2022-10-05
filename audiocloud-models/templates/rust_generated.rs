use audiocloud_api::model::*;
use audiocloud_api::api::*;
use serde::{Serialize, Deserialize};
use schemars::{JsonSchema, schema_for};
use schemars::schema::RootSchema;

{% for (manufacturer, this_models) in models.iter() %}
pub mod {{ manufacturer|lowercase }} {

use super::*;
{% for (name, model) in this_models.iter() %}
{{ RustPresetModelTemplate::new(name, model) }}
{{ RustParamsModelTemplate::new(name, model) }}
{{ RustReportsModelTemplate::new(name, model) }}
{{ RustConstantsTemplate::new(model) }}
{% endfor %}
}
{% endfor %}

pub fn schemas() -> RootSchema {
    merge_schemas([
{%- for (manufacturer, this_models) in models.iter() %}
{%- for (name, model) in this_models.iter() %}
      schema_for!(self::{{manufacturer|lowercase}}::{{name|pascal_case}}Preset),
      schema_for!(self::{{manufacturer|lowercase}}::{{name|pascal_case}}Parameters),
      schema_for!(self::{{manufacturer|lowercase}}::{{name|pascal_case}}Reports),
{%- endfor %}
{%- endfor %}
    ].into_iter())
}
