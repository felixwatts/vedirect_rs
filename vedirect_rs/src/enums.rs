use std::fmt::Display;
use bevy_reflect::Reflect;
use num_derive::FromPrimitive;
use thiserror::Error;
use strum::Display as StrumDisplay;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default, Reflect)]
pub struct AlarmReason(u32);

impl AlarmReason{
    pub const LOW_VOLTAGE: u32 = 1 << 0;
    pub const HIGH_VOLTAGE: u32 = 1 << 1;
    pub const LOW_SOC: u32 =  1 << 2;
    pub const LOW_STARTER_VOLTAGE: u32 =  1 << 3;
    pub const LOW_TEMPERATURE: u32 =  1 << 4;
    pub const HIGH_TEMPERATURE: u32 =  1 << 5;
    pub const MID_VOLTAGE: u32 =  1 << 6;
    pub const OVERLOAD: u32 =  1 << 7;
    pub const DC_RIPPLE: u32 =  1 << 8;
    pub const LOW_VAC_OUT: u32 =  1 << 9;
    pub const HIGH_VAC_OUT: u32 =  1 << 10;
    pub const SHORT_CIRCUIT: u32 =  1 << 11;
    pub const BMS_LOCKOUT: u32 =  1 << 12;

    pub fn includes_reason(&self, reason: u32) -> bool {
        (self.0 & reason) > 0
    }
}

impl Display for AlarmReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list_is_empty = true;
        let mut append = |s: &str| {
            if list_is_empty{
                list_is_empty = false;                
            } else {
                write!(f, " | ")?;
            }
            write!(f, "{}", s)
        };
        if self.includes_reason(Self::LOW_VOLTAGE) { append("LOW_VOLTAGE")?; };
        if self.includes_reason(Self::HIGH_VOLTAGE) { append("HIGH_VOLTAGE")?; };
        if self.includes_reason(Self::LOW_SOC) { append("LOW_SOC")?; };
        if self.includes_reason(Self::LOW_STARTER_VOLTAGE) { append("LOW_STARTER_VOLTAGE")?; };
        if self.includes_reason(Self::LOW_TEMPERATURE) { append("LOW_TEMPERATURE")?; };
        if self.includes_reason(Self::HIGH_TEMPERATURE) { append("HIGH_TEMPERATURE")?; };
        if self.includes_reason(Self::MID_VOLTAGE) { append("MID_VOLTAGE")?; };
        if self.includes_reason(Self::OVERLOAD) { append("OVERLOAD")?; };
        if self.includes_reason(Self::DC_RIPPLE) { append("DC_RIPPLE")?; };
        if self.includes_reason(Self::LOW_VAC_OUT) { append("LOW_VAC_OUT")?; };
        if self.includes_reason(Self::HIGH_VAC_OUT) { append("HIGH_VAC_OUT")?; };
        if self.includes_reason(Self::SHORT_CIRCUIT) { append("SHORT_CIRCUIT")?; };
        if self.includes_reason(Self::BMS_LOCKOUT) { append("BMS_LOCKOUT")?; };

        Ok(())
    }
}

pub type WarnReason = AlarmReason;

impl From<u32> for AlarmReason{
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[test]
fn test_alarm_reason() {
    let subject = AlarmReason(AlarmReason::OVERLOAD | AlarmReason::LOW_TEMPERATURE);
    assert_eq!(true, subject.includes_reason(AlarmReason::OVERLOAD));
    assert_eq!(true, subject.includes_reason(AlarmReason::LOW_TEMPERATURE));
    assert_eq!(false, subject.includes_reason(AlarmReason::SHORT_CIRCUIT));
}

#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize, Reflect, Clone)]
pub enum Mode{
    #[default]
    Charger = 1,
    Inverter = 2,
    Off = 4,
    Eco = 5,
    Hibernate = 253
}

#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize, Reflect, Clone)]
pub enum ConverterState {
    #[default]
    Off = 0,
    LowPower = 1,
    Fault = 2,
    Bulk = 3,
    Absorption = 4,
    Float = 5,
    Storage = 6,
    Equalize = 7,
    Inverting = 8,
    PowerSupply = 11,
    StartingUp = 245,
    RepeatedAbsorption = 246,
    AutoEqualize = 247,
    BatterySafe = 248,
    ExternalControl = 252
}
#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize, Reflect, Clone)]
pub enum ErrorState {
    #[default]
    NoError = 0,
    BatteryVoltsTooHigh = 2,
    ChargerTemperatureTooHigh = 17,
    ChargerOverCurrent = 18,
    ChargerCurrentReversed =19,
    BulkTimeLimitExceeded = 20,
    CurrentSensorIssue = 21,
    TerminalsOverheated = 26,
    ConverterIssue = 28,
    InputVoltageTooHigh = 33,
    InputCurrentTooHigh = 34,
    InputShutdownExcessBatteryVoltage = 38,
    InputShutdownCurrentFlowWhileOff = 39,
    LostCommunicationWithOneOfDevices = 65,
    SynchronisedChargingDeviceConfigurationIssue = 66,
    BMSConnectionLost = 67,
    NetworkMisconfigured = 68,
    FactoryCalibrationDataLost = 116,
    InvalidFirmware = 117,
    UserSettingsInvalid = 119
}

#[derive(Default,StrumDisplay,FromPrimitive, Serialize, Reflect, Clone)]
pub enum ModeState {
    Charger = 1,
    Inverter = 2,
    #[default]
    Off = 4,
    Eco = 5,
    Hibernate = 253
}

#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize, Reflect, Clone)]
pub enum OffReason {
    #[default]
    NoValue = 0x0,
    NoInputPower = 0x1,
    SwitchedOffPowerSwitch = 0x2,
    SwitchedOffDeviceModeRegister = 0x4,
    RemoteInput = 0x8,
    ProtectionActive = 0x10,
    Paygo = 0x20,
    BMS = 0x40,
    EngineShutdownDetection = 0x80,
    AnalysingInputVoltage = 0x100
}
#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize, Reflect, Clone)]
pub enum TrackerOperationMode {
    #[default]
    Off = 0,
    VoltageOrCurrentLimited = 1,
    MPPTActive = 2
}
#[derive(Error, Debug, Serialize)]
pub enum ExtractError {
    #[error("Incomplete data available")]
    Incomplete,
    #[error("couldn't find a match in data")]
    NoMatch,
    #[error("general failure")]
    Failure,
    #[error("extract-error-passthrough: {0}")]
    PassThrough(String)
}
