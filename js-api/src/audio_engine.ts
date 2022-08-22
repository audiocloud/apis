import { Static, Type } from "@sinclair/typebox";
import Option from "./utils/option";
import { Uuid } from "./utils/uuid";
import { ModifySessionSpec, PlayId, PlaySession, RenderId, RenderSession } from "./change";
import { MultiChannelTimestampedValue, MultiChannelValue } from "./model";
import { AppMediaObjectId, AppSessionId, DynamicId, FixedInstanceId, ParameterId, ReportId } from "./new_types";
import { SessionSpec } from "./cloud/apps";
import { InstanceRouting } from "./cloud/domains";
import { SessionFlowId } from "./session";

export const CompressedAudio = Type.Object({
    play_id:                            PlayId,
    timeline_pos:                       Type.Number(),
    stream_pos:                         Type.Integer(),
    last:                               Type.Boolean(),
})
export type CompressedAudio = Static<typeof CompressedAudio> & { buffer: Uint8Array }

export const AudioEngineCommand = Type.Union([
    Type.Object({
        "set_spec":                     Type.Object({
            "session_id":               AppSessionId,
            "spec":                     SessionSpec,
            "instances":                Type.Record(FixedInstanceId, InstanceRouting),
            "media_ready":              Type.Record(AppMediaObjectId, Type.String())
        })
    }),
    Type.Object({
        "media":                        Type.Object({
            "session_id":               AppSessionId,
            "media_ready":              Type.Record(AppMediaObjectId, Type.String()),
        })
    }),
    Type.Object({
        "instances":                    Type.Object({
            "session_id":               AppSessionId,
            "instances":                Type.Record(FixedInstanceId, InstanceRouting),
        })
    }),
    Type.Object({
        "modify_spec":                  Type.Object({
            "session_id":               AppSessionId,
            "transaction":              Type.Array(ModifySessionSpec),
            "instances":                Type.Record(FixedInstanceId, InstanceRouting),
            "media_ready":              Type.Record(AppMediaObjectId, Type.String())
        })
    }),
    Type.Object({
        "set_dynamic_parameters":       Type.Object({
            "session_id":               AppSessionId,
            "dynamic_id":               DynamicId,
            "parameters":               Type.Record(ParameterId, MultiChannelValue)
        })
    }),
    Type.Object({
        "render":                       Type.Object({
            "session_id":               AppSessionId,
            "render":                   RenderSession
        })
    }),
    Type.Object({
        "play":                         Type.Object({
            "session_id":               AppSessionId,
            "play":                     PlaySession
        })
    }),
    Type.Object({
        "update_play":                  Type.Object({
            "session_id":               AppSessionId,
            "play":                     PlaySession
        })
    }),
    Type.Object({
        "stop":                         Type.Object({
            "session_id":               AppSessionId
        })
    }),
    Type.Object({
        "close":                        Type.Object({
            "session_id":               AppSessionId
        })
    })
])
export type AudioEngineCommand = Static<typeof AudioEngineCommand>

export const AudioEngineEvent = Type.Union([
    Type.Literal("loaded"),
    Type.Object({
        "stopped":                      Type.Object({
            "session_id":               AppSessionId
        })
    }),
    Type.Object({
        "playing":                      Type.Object({
            "session_id":               AppSessionId,
            "play_id":                  PlayId,
            "audio":                    CompressedAudio,
            "peak_meters":              Type.Array(Type.Tuple([SessionFlowId, MultiChannelValue])),
            "dynamic_reports":          Type.Record(DynamicId, Type.Record(ReportId, MultiChannelTimestampedValue))
        })
    }),
    Type.Object({
        "playing_failed":               Type.Object({
            "session_id":               AppSessionId,
            "play_id":                  PlayId,
            "error":                    Type.String()
        })
    }),
    Type.Object({
        "rendering":                    Type.Object({
            "session_id":               AppSessionId,
            "rendering":                RenderId,
            "completion":               Type.Number()
        })
    }),
    Type.Object({
        "rendering_finished":           Type.Object({
            "session_id":               AppSessionId,
            "render_id":                RenderId,
            "path":                     Type.String()
        })
    }),
    Type.Object({
        "rendering_failed":             Type.Object({
            "session_id":               AppSessionId,
            "render_id":                RenderId,
            "reason":                   Type.String()
        })
    }),
    Type.Object({
        "error":                        Type.Object({
            "session_id":               AppSessionId,
            "error":                    Type.String()
        })
    }),
])
export type AudioEngineEvent = Static<typeof AudioEngineEvent>

export const AudioEngineError = Type.Union([
    Type.Object({"track_not_found":     Type.Integer() }),
    Type.Object({"item_not_found":      Type.Tuple([Type.Integer(), Type.Integer()]) }),
    Type.Object({"internal_error":      Type.String() }),
    Type.Object({"rpc":                 Type.String() })
])
export type AudioEngineError = Static<typeof CompressedAudio>
