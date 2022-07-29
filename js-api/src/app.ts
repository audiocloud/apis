import { Static, Type, TSchema } from "@sinclair/typebox";
import Option from "./utils/option";
import { DynamicId, FixedId, FixedInstanceId, MediaObjectId, MixerId, ReportId, TrackId } from "./new_types";
import { JsonTimeStamp, Timestamped } from "./time";
import { InstancePlayState, InstancePowerState } from "./instance";
import { DesiredSessionPlayState, SessionPlayState } from "./change";
import { CompressedAudio } from "./audio_engine";
import { MultiChannelValue } from "./model";

export function DiffStamped<T extends TSchema>(t: T) {
    return Type.Tuple([Type.Integer(), t])
}

export const FixedInstancePacket = Type.Object({
    errors:                 Type.Array(Type.String()),
    instance_metering:      Type.Record(ReportId, Type.Array(DiffStamped(MultiChannelValue))),
    input_metering:         Type.Array(DiffStamped(MultiChannelValue)),
    output_metering:        Type.Array(DiffStamped(MultiChannelValue)),
    media_pos:              Option(Type.Number()),
    power:                  Option(Timestamped(InstancePowerState)),
    media:                  Option(Timestamped(InstancePlayState)),
})
export type FixedInstancePacket = Static<typeof FixedInstancePacket>

export const DynamicInstancePacket = Type.Object({
    instance_metering:      Type.Record(ReportId, Type.Array(DiffStamped(MultiChannelValue))),
    input_metering:         Type.Array(DiffStamped(MultiChannelValue)),
    output_metering:        Type.Array(DiffStamped(MultiChannelValue)),
})
export type DynamicInstancePacket = Static<typeof DynamicInstancePacket>

export const TrackPacket = Type.Object({
    output_metering:        Type.Array(DiffStamped(MultiChannelValue))
})
export type TrackPacket = Static<typeof TrackPacket>

export const MixerPacket = Type.Object({
    output_metering:        Type.Array(DiffStamped(MultiChannelValue))
})
export type MixerPacket = Static<typeof MixerPacket>

export const SessionPacket = Type.Object({
    created_at:             JsonTimeStamp,
    fixed:                  Type.Record(FixedId, FixedInstancePacket),
    dynamic:                Type.Record(DynamicId, DynamicInstancePacket),
    mixers:                 Type.Record(MixerId, MixerPacket),
    tracks:                 Type.Record(TrackId, TrackPacket),
    waiting_for_instances:  Type.Array(FixedInstanceId),
    waiting_for_media:      Type.Array(MediaObjectId),
    compressed_audio:       Type.Array(CompressedAudio),
    desired_play_state:     DesiredSessionPlayState,
    play_state:             SessionPlayState,
    audio_engine_ready:     Type.Boolean(),
})
export type SessionPacket = Static<typeof SessionPacket>
