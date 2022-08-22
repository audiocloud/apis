import { Static, Type } from "@sinclair/typebox";
import { AppMediaObjectId } from "./new_types";
import { SessionTrackChannels, SessionTrackMediaFormat } from "./session";
import Option from "./utils/option";

const Uploading = Type.Object({
    progress:       Type.Number(),
    retry:          Type.Integer()
})
type Uploading = Static<typeof Uploading>


export const MediaDownloadState = Type.Union([
    Type.Literal('pending'),
    Type.Object({
        "downloading":      Type.Object({
            "progress":     Type.Number(),
            "retry":        Type.Integer()
        })
    }),
    Type.Literal('completed'),
    Type.Object({
        "failed":           Type.Object({
            "error":        Type.String(),
            "count":        Type.Integer(),
            "will_retry":   Type.Boolean()
        })
    }),
    Type.Literal('evicted')
])
export type MediaDownloadState = Static<typeof MediaDownloadState>

export const MediaUploadState = Type.Union([
    Type.Literal('pending'),
    Type.Object({
        "uploading":        Type.Object({
            "progress":     Type.Number(),
            "retry":        Type.Integer()
        })
    }),
    Type.Literal('completed'),
    Type.Object({
        "failed":           Type.Object({
            "error":        Type.String(),
            "count":        Type.Integer(),
            "will_retry":   Type.Boolean()
        })
    }),
])
export type MediaUploadState = Static<typeof MediaUploadState>

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

export const ImportInDomain = Type.Object({
    path:           Type.String(),
    channels:       SessionTrackChannels,
    format:         SessionTrackMediaFormat,
    seconds:        Type.Number(),
    sample_rate:    Type.Integer(),
    bytes:          Type.Integer()
})
export type ImportInDomain = Static<typeof ImportInDomain>

export const MediaObject = Type.Object({
    id:             AppMediaObjectId,
    metadata:       Option(MediaMetadata),
    path:           Option(Type.String()),
    download:       Option(MediaDownloadState),
    upload:         Option(MediaUploadState)
})
export type MediaObject = Static<typeof MediaObject>

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

