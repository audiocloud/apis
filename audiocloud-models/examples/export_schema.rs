use std::fs;

fn main() {
    // we generate a schema.json to be included in various builds
    fs::write("schema.json",
              serde_json::to_string(&audiocloud_models::schemas()).expect("to schemas")).expect("write schema.json");
}
