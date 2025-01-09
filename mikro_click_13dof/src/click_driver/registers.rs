use crate::click_driver::types::{Register, ReadRegister, WriteRegister};

pub const ACCEL_ADDR: u8 = 0b001_1000;
pub const GYRO_ADDR: u8 = 0b110_1000;
pub const MAG_ADDR: u8 = 0b001_0000;
const ENV_ADDR: u8 = 0b111_0110;

/// registers for the BME680 temperature sensor
#[allow(unused, non_camel_case_types)]
pub enum BME680 {
    //RESET = 0xE0, //RW
    ID = 0xD0, //RO
    //CONFIG = 0x75, //RW
    CTRL_MEAS = 0x74, //RW
    CTRL_HUM = 0x72, //RW
    CTRL_GAS1 = 0x71, //RW
    CTRL_GAS0 = 0x70, //RW
}

impl Into<u8> for BME680 {
    fn into (self) -> u8 {
        self as u8
    }
}

/// Registers for the BMI088 accelerometer
#[allow(unused, non_camel_case_types)]
pub enum BMI088_ACCEL {
    ID = 0x00,
}


impl Into<u8> for BMI088_ACCEL {
    fn into (self) -> u8 {
        self as u8
    }
}

/// Registers for the BMI088 accelerometer
#[allow(unused, non_camel_case_types)]
pub enum BMI088_GYRO {
    ID = 0x00,
}


impl Into<u8> for BMI088_GYRO {
    fn into (self) -> u8 {
        self as u8
    }
}

/// Registers for the BMM150
#[allow(unused, non_camel_case_types)]
pub enum BMM150 {
    ID = 0x40,
    POWER = 0x4b,
}

impl Into<u8> for BMM150 {
    fn into (self) -> u8 {
        self as u8
    }
}

// REGISTER IMPLEMENTATIONS

//ENVIRONMENTAL

/// Oversampling enum
#[allow(dead_code)]
pub enum EnvOversample {
    None = 0,
    OS1 = 0b001,
    OS2 = 0b010,
    OS4 = 0b011,
    OS8 = 0b100,
    OS16 = 0b101
}

/// Environmental Sensor id register and check
#[derive(Default)]
pub struct EnvId {
    data: u8,
}

impl Register for EnvId {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::ID as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for EnvId{
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl EnvId {
    pub fn is_id_correct(&self) -> bool {
        self.data == 0x61 as u8
    }
}

/// Environmental Sensor Control measurement register
#[derive(Default)]
pub struct EnvCtrlMeas {
    data: u8,
}

impl Register for EnvCtrlMeas {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::CTRL_MEAS as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for EnvCtrlMeas {
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl WriteRegister for EnvCtrlMeas {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

impl EnvCtrlMeas {
    pub fn set_temperature_oversample(&mut self, sampling: EnvOversample) {
        self.data = (self.data & 0b00011111) + ((sampling  as u8) << 5);
    }

    pub fn set_pressure_oversample(&mut self, sampling: EnvOversample) {
        self.data = (self.data & 0b11100011) + ((sampling as u8) << 2);
    }

    pub fn start_sample(&mut self, start: bool) {
        self.data = self.data & 0b11111100;
        if start {
            self.data += 1;
        } 
    }
}

/// Environmental Sensor Humidity Control Register
#[derive(Default)]
pub struct EnvCtrlHumid {
    data: u8,
}

impl Register for EnvCtrlHumid {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::CTRL_HUM as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for EnvCtrlHumid {
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl WriteRegister for EnvCtrlHumid {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

impl EnvCtrlHumid {
    pub fn set_humidity_oversample(&mut self, sampling: EnvOversample) {
        self.data = sampling as u8;
    }
}

/// Environmental Sensor Gas Control Register 0
/// We don't want to use the gas sensor, so just turn if off
#[derive(Default)]
pub struct EnvCtrlGas0 {
    data: u8,
}

impl Register for EnvCtrlGas0 {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::CTRL_GAS0 as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for EnvCtrlGas0 { 
    fn set_raw_data(&mut self, _data: &[u8]) {
        self.data = 1<<3; //turns off heater
    }
}

impl WriteRegister for EnvCtrlGas0 {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

//ACCELEROMETER
/// Accelerometer id register and check
#[derive(Default)]
pub struct AccId {
    data: u8,
}

impl Register for AccId {
    const DEVICE: u8 = ACCEL_ADDR;
    const REGISTER: u8 = BMI088_ACCEL::ID as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for AccId {
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl AccId {
    pub fn is_id_correct(&self) -> bool {
        self.data == 0x1e as u8
    }
}

//GYROSCOPE
/// Gyroscope id register and check
#[derive(Default)]
pub struct GyroId {
    data: u8,
}

impl Register for GyroId {
    const DEVICE: u8 = GYRO_ADDR;
    const REGISTER: u8 = BMI088_GYRO::ID as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for GyroId {
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl GyroId {
    pub fn is_id_correct(&self) -> bool {
        self.data == 0x0f as u8
    }
}

//MAGNETOMETER
/// Magnetometer id register and check
#[derive(Default)]
pub struct MagId {
    data: u8,
}

impl Register for MagId {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::ID as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for MagId{
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl MagId {
    pub fn is_id_correct(&self) -> bool {
        self.data == 0x32 as u8
    }
}

/// Magnetometer power on. This struct is RO and functions to only power on the magnetometer.
pub struct MagPower {
    data: u8,
}

impl Default for MagPower {
    fn default() -> Self { Self{ data: 1 } }
}

impl Register for MagPower {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::POWER as u8;
    const SIZE: u8 = 1;
}

impl WriteRegister for MagPower {
    fn get_raw_data(&self) -> u8 {
        return self.data;
    }
}

