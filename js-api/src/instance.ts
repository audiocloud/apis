import {Static, Type} from "@sinclair/typebox";
import {ParameterId} from "./new_types";
import {InstanceValue} from "./model";

export const MultiChannelValue = Type.Array(Type.Tuple([
    Type.Integer({minimum: 0, maximum: 64}),
    InstanceValue
]))
export type MultiChannelValue = Static<typeof MultiChannelValue>

export const InstanceParameters = Type.Record(ParameterId, MultiChannelValue)
export type InstanceParameters = Static<typeof InstanceParameters>
