import {Static, Type} from "@sinclair/typebox";

export const JsonTimeStamp = Type.String({format: 'date-time'})
export type JsonTimeStamp = Static<typeof JsonTimeStamp>

export const JsonTimeRange = Type.Object({
    from: JsonTimeStamp,
    to: JsonTimeStamp
})
export type JsonTimeRange = Static<typeof JsonTimeRange>

export type TimeRange = FromJsonTimeStamps<JsonTimeRange, 'from' | 'to'>

export type ToJsonTimeStamps<T extends object, K extends keyof T> = {
    [k in keyof T]: k extends K ? JsonTimeStamp : T[k]
}

export type ToJsonTimeRanges<T extends object, K extends keyof T> = {
    [k in keyof T]: k extends K ? JsonTimeRange : T[k]
}

export type FromJsonTimeStamps<T extends object, K extends keyof T> = {
    [k in keyof T]: k extends K ? Date : T[k]
}

export type FromJsonTimeRanges<T extends object, K extends keyof T> = {
    [k in keyof T]: k extends K ? TimeRange : T[k]
}
