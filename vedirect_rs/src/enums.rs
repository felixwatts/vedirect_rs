use num_derive::FromPrimitive;
use thiserror::Error;
use strum::Display as StrumDisplay;
use serde::Serialize;
#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize)]
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
#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize)]
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

#[derive(Default,StrumDisplay,FromPrimitive, Serialize)]
pub enum ModeState {
    Charger = 1,
    Inverter = 2,
    #[default]
    Off = 4,
    Eco = 5,
    Hibernate = 253
}

#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize)]
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
#[derive(Default, StrumDisplay, FromPrimitive, Debug, Serialize)]
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
}
