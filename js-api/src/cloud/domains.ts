import { Static, Type } from "@sinclair/typebox";
import Option from "../utils/option";
import { Model, ResourceId } from "../model";
import { AppId, AppSessionId, DomainId, FixedInstanceId, ModelId } from "../new_types";
import { JsonSession } from "../session";
import { Maintenance } from "./apps";

export const DynamicInstanceLimits = Type.Object({
    max_instances:      Type.Integer()
})
export type DynamicInstanceLimits = Static<typeof DynamicInstanceLimits>

export const DomainLimits = Type.Object({
    max_sessions:       Type.Integer(),
    resources:          Type.Record(ResourceId, Type.Number())
})
export type DomainLimits = Static<typeof DomainLimits>

export const DomainPowerInstanceSettings = Type.Object({
    warm_up_ms:         Type.Integer(),
    cool_down_ms:       Type.Integer(),
    idle_off_delay_ms:  Type.Integer(),
    instance:           FixedInstanceId,
    channel:            Type.Integer(),
})
export type DomainPowerInstanceSettings = Static<typeof DomainPowerInstanceSettings>

export const DomainMediaInstanceSettings = Type.Object({
    length:             Type.Number(),
    rewind_to_start:    Type.Boolean()
})
export type DomainMediaInstanceSettings = Static<typeof DomainMediaInstanceSettings>

export const DomainFixedInstance = Type.Object({
    input_start:        Option(Type.Integer()),
    output_start:       Option(Type.Integer()),
    sidecars:           Type.Array(ModelId),
    power:              Option(DomainPowerInstanceSettings),
    media:              Option(DomainMediaInstanceSettings),
    apps:               Type.Array(AppId),
    maintenance:        Type.Array(Maintenance),
})
export type DomainFixedInstance = Static<typeof DomainFixedInstance>

export const BootDomain = Type.Object({
    domain_id:          DomainId,
    event_base:         Type.Integer(),
    fixed_instances:    Type.Record(FixedInstanceId, DomainFixedInstance),
    dynamic_instances:  Type.Record(ModelId, DynamicInstanceLimits),
    sessions:           Type.Record(AppSessionId, JsonSession),
    models:             Type.Record(ModelId, Model),
    domain_limits:      DomainLimits,
    min_session_len:    Type.Number(),
    native_sample_rate: Type.Integer(),
    public_url:         Type.String(),
    cmd_topic:          Type.String(),
    evt_topic:          Type.String(),
    kafka_url:          Type.String(),
    consume_username:   Type.String(),
    consume_password:   Type.String(),
    produce_username:   Type.String(),
    produce_password:   Type.String(),
})



