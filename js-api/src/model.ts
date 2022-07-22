// noinspection JSUnusedGlobalSymbols

import {Type, Static} from "@sinclair/typebox";
import {ParameterId, ReportId} from "./new_types";

export const ResourceId = Type.Union([
    Type.Literal("ram"),
    Type.Literal("cpu"),
    Type.Literal("gpu"),
    Type.Literal("antelope_dsp"),
    Type.Literal("universal_audio_dsp")
])
export type ResourceId = Static<typeof ResourceId>

export const ControlChannels = Type.Union([
    Type.Literal("global"),
    Type.Literal("left"),
    Type.Literal("right"),
    Type.Literal("generic")
])
export type ControlChannels = Static<typeof ControlChannels>

export const InputChannelRole = Type.Union([
    Type.Literal("audio"),
    Type.Literal("side_chain")
])
export type InputChannelRole = Static<typeof InputChannelRole>

export const OutputChannelRole = Type.Union([
    Type.Literal("audio"),
])
export type OutputChannelRole = Static<typeof OutputChannelRole>

export const ModelInputs = Type.Array(Type.Tuple([ControlChannels, InputChannelRole]))
export type ModelInputs = Static<typeof ModelInputs>

export const ModelOutputs = Type.Array(Type.Tuple([ControlChannels, OutputChannelRole]))
export type ModelOutputs = Static<typeof ModelOutputs>

export const ModelElementScope = Type.Union([
    Type.Literal("global"),
    Type.Literal("all_inputs"),
    Type.Literal("all_outputs"),
    Type.Object({
        size: Type.Number()
    })
])
export type ModelElementScope = Static<typeof ModelElementScope>

export const InstanceValueUnit = Type.Union([
    Type.Literal("no"),
    Type.Literal("percent"),
    Type.Literal("dB"),
    Type.Literal("hz"),
    Type.Literal("oct"),
    Type.Literal("toggle")
])
export type InstanceValueUnit = Static<typeof InstanceValueUnit>

export const GlobalParameterRole = Type.Union([
    Type.Literal("enable"),
    Type.Literal("bypass")
])
export type GlobalParameterRole = Static<typeof GlobalParameterRole>

export const AmplifierId = Type.Union([
    Type.Literal("input"),
    Type.Literal("output"),
    Type.Literal("global"),
    Type.Literal("insert_input"),
    Type.Literal("insert_output"),
])
export type AmplifierId = Static<typeof AmplifierId>

export const AmpifierParameterRole = Type.Union([
    Type.Literal("enable"),
    Type.Literal("gain"),
    Type.Literal("distortion"),
    Type.Literal("slew_rate"),
])
export type AmpifierParameterRole = Static<typeof AmpifierParameterRole>

export const DynamicsId = Type.Union([
    Type.Literal("total"),
    Type.Literal("compressor"),
    Type.Literal("gate"),
    Type.Literal("limiter"),
    Type.Literal("de_esser"),
])
export type DynamicsId = Static<typeof DynamicsId>

export const DynamicsParameterRole = Type.Union([
    Type.Literal("ratio"),
    Type.Literal("threshold"),
    Type.Literal("ceiling"),
    Type.Literal("attack"),
    Type.Literal("release"),
    Type.Literal("auto_release"),
    Type.Literal("auto_attack"),
    Type.Literal("auto_ratio"),
    Type.Literal("knee"),
    Type.Literal("detector_input"),
    Type.Literal("detector_material"),
    Type.Literal("detector_filter"),
    Type.Literal("mid_emphasis"),
])
export type DynamicsParameterRole = Static<typeof DynamicsParameterRole>

export const FilterId = Type.Union([
    Type.Literal("high_pass"),
    Type.Literal("low"),
    Type.Literal("low_mid"),
    Type.Literal("mid"),
    Type.Literal("hig_hmid"),
    Type.Literal("high"),
    Type.Literal("low_pass"),
    Type.Literal("band_pass"),
    Type.Literal("dynamics"),
    Type.Literal("de_esser"),
])
export type FilterId = Static<typeof FilterId>

export const FilterParameterRole = Type.Union([
    Type.Literal("gain"),
    Type.Literal("gain_direction"),
    Type.Literal("frequency"),
    Type.Literal("bandwidth"),
    Type.Literal("type")
])
export type FilterParameterRole = Static<typeof FilterParameterRole>

export const InstanceParameterRole = Type.Union([
    Type.Literal("no_role"),
    Type.Object({
        global: GlobalParameterRole
    }),
    Type.Object({
        amplifier: Type.Tuple([AmplifierId, AmpifierParameterRole]),
    }),
    Type.Object({
        dynamics: Type.Tuple([DynamicsId, DynamicsParameterRole])
    }),
    Type.Object({
        filter: Type.Tuple([FilterId, FilterParameterRole])
    })
])
export type InstanceParameterRole = Static<typeof InstanceParameterRole>

export const InstanceValue = Type.Union([
    Type.Number(),
    Type.String(),
    Type.Boolean()
])
export type InstanceValue = Static<typeof InstanceValue>

export const InstanceValueOption = Type.Union([
    InstanceValue,
    Type.Tuple([InstanceValue, InstanceValue])
])
export type InstanceValueOption = Static<typeof InstanceValueOption>

export const ModelParameter = Type.Object({
    scope: ModelElementScope,
    unit: InstanceValueUnit,
    role: InstanceParameterRole,
    values: Type.Array(InstanceValueOption, {default: []})
})
export type ModelParameter = Static<typeof ModelParameter>

export const AmplifierReportRole = Type.Union([
    Type.Literal("peak_volume"),
    Type.Literal("rms_volume"),
    Type.Literal("lufs_volume_momentary"),
    Type.Literal("lufs_volume_short_term"),
    Type.Literal("lufs_volume_integrated"),
]);
export type AmplifierReportRole = Static<typeof AmplifierReportRole>

export const DynamicsReportRole = Type.Union([
    Type.Literal("gain_reduction"),
    Type.Literal("gain_reduction_limit_hit"),
])
export type DynamicsReportRole = Static<typeof DynamicsReportRole>

export const InstanceReportRole = Type.Union([
    Type.Literal("no_role"),
    Type.Object({
        amplifier: Type.Tuple([AmplifierId, AmplifierReportRole])
    }),
    Type.Object({
        dynamics: Type.Tuple([DynamicsId, DynamicsReportRole])
    })
])
export type InstanceReportRole = Static<typeof InstanceReportRole>

export const ModelReport = Type.Object({
    scope: ModelElementScope,
    unit: InstanceValueUnit,
    role: InstanceReportRole,
    values: Type.Array(InstanceValueOption, {default: []})
})
export type ModelReport = Static<typeof ModelReport>

export const ModelParameters = Type.Record(ParameterId, ModelParameter)
export type ModelParameters = Static<typeof ModelParameters>

export const ModelReports = Type.Record(ReportId, ModelReport)
export type ModelReports = Static<typeof ModelReports>

export const ModelCapability = Type.Union([
    Type.Literal("power_distributor"),
    Type.Literal("audio_router"),
    Type.Literal("audio_mixer"),
    Type.Literal("digital_input_output"),
])
export type ModelCapability = Static<typeof ModelCapability>

export const Model = Type.Object({
    resources:      Type.Optional(Type.Record(ResourceId, Type.Number())),
    inputs:         ModelInputs,
    outputs:        ModelOutputs,
    parameters:     ModelParameters,
    reports:        ModelReports,
    media:          Type.Boolean(),
    capabilities:   Type.Optional(Type.Array(ModelCapability))
}, {additionalProperties: false})
export type Model = Static<typeof Model>
