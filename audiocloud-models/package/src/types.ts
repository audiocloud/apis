export interface Stereo<T> {
    left: T,
    right: T
}

export type ToggleOr<T> = boolean | T

export type Tuple2<T>  = [T, T]
export type Tuple4<T>  = [T, T, T, T]
export type Tuple6<T>  = [T, T, T, T, T, T]
export type Tuple8<T>  = [T, T, T, T, T, T, T, T]
export type Tuple16<T> = [T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T]
export type Tuple24<T> = [T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T]

export type Either<A, B> = A | B
