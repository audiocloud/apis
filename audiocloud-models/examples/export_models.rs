use audiocloud_api::newtypes::ModelId;
use audiocloud_models::MODELS;
use std::fs::File;

fn main() {
  for (id, model) in &*MODELS {
    let ModelId { manufacturer, name } = id;
    let file_name = format!("generated/{manufacturer}_{name}.yaml",);
    serde_yaml::to_writer(File::create(file_name).expect("open yaml"), model).expect("YAML write");
  }
}
