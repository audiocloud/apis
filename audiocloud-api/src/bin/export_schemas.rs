use std::fs;
use utoipa::OpenApi;

use audiocloud_api::{merge_schemas, openapi_with_schemas_to_json};

fn export_cloud_openapi() {
    use audiocloud_api::cloud::*;

    fs::write("openapi_cloud.json",
              openapi_with_schemas_to_json(CloudApi::openapi(), schemas(), "Audio Cloud Orchestrator").expect("API convert to JSON")).expect("Write JSON to file");
}

fn export_audio_engine_openapi() {
    use audiocloud_api::audio_engine::*;

    fs::write("openapi_audio_engine.json",
              openapi_with_schemas_to_json(AudioEngineApi::openapi(), schemas(), "Audio Cloud Audio Engine").expect("API convert to JSON")).expect("Write JSON to file");
}

fn export_instance_driver_openapi() {
    use audiocloud_api::instance_driver::*;

    fs::write("openapi_instance_driver.json",
              openapi_with_schemas_to_json(InstanceDriverApi::openapi(), schemas(), "Audio Cloud Instance Driver").expect("API convert to JSON")).expect("Write JSON to file");
}

fn export_domain_openapi() {
    use audiocloud_api::domain::*;

    fs::write("openapi_domain.json",
              openapi_with_schemas_to_json(DomainApi::openapi(), schemas(), "Audio Cloud Domain").expect("API convert to JSON")).expect("Write JSON to file");
}

fn main() {
    export_cloud_openapi();
    export_audio_engine_openapi();
    export_instance_driver_openapi();
    export_domain_openapi();
}
