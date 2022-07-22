import {Static, Type, TSchema} from "@sinclair/typebox";

export default function Option<T extends TSchema>(t: T) {
    return Type.Union([Type.Null(), t])
}