import { Static, Type } from "@sinclair/typebox";
import Option from "../utils/option";
import { AppId, DomainId, MediaObjectId, SessionId } from "../new_types";
import { MediaDownloadState, MediaUploadState } from "../media";
import { RenderId } from "../change";

export const MediaPlacement = Type.Object({
    download:           Option(MediaDownloadState),
    uploaded:           Option(MediaUploadState),
})
export type MediaPlacement = Static<typeof MediaPlacement>

export const AppMedia = Type.Object({
    placements:         Type.Record(DomainId, MediaPlacement)
})
export type AppMedia = Static<typeof AppMedia>

export const CreateAppMedia = Type.Object({
    get_url:            Type.String(),
    grouping:           Type.Optional(Type.String()),
    grouping2:          Type.Optional(Type.String()),
    sync_to:            Type.Array(DomainId),
    context:            Type.String(),
    notify_url:         Type.String(),
})
export type CreateAppMedia = Static<typeof CreateAppMedia>

export const UpdateAppMedia = Type.Object({
    grouping:           Type.Optional(Type.String()),
    grouping2:          Type.Optional(Type.String()),
})
export type UpdateAppMedia = Static<typeof UpdateAppMedia>

export const QueryAppMedia = Type.Object({
    grouping_is:        Type.Optional(Type.String()),
    grouping_contains:  Type.Optional(Type.String()),
    grouping2_is:       Type.Optional(Type.String()),
    grouping2_contains: Type.Optional(Type.String()),
    domain_id:          Type.Optional(DomainId),
})
export type QueryAppMedia = Static<typeof QueryAppMedia>

export const ReportUploadState = Type.Object({
    app_id:             AppId,
    session_id:         SessionId,
    render_id:          RenderId,
    media_id:           MediaObjectId,
    state:              MediaUploadState,
})
export type ReportUploadState = Static<typeof ReportUploadState>

export const ReportDownloadState = Type.Object({
    app_id:             AppId,
    media_id:           MediaObjectId,
    state:              MediaDownloadState,
})
export type ReportDownloadState = Static<typeof ReportDownloadState>