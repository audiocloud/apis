import { Static, Type } from "@sinclair/typebox";
import { ModelId } from "../new_types";
import Option from "../utils/option";

export const ModelFilter = Type.Object({
    manufacturer_is: Option(Type.String()),
    name_contains:   Option(Type.String()),
    id_one_of:       Type.Array(ModelId),
})
export type ModelFilter = Static<typeof ModelFilter>