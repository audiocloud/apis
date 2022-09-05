import { Static, Type, TSchema } from "@sinclair/typebox";

export const SerializableError = Type.Object({
  "err":          Type.Object({
    "code":       Type.Integer(),
    "message":    Type.String()
  })  
})
export type SerializableError = Static<typeof SerializableError>

export const SerializableOk = <T extends TSchema>(type: T) => Type.Object({ ok: type })

export const SerializableResult = <T extends TSchema>(type: T) => Type.Union([
  SerializableOk(type),
  SerializableError
])