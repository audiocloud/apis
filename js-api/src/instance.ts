import { Static, Type } from "@sinclair/typebox";
import { ParameterId } from "./new_types";
import { InstanceValue } from "./model";
import { PlayId, RenderId } from "./change";
import { Timestamped } from "./time";

export const InstancePlayState = Type.Union([
    Type.Object({ "preparing_to_play":      { play_id: PlayId }}),
    Type.Object({ "playing":                { play_id: PlayId }}),
    Type.Object({ "preparing_to_render":    { length: Type.Number(), render_id: RenderId }}),
    Type.Object({ "rendering":              { length: Type.Number(), render_id: RenderId }}),
    Type.Object({ "rewinding":              { to: Type.Number() }}),
    Type.Literal("stopping"),
    Type.Literal("stopped"),
])
export type InstancePlayState = Static<typeof InstancePlayState>

export const DesiredInstancePlayState = Type.Union([
    Type.Object({ "playing":                { play_id: PlayId }}),
    Type.Object({ "rendering":              { length: Type.Number(), render_id: RenderId }}),
    Type.Literal("stopped"),
])
export type DesiredInstancePlayState = Static<typeof DesiredInstancePlayState>

export const InstancePowerState = Type.Union([
    Type.Literal("warming_up"),
    Type.Literal("cooling_down"),
    Type.Literal("powered_up"),
    Type.Literal("shut_down"),
])
export type InstancePowerState = Static<typeof InstancePowerState>

export const DesiredInstancePowerState = Type.Union([
    Type.Literal("powered_up"),
    Type.Literal("shut_down"),
])
export type DesiredInstancePowerState = Static<typeof DesiredInstancePowerState>

export const InstancePowerStateReport = Type.Object({
    desired: Timestamped(DesiredInstancePowerState),
    actual:  Timestamped(InstancePowerState),
})
export type InstancePowerStateReport = Static<typeof InstancePowerStateReport>

export const InstancePlayStateReport = Type.Object({
    desired: Timestamped(DesiredInstancePlayState),
    actual:  Timestamped(InstancePlayState),
})
export type InstancePlayStateReport = Static<typeof InstancePlayStateReport>

export const MultiChannelValue = Type.Array(Type.Tuple([
    Type.Integer({minimum: 0, maximum: 64}),
    InstanceValue
]))
export type MultiChannelValue = Static<typeof MultiChannelValue>

export const InstanceParameters = Type.Record(ParameterId, MultiChannelValue)
export type InstanceParameters = Static<typeof InstanceParameters>