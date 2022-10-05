{%- for (property_id, property_spec) in model.parameters.iter() %}
pub const {{property_id|screaming_snake}}_NAME: &str = "{{ property_id }}";
pub const {{property_id|screaming_snake}}_VALUES: [ModelValueOption; {{ property_spec.values.len() }}] = {{ ModelValueOptionsTemplate::new(property_spec.values) }};
{%- endfor -%}