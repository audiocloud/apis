import { Static, Type } from "@sinclair/typebox";
import { SessionPacket } from "./app";
import { DesiredSessionPlayState, ModifySessionSpec, SessionState } from "./change";
import { CreateSession, SessionSpec } from "./cloud/apps";
import { DownloadMedia } from "./media";
import { AppMediaObjectId, AppSessionId, SecureKey } from "./new_types";
import { SessionSecurity } from "./session";

export const DomainSessionCommand = Type.Union([
    Type.Object({
        "create":                   Type.Object({
            "app_session_id":       AppSessionId,
            "create":               CreateSession,
        })
    }),
    Type.Object({
        "set_spec":                 Type.Object({
            "app_session_id":       AppSessionId,
            "version":              Type.Integer(),
            "spec":                 SessionSpec,
        })
    }),
    Type.Object({
        "set_security":             Type.Object({
            "app_session_id":       AppSessionId,
            "version":              Type.Integer(),
            "security":             Type.Record(SecureKey, SessionSecurity),
        })
    }),
    Type.Object({
        "modify":                   Type.Object({
            "app_session_id":       AppSessionId,
            "version":              Type.Integer(),
            "modifications":        Type.Array(ModifySessionSpec),
        })
    }),
    Type.Object({
        "set_desired_play_state":   Type.Object({
            "app_session_id":       AppSessionId,
            "version":              Type.Integer(),
            "desired_play_state":   DesiredSessionPlayState,
        })
    }),
    Type.Object({
        "delete":                   Type.Object({
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
    Type.Object({ "packet":         Type.Tuple([AppSessionId, SessionPacket]) }),
    Type.Object({ "spec":           Type.Tuple([AppSessionId, SessionSpec]) }),
    Type.Object({ "state":          Type.Tuple([AppSessionId, SessionState]) }),
    Type.Object({ "login_success":  AppSessionId }),
    Type.Object({ "login_error":    Type.Tuple([AppSessionId, Type.String()]) }),
    Type.Object({ "session_error":  Type.Tuple([AppSessionId, Type.String()]) }),
])
export type WebSocketEvent = Static<typeof WebSocketEvent>

export const WebSocketCommand = Type.Union([
    Type.Object({ "login":          Type.Tuple([AppSessionId, SecureKey]) }),
    Type.Object({ "logout":         AppSessionId }),
    Type.Object({ "session":        DomainSessionCommand }),
])
export type WebSocketCommand = Static<typeof WebSocketCommand>
