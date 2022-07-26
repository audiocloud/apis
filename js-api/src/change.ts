import { Static, Type } from "@sinclair/typebox";
import Option from "./utils/option";
import { TrackId, FixedId, DynamicId, MixerId, InputId, MediaId, MediaObjectId, ParameterId, SecureKey } from "./new_types";
import { MixerInput, MixerInputValues, SessionDynamicInstance, SessionFixedInstance, SessionMixer, SessionMixerId, SessionMode, SessionObjectId, SessionSecurity, SessionTimeSegment, SessionTrackChannels } from "./session";
import { MultiChannelValue } from "./instance";
import { Timestamped } from "./time";

export const ModifySessionSpec = Type.Union([
    Type.Object({
        "add_track":                    Type.Object({
            "track_id":                 TrackId,
            "channels":                 SessionTrackChannels,
        })
    }),
    Type.Object({
        "add_track_media":              Type.Object({
            "track_id":                 TrackId,
            "media_id":                 MediaId,
            "channels":                 SessionTrackChannels,
            "media_segment":            SessionTimeSegment,
            "timeline_segment":         SessionTimeSegment,
            "object_id":                MediaObjectId,
        })
    }),
    Type.Object({
        "set_track_media_values":       Type.Object({
            "track_id":                 TrackId,
            "media_id":                 MediaId,
            "channels":                 Option(SessionTrackChannels),
            "media_segment":            Option(SessionTimeSegment),
            "timeline_segment":         Option(SessionTimeSegment),
            "object_id":                Option(MediaObjectId),
        })
    }),
    Type.Object({
        "delete_track_media":           Type.Object({
            "track_id":                 TrackId,
            "media_id":                 MediaId,
        })
    }),
    Type.Object({
        "delete_track":                 Type.Object({
            "track_id":                 TrackId,
        })
    }),
    Type.Object({
        "add_fixed_instance":           Type.Object({
            "fixed_id":                 FixedId,
            "process":                  SessionFixedInstance,
        })
    }),
    Type.Object({
        "add_dynamic_instance":         Type.Object({
            "dynamic_id":               DynamicId,
            "process":                  SessionDynamicInstance,
        })
    }),
    Type.Object({
        "add_mixer":                    Type.Object({
            "mixer_id":                 MixerId,
            "mixer":                    SessionMixer,
        })
    }),
    Type.Object({
        "delete_mixer":                 Type.Object({
            "mixer_id":                 SessionMixerId,
        })
    }),
    Type.Object({
        "delete_mixer_input":           Type.Object({
            "mixer_id":                 SessionMixerId,
            "input_id":                 InputId,
        })
    }),
    Type.Object({
        "delete_inputs_referencing":    Type.Object({
            "source_id":                SessionObjectId,
        })
    }),
    Type.Object({
        "add_mixer_input":              Type.Object({
            "mixer_id":                 SessionMixerId,
            "input_id":                 InputId,
            "input":                    MixerInput,
        })
    }),
    Type.Object({
        "set_input_values":             Type.Object({
            "mixer_id":                 SessionMixerId,
            "input_id":                 InputId,
            "values":                   MixerInputValues,
        })
    }),
    Type.Object({
        "set_fixed_instance_parameter_values":      Type.Object({
            "fixed_id":                             FixedId,
            "values":                               Type.Record(ParameterId, MultiChannelValue),
        })
    }),
    Type.Object({
        "set_dynamic_instance_parameter_values":    Type.Object({
            "dynamic_id":                           DynamicId,
            "values":                               Type.Record(ParameterId, MultiChannelValue),
        })
    }),
])
export type ModifySessionSpec = Static<typeof ModifySessionSpec>

export const ModifySession = Type.Union([
    Type.Object({
        "spec":                                     ModifySessionSpec }),
    Type.Object({
        "set_security":                             Type.Object({
            "key":                                  SecureKey,
            "security":                             SessionSecurity
        }),
    }),
    Type.Object({
        "revoke_security":                          Type.Object({
            "key":                                  SecureKey
        }),
    })
])
export type ModifySession = Static<typeof ModifySession>

export const SampleRate = Type.Union([
    Type.Literal("192"),
    Type.Literal("96"),
    Type.Literal("88.2"),
    Type.Literal("48"),
    Type.Literal("44.1"),
])
export type SampleRate = Static<typeof SampleRate>

export const PlayBitDepth = Type.Union([
    Type.Literal("24"),
    Type.Literal("16"),
])
export type PlayBitDepth = Static<typeof PlayBitDepth>

export const PlayId = Type.Integer()
export type PlayId = Static<typeof PlayId>

export const PlaySession = Type.Object({
    play_id:                    PlayId,
    segment:                    SessionTimeSegment,
    start_at:                   Type.Number(),
    looping:                    Type.Boolean(),
    sample_rate:                SampleRate,
    bit_depth:                  PlayBitDepth,
})
export type PlaySession = Static<typeof PlaySession>

export const PlaySegment = Type.Object({
    segment:                    SessionTimeSegment,
    looping:                    Type.Boolean(),
    start_at:                   Type.Number(),
})
export type PlaySegment = Static<typeof PlaySegment>

export const RenderId = Type.Integer()
export type RenderId = Static<typeof RenderId>

export const RenderSession = Type.Object({
    render_id:                  RenderId,
    segment:                    SessionTimeSegment,
    object_id:                  MediaObjectId,
    put_url:                    Type.String(),
    notify_url:                 Type.String(),
    context:                    Type.String(),
})
export type RenderSession = Static<typeof RenderSession>

export const SuccessfulRenderNotification = Type.Object({
    render_id:                  RenderId,
    object_id:                  MediaObjectId,
    context:                    Type.String(),
})
export type SuccessfulRenderNotification = Static<typeof SuccessfulRenderNotification>

export const RenderNotification = Type.Union([
    Type.Object({ "Ok":         SuccessfulRenderNotification }),
    Type.Object({ "Err":        Type.String() })
])
export type RenderNotification = Static<typeof RenderNotification>

export const SessionPlayState = Type.Union([
    Type.Object({ "preparing_to_play":      PlaySession }),
    Type.Object({ "preparing_to_render":    RenderSession }),
    Type.Object({ "playing":                PlaySession }),
    Type.Object({ "rendering":              RenderSession }),
    Type.Literal("preparing_to_stop"),
    Type.Literal("stopped"),
])
export type SessionPlayState = Static<typeof SessionPlayState>

export const DesiredSessionPlayState = Type.Union([
    Type.Object({ "play":                   PlaySession }),     // Play, with sample rate conversion
    Type.Object({ "render":                 RenderSession }),   // Rendering is always a F32 WAV at full sample rate, so nothing else needs to happen here
    Type.Literal("stopped"),
])
export type DesiredSessionPlayState = Static<typeof DesiredSessionPlayState>

export const SessionState = Type.Object({
    play_state:                             Timestamped(SessionPlayState),
    desired_play_state:                     Timestamped(DesiredSessionPlayState),
    mode:                                   Timestamped(SessionMode)
})
export type SessionState = Static<typeof SessionState>

export const ModifySessionError = Type.Union([
    Type.Object({ "track_exists":                       TrackId }),
    Type.Object({ "fixed_instance_exists":              FixedId }),
    Type.Object({ "dynamic_instance_exists":            DynamicId }),
    Type.Object({ "mixer_exists":                       MixerId }),

    Type.Object({ "track_does_not_exist":               TrackId }),
    Type.Object({ "fixed_instance_does_not_exist":      FixedId }),
    Type.Object({ "dynamic_instance_does_not_exist":    DynamicId }),
    Type.Object({ "mixer_does_not_exist":               MixerId }),

    Type.Object({ "input_exists":                       Type.Tuple([SessionMixerId, InputId]) }),
    Type.Object({ "input_does_not_exist":               Type.Tuple([SessionMixerId, InputId]) }),

    Type.Object({ "media_exists":                       Type.Tuple([TrackId, MediaId]) }),
    Type.Object({ "media_does_not_exist":               Type.Tuple([TrackId, MediaId]) }),

    Type.Literal("cycle_detected")
])
export type ModifySessionError = Static<typeof ModifySessionError>
