/// FTMS data structure
/// Used to represent the data received from FTMS devices
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FTMSData {
    pub speed: f32,
    pub cadence: f32,
    pub distance: f32,
    pub resistance: f64,
    pub power: u8,
    pub calories: f64,
    pub heart_rate: f64,
    pub time: u16,
}

/// FTMS control operation codes
///
/// Taken from https://github.com/jetoneza/cycling_trainer/blob/main/src-tauri/src/ble/constants.rs#L24-L32
#[allow(dead_code)]
pub enum FTMSControlOpCode {
    RequestControl = 0x00,
    TargetPower = 0x05,
    Start = 0x07,
    Stop = 0x08,
    SpinDownControl = 0x13,
    TargetCadence = 0x14,
    Success = 0x80,
}

/// Stop codes, bytes used to communicate a desire to pause or stop a session.
///
/// Taken from https://github.com/jetoneza/cycling_trainer/blob/main/src-tauri/src/ble/constants.rs#L47-L50
#[allow(dead_code)]
pub enum StopCode {
    Stop = 0x01,
    Pause = 0x02,
}
