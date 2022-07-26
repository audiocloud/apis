import { Static, Type } from "@sinclair/typebox";
import { DomainId, DynamicId, FixedId, FixedInstanceId, MixerId, ModelId, SecureKey, TrackId } from "../new_types";
import { SessionTrack, SessionMixer, SessionDynamicInstance, SessionFixedInstance, SessionSecurity } from "../session";
import { JsonTimeRange } from "../time";
import { DomainLimits, DynamicInstanceLimits } from "./domains";

export const Maintenance = Type.Object({
    time:               JsonTimeRange,
    reason:             Type.String(),
})
export type Maintenance = Static<typeof Maintenance>

export const AppFixedInstance = Type.Object({
    power:              Type.Boolean(),
    media:              Type.Boolean(),
    sidecars:           Type.Array(ModelId),
    maintenance:        Type.Array(Maintenance)
})
export type AppFixedInstance = Static<typeof AppFixedInstance>

export const AppDomain = Type.Object({
    fixed_instances:    Type.Record(FixedInstanceId, AppFixedInstance),
    dynamic_instances:  Type.Record(ModelId, DynamicInstanceLimits),
    domain_limits:      DomainLimits,
    min_session_len:    Type.Number(),
    public_url:         Type.String(),
    native_sample_rate: Type.Integer(),
    maintenance:        Type.Array(Maintenance),
})
export type AppDomain = Static<typeof AppDomain>

export const CreateSession = Type.Object({
    domain:             DomainId,
    time:               JsonTimeRange,
    tracks:             Type.Optional(Type.Record(TrackId, SessionTrack)),
    mixers:             Type.Optional(Type.Record(MixerId, SessionMixer)),
    dynamic:            Type.Optional(Type.Record(DynamicId, SessionDynamicInstance)),
    fixed:              Type.Optional(Type.Record(FixedId, SessionFixedInstance)),
    security:           Type.Optional(Type.Record(SecureKey, SessionSecurity)),
    dry_run:            Type.Boolean()
})
export type CreateSession = Static<typeof CreateSession>

export const SessionSpec = Type.Object ({
    tracks:             Type.Optional(Type.Record(TrackId, SessionTrack)),
    mixers:             Type.Optional(Type.Record(MixerId, SessionMixer)),
    dynamic:            Type.Optional(Type.Record(DynamicId, SessionDynamicInstance)),
    fixed:              Type.Optional(Type.Record(FixedId, SessionFixedInstance)),
})
export type SessionSpec = Static<typeof SessionSpec>
