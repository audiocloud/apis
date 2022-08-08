import { Static, Type } from "@sinclair/typebox";

export const DomainId = Type.String({minLength: 8, maxLength: 32})
export type DomainId = Static<typeof DomainId>

export const AppId = Type.String({minLength: 8, maxLength: 32})
export type AppId = Static<typeof AppId>

export const SessionId = Type.String({minLength: 8, maxLength: 32})
export type SessionId = Static<typeof SessionId>

export const SocketId = Type.String({minLength: 8})
export type SocketId = Static<typeof SocketId>

export const AppSessionId = Type.RegEx(/[\w-_]{8,32}\/[\w-_]{8,32}/)
export type AppSessionId = Static<typeof AppSessionId>

export const FixedInstanceId = Type.RegEx(/[\w-_]{1,32}\/[\w-_]{1,32}\/[\w-_]{1,32}/)
export type FixedInstanceId = Static<typeof FixedInstanceId>

export const ModelId = Type.RegEx(/[\w-_]{1,32}\/[\w-_]{1,32}/)
export type ModelId = Static<typeof ModelId>

export const FilterId = Type.Union([
    Type.Literal("high_pass"),
    Type.Literal("low"),
    Type.Literal("low_mid"),
    Type.Literal("mid"),
    Type.Literal("high_mid"),
    Type.Literal("high"),
    Type.Literal("low_pass"),
    Type.Literal("band_pass"),
    Type.Literal("dynamics"),
    Type.Literal("de_esser"),
])
export type FilterId = Static<typeof FilterId>

export const TrackId = Type.String({minLength: 1})
export type TrackId = Static<typeof TrackId>

export const MixerId = Type.String({minLength: 1})
export type MixerId = Static<typeof MixerId>

export const DynamicId = Type.String({minLength: 1})
export type DynamicId = Static<typeof DynamicId>

export const FixedId = Type.String({minLength: 1})
export type FixedId = Static<typeof FixedId>

export const ConnectionId = Type.String({minLength: 1})
export type ConnectionId = Static<typeof ConnectionId>

export const MediaId = Type.String({minLength: 1})
export type MediaId = Static<typeof MediaId>

export const SecureKey = Type.String({minLength: 8})
export type SecureKey = Static<typeof SecureKey>

export const MediaObjectId = Type.String({minLength: 8})
export type MediaObjectId = Static<typeof MediaObjectId>

export const ParameterId = Type.RegEx(/[\w-_]{1,32}/)
export type ParameterId = Static<typeof ParameterId>

export const ReportId = Type.RegEx(/[\w-_]{1,32}/)
export type ReportId = Static<typeof ReportId>

export const AppMediaObjectId = Type.String(/[\w-_]{8,32}\/[\w-_]{8,}/)
export type AppMediaObjectId = Static<typeof AppMediaObjectId>