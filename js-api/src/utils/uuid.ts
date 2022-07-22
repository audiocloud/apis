import { Static, Type } from "@sinclair/typebox";

export const Uuid = Type.String({format: 'uuid'})
export type Uuid = Static<typeof Uuid>