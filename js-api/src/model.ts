// noinspection JSUnusedGlobalSymbols

import { Type, Static } from "@sinclair/typebox";
import { ParameterId, ReportId } from "./new_types";

export const ResourceId = Type.Union([
    Type.Literal("ram"),
    Type.Literal("cpu"),
    Type.Literal("gpu"),
    Type.Literal("antelope_dsp"),
    Type.Literal("universal_audio_dsp")
])
export type ResourceId = Static<typeof ResourceId>

export const ModelValueUnit = Type.Union([
    Type.Literal("no"),
    Type.Literal("percent"),
    Type.Literal("dB"),
    Type.Literal("hz"),
    Type.Literal("oct"),
    Type.Literal("toggle"),
    Type.Literal("amps"),
    Type.Literal("watthrs"),
])
export type ModelValueUnit = Static<typeof ModelValueUnit>

export const ModelValue = Type.Union([
    Type.String(),
    Type.Number(),
    Type.Boolean(),
])
export type ModelValue = Static<typeof ModelValue>

export const ModelValueOption = Type.Union([
    Type.Object({ "single": ModelValue }),
    Type.Object({ "range" : Type.Tuple([ModelValue, ModelValue]) }),
])
export type ModelValueOption = Static<typeof ModelValueOption>

export const ControlChannels = Type.Union([
    Type.Literal("global"),
    Type.Literal("left"),
    Type.Literal("right"),
    Type.Literal("generic")
])
export type ControlChannels = Static<typeof ControlChannels>

export const ModelInput = Type.Union([
    Type.Object({ "audio": ControlChannels }),
    Type.Literal("sidechain"),
    Type.Literal("midi"),
])
export type ModelInput = Static<typeof ModelInput>

export const ModelOutput = Type.Union([
    Type.Object({ "audio": ControlChannels }),
    Type.Literal("midi"),
])
export type ModelOutput = Static<typeof ModelOutput>

export const ModelInputs = Type.Array(ControlChannels)
export type ModelInputs = Static<typeof ModelInputs>

export const ModelOutputs = Type.Array(ControlChannels)
export type ModelOutputs = Static<typeof ModelOutputs>

export const ModelElementScope = Type.Union([
    Type.Literal("global"),
    Type.Literal("all_inputs"),
    Type.Literal("all_outputs"),
    Type.Object({ "size": Type.Integer() })
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

export const AmplifierParameterRole = Type.Union([
    Type.Literal("enable"),
    Type.Literal("gain"),
    Type.Literal("distortion"),
    Type.Literal("slew_rate"),
])
export type AmplifierParameterRole = Static<typeof AmplifierParameterRole>

export const ChannelParameterRole = Type.Union([
    Type.Literal("pan")
])
export type ChannelParameterRole = Static<typeof ChannelParameterRole>

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

export const FilterParameterRole = Type.Union([
    Type.Literal("gain"),
    Type.Literal("gain_direction"),
    Type.Literal("frequency"),
    Type.Literal("bandwidth"),
    Type.Literal("type")
])
export type FilterParameterRole = Static<typeof FilterParameterRole>

export const PowerReportRole = Type.Union([
    Type.Literal("powered"),
    Type.Literal("current"),
    Type.Literal("power_factor"),
    Type.Literal("total_energy"),
])
export type PowerReportRole = Static<typeof PowerReportRole>

export const RoleFilterId = Type.Union([
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

export const InstanceParameterRole = Type.Union([
    Type.Literal("no_role"),
    Type.Object({
        global: GlobalParameterRole
    }),
    Type.Object({
        amplifier: Type.Tuple([AmplifierId, AmplifierParameterRole]),
    }),
    Type.Object({
        dynamics: Type.Tuple([DynamicsId, DynamicsParameterRole])
    }),
    Type.Object({
        filter: Type.Tuple([RoleFilterId, FilterParameterRole])
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

export const ModelReportRole = Type.Union([
    Type.Literal("no_role"),
    Type.Object({ "power": PowerReportRole }),
    Type.Object({ "amplifier": Type.Tuple([AmplifierId, AmplifierReportRole]) }),
    Type.Object({ "dynamics": Type.Tuple([DynamicsId, DynamicsReportRole]) }),
])
export type ModelReportRole = Static<typeof ModelReportRole>

export const ModelParameterRole = Type.Union([
    Type.Literal("no_role"),
    Type.Literal("power"),
    Type.Object({ "Global": GlobalParameterRole }),
    Type.Object({ "Channel": ChannelParameterRole }),
    Type.Object({ "Amplifier": Type.Tuple([AmplifierId, AmplifierParameterRole]) }),
    Type.Object({ "Dynamics": Type.Tuple([DynamicsId, DynamicsParameterRole]) }),
    Type.Object({ "Filter": Type.Tuple([RoleFilterId, FilterParameterRole]) }),
])
export type ModelParameterRole = Static<typeof ModelParameterRole>

export const ModelParameter = Type.Object({
    scope:      ModelElementScope,
    unit:       Type.Optional(ModelValueUnit),
    role:       ModelParameterRole,
    values:     Type.Array(ModelValueOption)
})
export type ModelParameter = Static<typeof ModelParameter>

export const ModelReport = Type.Object({
    scope:      ModelElementScope,
    unit:       Type.Optional(ModelValueUnit),
    role:       ModelReportRole,
    values:     Type.Array(ModelValueOption),
    public:     Type.Optional(Type.Boolean())
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
