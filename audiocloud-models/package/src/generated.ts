import * as T from "./types"



export interface CloudInsert1X1Preset {}
export interface CloudInsert1X1Preset {}
export interface CloudInsert1X1Reports {
    insert_input: number,
    insert_output: number,}

export interface CloudInsert2X2Preset {}
export interface CloudInsert2X2Preset {}
export interface CloudInsert2X2Reports {
    insert_output: T.Stereo<number>,
    insert_input: T.Stereo<number>,}

export interface CloudInsert24X2Preset {}
export interface CloudInsert24X2Preset {}
export interface CloudInsert24X2Reports {
    insert_input: T.Tuple24<number>,
    insert_output: T.Stereo<number>,}



export interface Dual1084Preset {
    high_mid_freq: T.Stereo<T.ToggleOr<number>>,
    low_freq: T.Stereo<T.ToggleOr<number>>,
    high_mid_width: T.Stereo<boolean>,
    high_mid_gain: T.Stereo<number>,
    eql_toggle: T.Stereo<boolean>,
    low_mid_freq: T.Stereo<T.ToggleOr<number>>,
    output_pad: T.Stereo<T.ToggleOr<number>>,
    high_pass_filter: T.Stereo<T.ToggleOr<number>>,
    high_freq: T.Stereo<T.ToggleOr<number>>,
    low_mid_width: T.Stereo<boolean>,
    low_mid_gain: T.Stereo<number>,
    low_gain: T.Stereo<number>,
    input_gain: T.Stereo<T.ToggleOr<number>>,
    high_gain: T.Stereo<number>,}
export interface Dual1084Preset {
    high_mid_freq: T.Stereo<T.ToggleOr<number>>,
    low_freq: T.Stereo<T.ToggleOr<number>>,
    high_mid_width: T.Stereo<boolean>,
    high_mid_gain: T.Stereo<number>,
    eql_toggle: T.Stereo<boolean>,
    low_mid_freq: T.Stereo<T.ToggleOr<number>>,
    output_pad: T.Stereo<T.ToggleOr<number>>,
    high_pass_filter: T.Stereo<T.ToggleOr<number>>,
    high_freq: T.Stereo<T.ToggleOr<number>>,
    low_mid_width: T.Stereo<boolean>,
    low_mid_gain: T.Stereo<number>,
    low_gain: T.Stereo<number>,
    input_gain: T.Stereo<T.ToggleOr<number>>,
    high_gain: T.Stereo<number>,}
export interface Dual1084Reports {}

export interface SummatraPreset {
    input: T.Tuple24<number>,
    pan: T.Tuple24<number>,
    bus_assign: T.Tuple24<number>,}
export interface SummatraPreset {
    input: T.Tuple24<number>,
    pan: T.Tuple24<number>,
    bus_assign: T.Tuple24<number>,}
export interface SummatraReports {}



export interface PowerPdu4CPreset {
    power: T.Tuple4<boolean>,}
export interface PowerPdu4CPreset {
    power: T.Tuple4<boolean>,}
export interface PowerPdu4CReports {
    power: T.Tuple4<boolean>,
    power_factor: T.Tuple4<number>,
    energy: T.Tuple4<number>,
    current: T.Tuple4<number>,}

