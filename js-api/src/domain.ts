import { Static, Type } from "@sinclair/typebox";
import { SessionPacket } from "./app";
import { DesiredSessionPlayState, ModifySessionSpec } from "./change";
import { CreateSession, SessionSpec } from "./cloud/apps";
import { DownloadMedia, MediaDownloadState } from "./media";
import { AppMediaObjectId, AppSessionId, MediaObjectId, SecureKey, SessionId } from "./new_types";
import { SessionSecurity } from "./session";
import Option from "./utils/option";

export const DomainSessionCommand = Type.Union([
    Type.Object({
        "Create":                   Type.Object({
            "app_session_id":       AppSessionId,
            "create":               CreateSession,
        })
    }),
    Type.Object({
        "SetSpec":                  Type.Object({
            "app_session_id":       AppSessionId,
            "version":              Type.Integer(),
            "spec":                 SessionSpec,
        })
    }),
    Type.Object({
        "SetSecurity":              Type.Object({
            "app_session_id":       AppSessionId,
            "version":              Type.Integer(),
            "security":             Type.Record(SecureKey, SessionSecurity),
        })
    }),
    Type.Object({
        "Modify":                   Type.Object({
            "app_session_id":       AppSessionId,
            "version":              Type.Integer(),
            "modifications":        Type.Array(ModifySessionSpec),
        })
    }),
    Type.Object({
        "SetDesiredPlayState":      Type.Object({
            "app_session_id":       AppSessionId,
            "version":              Type.Integer(),
            "desired_play_state":   DesiredSessionPlayState,
        })
    }),
    Type.Object({
        "Delete":                   Type.Object({
            "app_session_id":       AppSessionId,
        })
    }),
])
export type DomainSessionCommand = Static<typeof DomainSessionCommand>

export const DomainMediaCommand = Type.Union([
    Type.Object({
        "download":                 Type.Object({
            "app_media_id":         AppMediaObjectId,
            "download":             DownloadMedia,
        })
    }),
    Type.Object({
        "delete":                   Type.Object({
            "app_media_id":         AppMediaObjectId,
        })
    }),
])
export type DomainMediaCommand = Static<typeof DomainMediaCommand>

export const WebSocketEvent = Type.Union([
    Type.Object({ "packet":         Type.Tuple([SessionId, SessionPacket]) }),
    Type.Object({ "download":       Type.Tuple([MediaObjectId, MediaDownloadState]) }),
])
export type WebSocketEvent = Static<typeof WebSocketEvent>