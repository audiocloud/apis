import { Static, Type } from "@sinclair/typebox";
import { AppMediaObjectId, AppSessionId } from "./new_types";
import { SessionTrackChannels, SessionTrackMediaFormat } from "./session";
import { JsonTimeStamp } from "./time";
import Option from "./utils/option";

export const MediaJobState = Type.Object({
    progress:       Type.Number(),
    retry:          Type.Integer(),
    error:          Option(Type.String()),
    in_progress:    Type.Boolean(),
    updated_at:     JsonTimeStamp
})
export type MediaJobState = Static<typeof MediaJobState>

export const DownloadMedia = Type.Object({
    get_url:        Type.String(),
    notify_url:     Type.String(),
    context:        Type.String()
})
export type DownloadMedia = Static<typeof DownloadMedia>

export const MediaMetadata = Type.Object({
    channels:       SessionTrackChannels,
    format:         SessionTrackMediaFormat,
    seconds:        Type.Number(),
    sample_rate:    Type.Integer(),
    bytes:          Type.Integer()
})
export type MediaMetadata = Static<typeof MediaMetadata>

export const ImportToDomain = Type.Object({
    path:           Type.String(),
    channels:       SessionTrackChannels,
    format:         SessionTrackMediaFormat,
    seconds:        Type.Number(),
    sample_rate:    Type.Integer(),
    bytes:          Type.Integer()
})
export type ImportToDomain = Static<typeof ImportToDomain>

export const UploadToDomain = Type.Object({
    channels:       SessionTrackChannels,
    format:         SessionTrackMediaFormat,
    seconds:        Type.Number(),
    sample_rate:    Type.Integer(),
    bytes:          Type.Integer(),
    url:            Type.String(),
    notify_url:     Option(Type.String()),
    context:        Option(Type.Any())
})
export type UploadToDomain = Static<typeof UploadToDomain>

export const DownloadFromDomain = Type.Object({
    url:            Type.String(),
    notify_url:     Option(Type.String()),
    context:        Option(Type.Any())
})
export type DownloadFromDomain = Static<typeof UploadToDomain>

export const MediaDownload = Type.Object({
    download:       DownloadFromDomain,
    state:          MediaJobState
})
export type MediaDownload = Static<typeof MediaDownload>

export const MediaUpload = Type.Object({
    download:       UploadToDomain,
    state:          MediaJobState
})
export type MediaUpload = Static<typeof MediaUpload>

export const MediaObject = Type.Object({
    id:             AppMediaObjectId,
    metadata:       Option(MediaMetadata),
    path:           Option(Type.String()),
    download:       Option(MediaDownload),
    upload:         Option(MediaUpload)
})
export type MediaObject = Static<typeof MediaObject>

export const UpdateMediaSession = Type.Object({
    media_objects:  Type.Array(AppMediaObjectId),
    ends_at:        JsonTimeStamp
})

export const MediaServiceCommand = Type.Union([
    Type.Object({
        "set_session_media":    AppSessionId,
        "media":                Type.Array(AppMediaObjectId)
    }),
    Type.Object({
        "delete_session":       AppSessionId
    })
])
export type MediaServiceCommand = Static<typeof MediaServiceCommand>
