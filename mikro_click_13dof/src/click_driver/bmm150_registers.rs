use crate::click_driver::types::{Register, ReadRegister, WriteRegister};

pub const MAG_ADDR: u8 = 0b001_0000;


/// Registers for the BMM150
#[allow(unused, non_camel_case_types)]
pub enum BMM150 {
    ID = 0x40,
    POWER = 0x4b,
    DATA = 0x42, // 8 bytes
    CONTROL = 0x4c,
    DIGX1 = 0x5D,
    DIGZ4 = 0x62,
    DIGZ2 = 0x68
}

impl Into<u8> for BMM150 {
    fn into (self) -> u8 {
        self as u8
    }
}

//MAGNETOMETER
/// Magnetometer id register and check
#[derive(Default)]
pub struct Id {
    data: u8,
}

impl Register for Id {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::ID as u8;
    const SIZE: u8 = 1;
}

impl ReadRegister for Id{
    fn set_raw_data(&mut self, data: &[u8]) {
        self.data = data[0];
    }
}

impl Id {
    pub fn is_id_correct(&self) -> bool {
        self.data == 0x32 as u8
    }
}

/// Magnetometer power on. This struct is RO and functions to only power on the magnetometer.
pub struct Power {
    data: u8,
}

impl Default for Power {
    fn default() -> Self { Self{ data: 1 } }
}

impl Register for Power {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::POWER as u8;
    const SIZE: u8 = 1;
}

impl WriteRegister for Power {
    fn get_raw_data(&self) -> u8 {
        return self.data;
    }
}

/// Magnetometer data register(s)
pub struct Data {
    data: [u8; 8],
}

impl Default for Data {
    fn default() -> Self {
        Self{ data: [0; 8] }
    }
}

impl Register for Data {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::DATA as u8;
    const SIZE: u8 = 8;
}

impl ReadRegister for Data {
    fn set_raw_data(&mut self, data: &[u8]) {
        for (i, val) in data.iter().enumerate() {
            self.data[i] = *val;
        }
    }
}

impl Data {
    pub fn get_x(&self) -> i16 {
        ((self.data[0] as i16 + ((self.data[1] as i16) << 8)) as i16) >> 3
    } 

    pub fn get_y(&self) -> i16 {
        ((self.data[2] as i16 + ((self.data[3] as i16) << 8)) as i16) >> 3
    } 

    pub fn get_z(&self) -> i16 {
        ((self.data[4] as i16 + ((self.data[5] as i16) << 8)) as i16) >> 1
    } 

    pub fn get_rhall(&self) -> i16 {
        ((self.data[6] as i16 + ((self.data[7] as i16) << 8)) as i16) >> 2
    }

}


///Magnetometer rate enums

/// Magnetometer control register
#[derive(Default)]
pub struct Control {
    data: u8,
}

impl Register for Control {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::CONTROL as u8;
    const SIZE: u8 = 1;
}

impl WriteRegister for Control {
    fn get_raw_data(&self) -> u8 {
        self.data
    }
}

//will leave with sensible defaults of 10Hz (default) and normal mode (all 0's)
// impl Control {
//     pub fn set_data_rate () {
        
//     }

//     pub fn set_op_mode() {

//     }
// }

///Magnetometer Trim Register 1
pub struct DigX1 {
    data: [u8; 2],
}

impl Default for DigX1 {
    fn default() -> Self {
        Self{ data: [0; 2] }
    }
}

impl Register for DigX1 {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::DIGX1 as u8;
    const SIZE: u8 = 2;
}

impl ReadRegister for DigX1 {
    fn set_raw_data(&mut self, data: &[u8]) {
        for (i, val) in data.iter().enumerate() {
            self.data[i] = *val;
        }
    }
}

impl DigX1 {
    pub fn get_x1(&self) -> u8 {
        self.data[0]
    }

    pub fn get_y1(&self) -> u8 {
        self.data[1]
    }
}


///Magnetometer Trim Register 2
pub struct DigZ4 {
    data: [u8; 4],
}

impl Default for DigZ4 {
    fn default() -> Self {
        Self{ data: [0; 4] }
    }
}

impl Register for DigZ4 {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::DIGZ4 as u8;
    const SIZE: u8 = 4;
}

impl ReadRegister for DigZ4 {
    fn set_raw_data(&mut self, data: &[u8]) {
        for (i, val) in data.iter().enumerate() {
            self.data[i] = *val;
        }
    }
}

impl DigZ4 {
        pub fn get_x2(&self) -> u8 {
            self.data[2]
        }
    
        pub fn get_y2(&self) -> u8 {
            self.data[3]
        }

        pub fn get_z4(&self) -> u16 {
            ((self.data[1] as u16) << 8) + (self.data[0] as u16)
        }
    
}


///Magnetometer Trim Register 3
pub struct DigZ2 {
    data: [u8; 10],
}

impl Default for DigZ2 {
    fn default() -> Self {
        Self{ data: [0; 10] }
    }
}

impl Register for DigZ2 {
    const DEVICE: u8 = MAG_ADDR;
    const REGISTER: u8 = BMM150::DIGZ2 as u8;
    const SIZE: u8 = 10;
}

impl ReadRegister for DigZ2 {
    fn set_raw_data(&mut self, data: &[u8]) {
        for (i, val) in data.iter().enumerate() {
            self.data[i] = *val;
        }
    }
}

impl DigZ2 {
    pub fn get_z1(&self) -> u16 {
        ((self.data[3] as u16) << 8) + (self.data[2] as u16)
    }

    pub fn get_z2(&self) -> u16 {
        ((self.data[1] as u16) << 8) + (self.data[0] as u16)
    }

    pub fn get_z3(&self) -> u16 {
        ((self.data[7] as u16) << 8) + (self.data[6] as u16)
    }

    pub fn get_xy1(&self) -> u8 {
        self.data[9]
    }

    pub fn get_xy2(&self) -> u8 {
        self.data[8]
    }

    pub fn get_xyz1(&self) -> u16 {
        (((self.data[5] & 0x7F) as u16) << 8) + (self.data[4] as u16)
    }

}
