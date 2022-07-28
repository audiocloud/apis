// noinspection JSUnusedGlobalSymbols

import { TypeCompiler } from '@sinclair/typebox/compiler'
import { ValueError } from "@sinclair/typebox/value/errors";
import { CreateSession } from "./cloud/apps"

export interface ValidationError {
    errors: Array<ValueError>
}

export interface ValidationOk<T> {
    ok: T
}

export type ValidationResult<T> = ValidationOk<T> | ValidationError

export class AudioCloudValidators {
    private readonly check_create_session = TypeCompiler.Compile(CreateSession)

    validate_create_session(create_session: any): ValidationResult<CreateSession> {
        if (this.check_create_session.Check(create_session)) {
            return {ok: create_session}
        } else {
            return {errors: [...this.check_create_session.Errors(create_session)]}
        }
    }
}
