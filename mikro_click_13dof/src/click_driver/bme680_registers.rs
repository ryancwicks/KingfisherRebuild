use crate::click_driver::types::{Register, ReadRegister, WriteRegister};

//ENVIRONMENTAL
const ENV_ADDR: u8 = 0b111_0110;

/// registers for the BME680 temperature sensor
#[allow(unused, non_camel_case_types)]
pub enum BME680 {
    //RESET = 0xE0, //RW
    ID = 0xD0, //RO
    CONFIG = 0x75, //RW
    CTRL_MEAS = 0x74, //RW
    CTRL_HUM = 0x72, //RW
    CTRL_GAS1 = 0x71, //RW
    CTRL_GAS0 = 0x70, //RW
    TEMPERATURE = 0x22, //RO, 3 bytes
    PRESSURE = 0x1F, //RO. 3 bytes
    HUMIDITY = 0x25, //RO, 2 bytes
    
    //Constant cal params, all RO
    COEFFS = 0x8a, //42 parameters
}

impl Into<u8> for BME680 {
    fn into (self) -> u8 {
        self as u8
    }
}


/// Oversampling enum
#[allow(dead_code)]
pub enum Oversample {
    None = 0,
    OS1 = 0b001,
    OS2 = 0b010,
    OS4 = 0b011,
    OS8 = 0b100,
    OS16 = 0b101
}

//Filter enum
#[allow(dead_code)]
pub enum Filter {
    None = 0,
    F1 = 1,
    F3 = 2,
    F7 = 3,
    F15 = 4,
    F31 = 5,
    F63 = 6,
    F127 = 7
    
}

//coefficients
#[allow(non_camel_case_types)]
enum ConstantCoefficients1 {
    BME68X_IDX_T2_LSB =                        0,
    BME68X_IDX_T2_MSB =                        1,
    BME68X_IDX_T3     =                        2,
    BME68X_IDX_P1_LSB =                        4,
    BME68X_IDX_P1_MSB =                        5,
    BME68X_IDX_P2_LSB =                        6,
    BME68X_IDX_P2_MSB =                        7,
    BME68X_IDX_P3     =                        8,
    BME68X_IDX_P4_LSB =                        10,
    BME68X_IDX_P4_MSB =                        11,
    BME68X_IDX_P5_LSB =                        12,
    BME68X_IDX_P5_MSB =                        13,
    BME68X_IDX_P7     =                        14,
    BME68X_IDX_P6     =                        15,
    BME68X_IDX_P8_LSB =                        18,
    BME68X_IDX_P8_MSB =                        19,
    BME68X_IDX_P9_LSB =                        20,
    BME68X_IDX_P9_MSB =                        21,
    BME68X_IDX_P10    =                        22,
}

impl Into<usize> for ConstantCoefficients {
    fn into (self) -> usize {
        self as usize
    }
}

/// Environmental sensor Cal parameters
/// Get the calibration coefficients
pub struct Coeffs {
    pub data: [u8; 23],
}

impl Default for Coeffs {
    fn default() -> Self { Self{ data: [0; 23] } }
}

impl Register for Coeffs {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::COEFFS as u8;
    const SIZE: u8 = 23;
}

impl ReadRegister for Coeffs{
    fn set_raw_data(&mut self, data: &[u8]) {
        for (i, val) in data.iter().enumerate() {
            self.data[i] = *val;
        }
    }
}

impl Coeffs {
    pub fn t1(&self) -> f32 {
        (self.data[ConstantCoefficients::BME68X_IDX_T1_LSB as usize] as u16 + (self.data[ConstantCoefficients::BME68X_IDX_T1_MSB as usize] as u16) << 8) as f32
    }

    pub fn t2(&self) -> f32 {
        (self.data[ConstantCoefficients::BME68X_IDX_T2_LSB as usize] as u16 + (self.data[ConstantCoefficients::BME68X_IDX_T2_MSB as usize] as u16) << 8) as f32
    }

    pub fn t3(&self) -> f32 {
        self.data[ConstantCoefficients::BME68X_IDX_T3 as usize] as f32
    }



}


/// Environmental Sensor id register and check
#[derive(Default)]
pub struct Id {
    data: u8,
}

impl Register for Id {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::ID as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for Id{
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl Id {
    pub fn is_id_correct(&self) -> bool {
        self.data == 0x61 as u8
    }
}

/// Environmental Sensor Control measurement register
#[derive(Default)]
pub struct CtrlMeas {
    data: u8,
}

impl Register for CtrlMeas {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::CTRL_MEAS as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for CtrlMeas {
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl WriteRegister for CtrlMeas {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

impl CtrlMeas {
    pub fn set_temperature_oversample(&mut self, sampling: Oversample) {
        self.data = (self.data & 0b00011111) + ((sampling  as u8) << 5);
    }
    
    pub fn set_pressure_oversample(&mut self, sampling: Oversample) {
        self.data = (self.data & 0b11100011) + ((sampling as u8) << 2);
    }
    
    pub fn start_sample(&mut self, start: bool) {
        self.data = self.data & 0b11111100;
        if start {
            self.data += 1;
        } 
    }
}

///Environmental Sensor Config Register
/// mostly for setting temp and pressure filtering
#[derive(Default)]
pub struct Config {
    data: u8,
}

impl Register for Config {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::CONFIG as u8;
    const SIZE: u8 = 1;
}

impl WriteRegister for Config {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

impl Config {
    pub fn set_filter(&mut self, filter_num: Filter) {
        self.data = (filter_num as u8) << 2; 
    }
}

/// Environmental Sensor Humidity Control Register
#[derive(Default)]
pub struct CtrlHumid {
    data: u8,
}

impl Register for CtrlHumid {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::CTRL_HUM as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for CtrlHumid {
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl WriteRegister for CtrlHumid {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

impl CtrlHumid {
    pub fn set_humidity_oversample(&mut self, sampling: Oversample) {
        self.data = sampling as u8;
    }
}

/// Environmental Sensor Gas Control Register 0
/// We don't want to use the gas sensor, so just turn if off
#[derive(Default)]
pub struct CtrlGas0 {
    data: u8,
}

impl Register for CtrlGas0 {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::CTRL_GAS0 as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for CtrlGas0 { 
    fn set_raw_data(&mut self, _data: &[u8]) {
        self.data = 1<<3; //turns off heater
    }
}

impl WriteRegister for CtrlGas0 {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

/// Environmental Temperature Sensor
#[derive(Default)]
pub struct Temperature {
    data: [u8; 3],
}

impl Register for Temperature {
    const DEVICE: u8 = ENV_ADDR;
    const REGISTER: u8 = BME680::TEMPERATURE as u8;
    const SIZE: u8 = 3; 
}

impl ReadRegister for Temperature {
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data[0] = data[0];
        self.data[1] = data[1];
        self.data[2] = data[2];
    }
}

impl Temperature {
    //Get 20 bit temperature - assuming iir is enabled (which is should be)
    pub fn get_raw_temperature(&self) -> u32 {
        ((self.data[0] as u32) << 12) + ((self.data[1] as u32) << 4) + ((self.data[2] as u32) >> 4)
    }
    
}

//Environmental Pressure Sensor

//Environmental Humidity Sensor