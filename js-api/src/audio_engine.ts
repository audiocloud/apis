import { Static, Type } from "@sinclair/typebox";
import Option from "./utils/option";
import { Uuid } from "./utils/uuid";
import { PlayId, PlaySegment, PlaySession, RenderId, RenderSession } from "./change";
import { MultiChannelValue } from "./instance";

export const CompressedAudio = Type.Object({
    play_id:                            PlayId,
    timeline_pos:                       Type.Number(),
    stream_pos:                         Type.Integer(),
    last:                               Type.Boolean(),
})
export type CompressedAudio = Static<typeof CompressedAudio> & { buffer: Uint8Array }

export const AudioEngineCommand = Type.Union([
    Type.Object({
        "set_track_state_chunk":        Type.Object({
            "track_id":                 Uuid,
            "chunk":                    Type.String()
        })
    }),
    Type.Object({
        "set_item_state_chunk":         Type.Object({
            "track_id":                 Uuid,
            "item_id":                  Uuid,
            "chunk":                    Type.String() 
        })
    }),
    Type.Object({
        "set_track_values":             Type.Object({
            "track_id":                 Uuid,
            "volume":                   Option(Type.Number()),
            "pan":                      Option(Type.Number()),
            "master_send":              Option(Type.Boolean())
        })
    }),
    Type.Object({
        "set_receive_values":           Type.Object({
            "track_id":                 Uuid,
            "receive_track_id":         Uuid,
            "volume":                   Option(Type.Number()),
            "pan":                      Option(Type.Number())
        })
    }),
    Type.Object({
        "set_fx_values":                Type.Object({
            "track_id":                 Uuid,
            "fx_id":                    Uuid,
            "values":                   Type.Record(Type.Number(), Type.Number())
        })
    }),
    Type.Object({
        "set_fx_state_values":          Type.Object({
            "track_id":                 Uuid,
            "fx_id":                    Uuid,
            "enabled":                  Option(Type.Boolean()),
            "dry_wet":                  Option(Type.Number())
        })
    }),
    Type.Object({
        "set_master":                   Type.Object({
            "track_id":                 Uuid 
        })
    }),
    Type.Object({
        "delete_track":                 Type.Object({
            "track_id":                 Uuid 
        })
    }),
    Type.Object({ "play":               PlaySession }),
    Type.Object({ "set_play_segment":   PlaySegment }),
    Type.Object({ "render":             RenderSession }),
    Type.Literal("stop"),
    Type.Literal("exit")
])
export type AudioEngineCommand = Static<typeof AudioEngineCommand>

export const AudioEngineEvent = Type.Union([
    Type.Literal("loaded"),
    Type.Literal("stopped"),
    Type.Object({
        "playing":                      Type.Object({
            "playing":                  PlaySession,
            "audio":                    CompressedAudio
        })
    }),
    Type.Object({
        "rendering":                    Type.Object({
            "rendering":                RenderSession
        })
    }),
    Type.Object({
        "rendering_finished":           Type.Object({
            "render_id":                RenderId,
            "path":                     Type.String()
        })
    }),
    Type.Object({
        "meters":                       Type.Object({
            "peak_meters":              Type.Array(Uuid, MultiChannelValue)
        })
    }),
    Type.Object({
        "exit":                         Type.Object({
            "code":                     Type.Number()
        })
    }),
])
export type AudioEngineEvent = Static<typeof AudioEngineEvent>

export const AudioEngineError = Type.Union([
    Type.Object({"track_not_found":     Type.Integer() }),
    Type.Object({"item_not_found":      Type.Tuple(Type.Integer(), Type.Integer()) }),
    Type.Object({"internal_error":      Type.String() }),
    Type.Object({"rpc":                 Type.String() })
])
export type AudioEngineError = Static<typeof CompressedAudio>