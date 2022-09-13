#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct {{ rust_name }}Reports {
{%- for (report_id, report_spec) in model.reports.iter() %}
    pub {{report_id}}: Option<{{ (report_spec, model)|rust_report_type }}>,
{%- endfor -%}
}
