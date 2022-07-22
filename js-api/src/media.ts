import { Static, Type } from "@sinclair/typebox";
import Option from "./utils/option";

const Pending = Type.Literal("pending")
type Pending = Static<typeof Pending>

const Downloading = Type.Object({
    progress:       Type.Number(),
    retry:          Type.Integer()
})
type Downloading = Static<typeof Downloading>

const Uploading = Type.Object({
    progress:       Type.Number(),
    retry:          Type.Integer()
})
type Uploading = Static<typeof Uploading>

const Completed = Type.Literal("completed")
type Completed = Static<typeof Completed>

const Failed = Type.Object({
    error:          Type.String(),
    count:          Type.Integer(),
    will_retry:     Type.Boolean()
})
type Failed = Static<typeof Failed>

const Evicted = Type.Literal("evicted")
type Evicted = Static<typeof Evicted>

export const MediaDownloadState = Type.Union([
    Pending,
    Type.Object({ "downloading": Downloading }),
    Completed,
    Type.Object({ "failed": Failed}),
    Evicted
])
export type MediaDownloadState = Static<typeof MediaDownloadState>

export const MediaUploadState = Type.Union([
    Pending,
    Type.Object({ "uploading": Uploading }),
    Completed,
    Type.Object({ "failed": Failed})
])
export type MediaUploadState = Static<typeof MediaUploadState>

export const DownloadMedia = Type.Object({
    get_url:        Type.String(),
    notify_url:     Type.String(),
    context:        Type.String()
})
export type DownloadMedia = Static<typeof DownloadMedia>