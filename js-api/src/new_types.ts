import {Static, Type} from "@sinclair/typebox";

export const DomainId = Type.String({minLength: 8, maxLength: 32})
export type DomainId = Static<typeof DomainId>

export const AppId = Type.String({minLength: 8, maxLength: 32})
export type AppId = Static<typeof DomainId>

export const ModelId = Type.RegEx(/[\w-_]{1,32}\/[\w-_]{1,32}/)
export type ModelId = Static<typeof ModelId>

export const FixedInstanceId = Type.RegEx(/[\w-_]{1,32}\/[\w-_]{1,32}\/[\w-_]{1,32}/)
export type FixedInstanceId = Static<typeof FixedInstanceId>

export const TrackId = Type.String({minLength: 1})
export type TrackId = Static<typeof TrackId>

export const MixerId = Type.String({minLength: 1})
export type MixerId = Static<typeof MixerId>

export const DynamicId = Type.String({minLength: 1})
export type DynamicId = Static<typeof DynamicId>

export const FixedId = Type.String({minLength: 1})
export type FixedId = Static<typeof FixedId>

export const MediaId = Type.String({minLength: 1})
export type MediaId = Static<typeof MediaId>

export const InputId = Type.String({minLength: 1})
export type InputId = Static<typeof InputId>

export const SecureKey = Type.String({minLength: 8})
export type SecureKey = Static<typeof SecureKey>

export const MediaObjectId = Type.String({minLength: 8})
export type MediaObjectId = Static<typeof MediaObjectId>

export const ParameterId = Type.RegEx(/[\w-_]{1,32}/)
export type ParameterId = Static<typeof ParameterId>

export const ReportId = Type.RegEx(/[\w-_]{1,32}/)
export type ReportId = Static<typeof ReportId>
