use audiocloud_api::{merge_schemas, openapi_with_schemas_to_json};
use schemars::schema_for;
use utoipa::OpenApi;

fn export_cloud_openapi() {
    use audiocloud_api::cloud::*;

    #[derive(OpenApi)]
    #[openapi(paths(apps::get,
                    apps::update,
                    tasks::create,
                    tasks::adjust_time,
                    tasks::delete,
                    tasks::modify_spec,
                    domains::get,
                    domains::boot,
                    domains::add_maintenance,
                    domains::delete_maintenance,
                    domains::add_fixed_instance_maintenance,
                    domains::delete_fixed_instance_maintenance,
                    media::upload_media_object,
                    media::download_media_object,
                    media::delete_media_object,
                    media::report_media_job_progress))]
    pub struct CloudApi;

    let merged = merge_schemas([schema_for!(CloudError),
                                schema_for!(audiocloud_api::ModifyTaskError),
                                schema_for!(audiocloud_api::AppId),
                                schema_for!(audiocloud_api::DomainId),
                                schema_for!(audiocloud_api::TaskId),
                                schema_for!(audiocloud_api::TimeRange),
                                schema_for!(audiocloud_api::TrackNode),
                                schema_for!(audiocloud_api::MixerNode),
                                schema_for!(audiocloud_api::DynamicInstanceNode),
                                schema_for!(audiocloud_api::FixedInstanceNode),
                                schema_for!(audiocloud_api::NodeConnection),
                                schema_for!(audiocloud_api::TaskSecurity),
                                schema_for!(audiocloud_api::TrackMedia),
                                schema_for!(audiocloud_api::TaskSpec),
                                schema_for!(audiocloud_api::ModifyTaskSpec),
                                schema_for!(audiocloud_api::ModifyTask),
                                schema_for!(audiocloud_api::Model),
                                schema_for!(audiocloud_api::MediaJobState),
                                schema_for!(audiocloud_api::UploadToDomain),
                                schema_for!(audiocloud_api::DownloadFromDomain),
                                schema_for!(apps::GetAppResponse),
                                schema_for!(apps::UpdateApp),
                                schema_for!(apps::AppUpdated),
                                schema_for!(tasks::CreateTask),
                                schema_for!(tasks::TaskCreated),
                                schema_for!(tasks::TaskUpdated),
                                schema_for!(tasks::TaskDeleted),
                                schema_for!(tasks::AdjustTaskTime),
                                schema_for!(tasks::ModifyTaskList),
                                schema_for!(domains::DomainMediaInstanceSettings),
                                schema_for!(domains::DomainPowerInstanceSettings),
                                schema_for!(domains::GetDomainResponse),
                                schema_for!(domains::BootDomainResponse),
                                schema_for!(domains::DomainUpdated),
                                schema_for!(domains::AddMaintenance),
                                schema_for!(domains::DeleteMaintenance),
                                schema_for!(domains::Maintenance),
                                schema_for!(domains::AppFixedInstance),
                                schema_for!(domains::DomainFixedInstance),
                                schema_for!(domains::DynamicInstanceLimits),
                                schema_for!(domains::DomainLimits),
                                schema_for!(media::DownloadCreated),
                                schema_for!(media::UploadCreated),
                                schema_for!(media::MediaObjectDeleted),
                                schema_for!(media::ReportMediaJobProgress)].into_iter());

    let json = openapi_with_schemas_to_json(CloudApi::openapi(), merged, "Audio Cloud Orchestrator").expect("API convert to JSON");

    std::fs::write("openapi_cloud.json", json).expect("Write JSON to file");
}

fn export_audio_engine_openapi() {
    use audiocloud_api::audio_engine::*;

    #[derive(OpenApi)]
    #[openapi(paths(tasks::set_spec,
                    tasks::modify_spec,
                    tasks::delete,
                    tasks::list,
                    tasks::play,
                    tasks::seek,
                    tasks::stop_playing,
                    tasks::cancel_render,
                    tasks::render,
                    environment::set_media,
                    environment::set_instances))]
    struct AudioEngineApi;

    let merged = merge_schemas([schema_for!(AudioEngineError),
                                schema_for!(TaskReplaced),
                                schema_for!(TaskDeleted),
                                schema_for!(MediaUpdated),
                                schema_for!(InstancesUpdated),
                                schema_for!(AudioEngineFixedInstance),
                                schema_for!(SetInstances),
                                schema_for!(SetMedia),
                                schema_for!(TaskWithStatusList),
                                schema_for!(TaskWithStatus),
                                schema_for!(SetMedia),
                                schema_for!(SetInstances),
                                schema_for!(audiocloud_api::RequestPlay),
                                schema_for!(audiocloud_api::RequestSeek),
                                schema_for!(audiocloud_api::RequestChangeMixer),
                                schema_for!(audiocloud_api::RequestStopPlay),
                                schema_for!(audiocloud_api::RequestCancelRender),
                                schema_for!(audiocloud_api::ModifyTaskSpec),
                                schema_for!(audiocloud_api::TaskSpec)].into_iter());

    let json = openapi_with_schemas_to_json(AudioEngineApi::openapi(), merged, "Audio Cloud Audio Engine").expect("API convert to JSON");

    std::fs::write("openapi_audio_engine.json", json).expect("Write JSON to file");
}

fn export_instance_driver_openapi() {
    use audiocloud_api::instance_driver::*;

    #[derive(OpenApi)]
    #[openapi(paths())]
    struct InstanceDriverApi;

    let merged = merge_schemas([].into_iter());

    let json = openapi_with_schemas_to_json(InstanceDriverApi::openapi(), merged, "Audio Cloud Audio Engine").expect("API convert to JSON");

    std::fs::write("openapi_instance_driver.json", json).expect("Write JSON to file");
}

fn export_domain_openapi() {
    use audiocloud_api::domain::*;

    #[derive(OpenApi)]
    #[openapi(paths())]
    struct DomainApi;

    let merged = merge_schemas([].into_iter());

    let json = openapi_with_schemas_to_json(DomainApi::openapi(), merged, "Audio Cloud Audio Engine").expect("API convert to JSON");

    std::fs::write("openapi_domain.json", json).expect("Write JSON to file");
}

fn main() {
    export_cloud_openapi();
    export_audio_engine_openapi();
    export_instance_driver_openapi();
    export_domain_openapi();
}
