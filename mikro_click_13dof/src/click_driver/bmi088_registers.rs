use crate::click_driver::types::{Register, ReadRegister, WriteRegister};

pub const ACCEL_ADDR: u8 = 0b001_1000;
pub const GYRO_ADDR: u8 = 0b110_1000;


/// Registers for the BMI088 accelerometer
#[allow(unused, non_camel_case_types)]
pub enum BMI088_ACCEL {
    ID = 0x00,
    CONF = 0x7c,
    CTRL = 0x7d,
    DATA = 0x12, //6 bytes
}


impl Into<u8> for BMI088_ACCEL {
    fn into (self) -> u8 {
        self as u8
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

/// Set power mode register (only sets active)
pub struct AccPowerConf {
    data: u8,
}

impl Default for AccPowerConf {
    fn default() -> Self {
        Self { data: 0x00 }
    }
}

impl Register for AccPowerConf {
    const DEVICE: u8 = ACCEL_ADDR;
    const REGISTER: u8 = BMI088_ACCEL::CONF as u8;
    const SIZE: u8 = 1;
}

impl WriteRegister for AccPowerConf {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

// Power on accelerometer register (only sets to on)
pub struct AccPowerCtrl {
    data: u8,
}

impl Default for AccPowerCtrl {
    fn default() -> Self {
        Self { data: 0x04 } //accelerometer on.
    }
}

impl Register for AccPowerCtrl {
    const DEVICE: u8 = ACCEL_ADDR;
    const REGISTER: u8 = BMI088_ACCEL::CTRL as u8;
    const SIZE: u8 = 1;
}

impl WriteRegister for AccPowerCtrl {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

/// Data read register. This assumes the default 6G range.
pub struct AccData {
    data: [u8; 6]
}

impl Default for AccData {
    fn default() -> Self {
        Self {data: [0; 6]}
    }
}

impl Register for AccData {
    const DEVICE: u8 = ACCEL_ADDR;
    const REGISTER: u8 = BMI088_ACCEL::DATA as u8;
    const SIZE: u8 = 6;
}

impl ReadRegister for AccData {
    fn set_raw_data(&mut self, data: &[u8]) {
        for (i, val) in data.iter().enumerate() {
            self.data[i] = *val;
        }
    }
}

impl AccData {
    //Get the raw accelerometer data in G's (assumes default range for accelometer of +/- 6G)
    pub fn get_data(&self) -> (f32, f32, f32) {
        let raw_x = (((self.data[1] as u16) << 8) + self.data[0] as u16) as i16;
        let raw_y = (((self.data[3] as u16) << 8) + self.data[2] as u16) as i16;
        let raw_z = (((self.data[5] as u16) << 8) + self.data[4] as u16) as i16;

        //Accel_X_in_mg = Accel_X_int16 / 32768 * 1000 * 2^(<0x41> + 1)*1.5
        let x = raw_x as f32 / 32768.0 * 15.0 * 4.0;
        let y = raw_y as f32 / 32768.0 * 15.0 * 4.0;
        let z = raw_z as f32 / 32768.0 * 15.0 * 4.0;

        (x, y, z)
    }
}




//GYROSCOPE
/// Registers for the BMI088 Gyro
#[allow(unused, non_camel_case_types)]
pub enum BMI088_GYRO {
    ID = 0x00,
    DATA = 0x02,
}


impl Into<u8> for BMI088_GYRO {
    fn into (self) -> u8 {
        self as u8
    }
}


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

/// Data Gyro read register. This assumes the default range.
pub struct GyroData {
    data: [u8; 6]
}

impl Default for GyroData {
    fn default() -> Self {
        Self {data: [0; 6]}
    }
}

impl Register for GyroData {
    const DEVICE: u8 = GYRO_ADDR;
    const REGISTER: u8 = BMI088_GYRO::DATA as u8;
    const SIZE: u8 = 6;
}

impl ReadRegister for GyroData {
    fn set_raw_data(&mut self, data: &[u8]) {
        for (i, val) in data.iter().enumerate() {
            self.data[i] = *val;
        }
    }
}

impl GyroData {
    //Get the raw accelerometer data in G's (assumes default range for accelometer of +/- 6G)
    pub fn read_gyro_data(&self) -> (f32, f32, f32) {
        let raw_x = (((self.data[1] as u16) << 8) + self.data[0] as u16) as i16;
        let raw_y = (((self.data[3] as u16) << 8) + self.data[2] as u16) as i16;
        let raw_z = (((self.data[5] as u16) << 8) + self.data[4] as u16) as i16;

        
        let x = (raw_x as f32 ) * 4000.0/ 65536.0;
        let y = (raw_y as f32 ) * 4000.0/ 65536.0;
        let z = (raw_z as f32 ) * 4000.0/ 65536.0;

        (x, y, z)
    }
}