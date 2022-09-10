import { Static, Type } from "@sinclair/typebox";
import { ModifySessionError } from "../change";
import { ResourceId } from "../model";
import { AppId, AppMediaObjectId, AppSessionId, DomainId, DynamicId, FixedId, FixedInstanceId, ModelId } from "../new_types";

export const CloudError = Type.Union([
    Type.Literal("api_key_not_found"),
    Type.Object({ "app_file_not_found":                 AppMediaObjectId }),
    Type.Object({ "app_not_found":                      AppId }),
    Type.Object({ "invalid_session_id":                 Type.String() }),
    Type.Object({ "invalid_media_id":                   Type.String() }),
    Type.Literal("only_future_reservations"),
    Type.Literal("time_malformed"),
    Type.Object({ "duration_too_short":                 Type.Number() }),
    Type.Literal("too_many_sessions"),
    Type.Object({ "internal_inconsistency":             Type.String() }),
    Type.Object({ "overlapping_fixed_instances":        Type.Array(FixedInstanceId)}),
    Type.Object({ "domain_not_found":                   DomainId }),
    Type.Object({ "instance_not_found":                 FixedInstanceId }),
    Type.Object({ "instance_not_referenced":            Type.Tuple([Type.Integer(), FixedInstanceId]) }),
    Type.Object({ "dynamic_instance_not_supported":     Type.Tuple([DynamicId, DomainId, ModelId]) }),
    Type.Object({ "fixed_instance_not_supported":       Type.Tuple([ FixedId, DomainId, FixedInstanceId]) }),
    Type.Object({ "fixed_instance_access_denied":       Type.Tuple([ FixedId, DomainId, FixedInstanceId, AppId]) }),
    Type.Object({ "out_of_resource":                    Type.Tuple([ ResourceId, Type.Number()]) }),
    Type.Object({ "object_too_short_lived":             Type.Tuple([DomainId, Type.Integer(), Type.Integer()]) }),
    Type.Object({ "session_not_found":                  AppSessionId }),
    Type.Object({ "session_modification":               ModifySessionError }),
    Type.Object({ "database":                           Type.String() }),
    Type.Object({ "authentication":                     Type.String() }),
    Type.Object({ "authorization":                      Type.String() }),
    Type.Literal("blocking_lock")
])
export type CloudError = Static<typeof CloudError>
