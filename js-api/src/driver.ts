import { Static, Type } from "@sinclair/typebox";
import Option from "./utils/option";
import { PlayId, RenderId } from "./change";
import { DesiredInstancePlayState, InstancePlayState } from "./instance";
import { ParameterId } from "./new_types";
import { InstanceReports } from "./session";
import { MultiChannelValue } from "./model";

export const InstanceDriverCommand = Type.Union([
    Type.Literal("check_connection"),
    Type.Literal("stop"),
    Type.Object({
        "play":                         Type.Object({
            "play_id":                  PlayId
        })
    }),
    Type.Object({
        "render":                       Type.Object({
            "length":                   Type.Number(),
            "render_id":                RenderId
        })
    }),
    Type.Object({
        "rewind":                       Type.Object({
            "to":                       Type.Number()
        })
    }),
    Type.Object({ "set_parameters":     Type.Record(ParameterId, MultiChannelValue) }),
])
export type InstanceDriverCommand = Static<typeof InstanceDriverCommand>

export const InstanceDriverError = Type.Union([
    Type.Object({
        "parameter_does_not_exist":     Type.Object({
            "parameter":                Type.String()
        })
    }),
    Type.Literal("media_not_present"),
    Type.Literal("not_interruptable"),
    Type.Object({
        "rpc":                          Type.Object({
            "error":                    Type.String()
        })
    })
])
export type InstanceDriverError = Static<typeof InstanceDriverError>

export const InstanceDriverEvent = Type.Union([
    /// Sent when the driver has started
    Type.Literal("started"),

    /// If an I/O error happened during communication with device
    Type.Object({ "io_error":           Type.Object({ "error": Type.String() }) }),

    /// Driver lost connection to the hardware
    Type.Literal("connection_lost"),

    /// Driver connected to the hardware
    Type.Literal("connected"),

    /// Received metering updates from the hardware
    Type.Object({ "reports":            Type.Object({ "reports": InstanceReports }) }),

    /// Playing; media current position reported
    Type.Object({
        "play_state":                   Type.Object({
            "desired":                  DesiredInstancePlayState,
            "current":                  InstancePlayState,
            "media":                    Option(Type.Number()),
        })
    }),
])
export type InstanceDriverEvent = Static<typeof InstanceDriverEvent>