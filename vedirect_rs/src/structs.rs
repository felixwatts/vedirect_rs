use bevy_reflect::Reflect;
use crate::enums::{AlarmReason, ConverterState, ErrorState, Mode, OffReason, TrackerOperationMode, WarnReason};
use serde::Serialize;
#[derive(Default,Debug, Serialize, Reflect, Clone)]
pub struct VEDirectBlock {
    pub pid: String,
    pub fw: String,
    pub serial: String,
    pub battery_volts: f32,
    pub battery_current: f32,
    pub load_current: f32,
    pub panel_volts: f32,
    pub panel_power: f32,
    pub converter_state: ConverterState,
    pub error_state: ErrorState,
    pub off_reason: OffReason,
    pub mppt_mode: TrackerOperationMode,
    pub load_status: bool,
    pub yield_total: f32,
    pub yield_today: f32,
    pub yield_yesterday: f32,
    pub maxpower_today: i32,
    pub maxpower_yesterday: i32,
    pub day_sequence_number: usize,

    pub relay_state: bool,
    pub alarm_reason: AlarmReason,
    pub mode: Mode,
    pub ac_out_v: f32,
    pub ac_out_i: f32,
    pub ac_out_s: f32,
    pub warn_reason: WarnReason
}
