// noinspection JSUnusedGlobalSymbols

import { Static, Type } from "@sinclair/typebox";
import {
    AppId,
    DomainId,
    DynamicId,
    FixedId, FixedInstanceId,
    InputId,
    MediaId,
    MediaObjectId,
    MixerId, ModelId,
    SecureKey,
    TrackId
} from "./new_types";
import {FromJsonTimeRanges, JsonTimeRange} from "./time";
import {InstanceParameters} from "./instance";

export const SessionTrackChannels = Type.Union([
    Type.Literal("mono"),
    Type.Literal("stereo")
])
export type SessionTrackChannels = Static<typeof SessionTrackChannels>

export const SessionTimeSegment = Type.Object({
    start: Type.Number(),
    length: Type.Number()
})
export type SessionTimeSegment = Static<typeof SessionTimeSegment>

export const SessionTrackMedia = Type.Object({
    channels: SessionTrackChannels,
    media_segment: SessionTimeSegment,
    timeline_segment: SessionTimeSegment,
    object_id: MediaObjectId,
})
export type SessionTrackMedia = Static<typeof SessionTrackMedia>

export const SessionTrack = Type.Object({
    channels: SessionTrackChannels,
    media: Type.Record(MediaId, SessionTrackMedia),
})
export type SessionTrack = Static<typeof SessionTrack>

export const SessionObjectId = Type.Union([
    Type.Object({mixer: MixerId}),
    Type.Object({fixed_instance: FixedId}),
    Type.Object({dynamic_instance: DynamicId}),
    Type.Object({track: TrackId}),
])
export type SessionObjectId = Static<typeof SessionObjectId>

export const MixerChannels = Type.Union([
    Type.Object({mono: Type.Integer({minimum: 0, maximum: 256})}),
    Type.Object({stereo: Type.Integer({minimum: 0, maximum: 256})}),
])
export type MixerChannels = Static<typeof MixerChannels>

export const MixerInput = Type.Object({
    source_id: SessionObjectId,
    input_channels: MixerChannels,
    mixer_channels: MixerChannels,
    volume: Type.Number({default: 0}),
    pan: Type.Number({minimum: -1, maximum: 1, default: 0}),
})
export type MixerInput = Static<typeof MixerInput>

export const SessionMixer = Type.Object({
    inputs: Type.Record(InputId, MixerInput)
})
export type SessionMixer = Static<typeof SessionMixer>

export const SessionDynamicInstance = Type.Object({
    model_id: ModelId,
    parameters: InstanceParameters,
    inputs: Type.Record(InputId, MixerInput)
})
export type SessionDynamicInstance = Static<typeof SessionDynamicInstance>

export const SessionFixedInstance = Type.Object({
    instance_id: FixedInstanceId,
    parameters: InstanceParameters,
    inputs: Type.Record(InputId, MixerInput)
})
export type SessionFixedInstance = Static<typeof SessionFixedInstance>

export const SessionSecurity = Type.Object({
    structure: Type.Boolean({default: false}),
    media: Type.Boolean({default: false}),
    parameters: Type.Boolean({default: false}),
    transport: Type.Boolean({default: false}),
    audio: Type.Boolean({default: false}),
})
export type SessionSecurity = Static<typeof SessionSecurity>

export const JsonSession = Type.Object({
    version: Type.Number(),
    domain_id: DomainId,
    app_id: AppId,
    time: JsonTimeRange,
    tracks: Type.Record(TrackId, SessionTrack),
    mixers: Type.Record(MixerId, SessionMixer),
    dynamic: Type.Record(DynamicId, SessionDynamicInstance),
    fixed: Type.Record(FixedId, SessionFixedInstance),
    security: Type.Record(SecureKey, SessionSecurity),
    deleted: Type.Boolean()
})
export type JsonSession = Static<typeof JsonSession>

export const JsonCreateSession = Type.Integer([
    Type.Omit(JsonSession, ['deleted', 'version']),
    Type.Object({
        dry_run: Type.Boolean()
    })
])
export type JsonCreateSession = Static<typeof JsonCreateSession>

export type Session = FromJsonTimeRanges<JsonSession, 'time'>

export const SessionMixerId = Type.Union([
    Type.Object({ "mixer": MixerId }),
    Type.Object({ "fixed_instance": FixedId }),
    Type.Object({ "dynamic_instance": DynamicId })
])
export type SessionMixerId = Static<typeof SessionMixerId>