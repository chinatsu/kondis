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

pub enum FTMSControlOpCode {
    RequestControl = 0x00,
    TargetPower = 0x05,
    // Start = 0x07,
    Stop = 0x08,
    // SpinDownControl = 0x13,
    TargetCadence = 0x14,
    // Success = 0x80,
}

pub enum StopCode {
    Stop = 0x01,
    // Pause = 0x02,
}
