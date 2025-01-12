use embassy_stm32::i2c::I2c;
use embassy_stm32::{mode, i2c::Error};
use embassy_time::Timer;

mod mag {
    pub use crate::click_driver::bmm150_registers::{Id, Power, Control, Data, DigX1, DigZ4, DigZ2};
}
mod gyro {
    pub use crate::click_driver::bmi088_registers::{GyroId, GyroData};
}
mod accel {
    pub use crate::click_driver::bmi088_registers::{AccId, AccPowerCtrl, AccPowerConf, AccData};
}
use crate::click_driver::types::{Register, ReadRegister, WriteRegister};

use crate::imu_types::ImuMessages;

use embassy_sync::{
    blocking_mutex::raw::ThreadModeRawMutex,
    channel::Sender,
};

use defmt::{info, error};

/// Mikro click driver interface
pub struct ClickDriver<'a> {
    i2c: I2c<'a, mode::Async>,

    //mag trim
    mx1: u8,
    my1: u8,
    mx2: u8,
    my2: u8,
    mz4: u16,
    mz1: u16,
    mz2: u16,
    mz3: u16,
    mxy1: u8,
    mxy2: u8,
    mxyz1: u16,
}

impl <'a> ClickDriver <'a>{
    
    /// Create a new Click Driver instance
    pub fn new(i2c: I2c<'a, mode::Async>) -> Self {
        ClickDriver {
            i2c,
            mx1: 0,
            my1: 0,
            mx2: 0,
            my2: 0,
            mz4: 0,
            mz1: 0,
            mz2: 0,
            mz3: 0,
            mxy1: 0,
            mxy2: 0,
            mxyz1: 0
        }
    }
    
    //----------------------------------------------------------------------------
    // Magnetometer commands
    
    /// Check if you can communicate with the BMM150 Magnetic sensor
    async fn check_magnetometer_exists(&mut self) -> Result<bool, Error>  {
        let mut register = mag::Id::default();
        self.read_register(&mut register).await?;
        Ok(register.is_id_correct())
    }
    
    /// Send the power on command to the gyroscope
    async fn power_on_magnetometer(&mut self) -> Result<(), Error> {
        let register = mag::Power::default();
        self.write_register(&register).await?;
        
        let control = mag::Control::default();
        self.write_register(&control).await?;
        
        Ok(())
    }
    
    /// Read in the mag trim values
    async fn read_mag_trim(&mut self) -> Result<(), Error> {
        let mut x1 = mag::DigX1::default();
        self.read_register(&mut x1).await?;
        let mut z4 = mag::DigZ4::default();
        self.read_register(&mut z4).await?;
        let mut z2 = mag::DigZ2::default();
        self.read_register(&mut z2).await?;
        
        self.mx1 = x1.get_x1();
        self.my1 = x1.get_y1();
        self.mx2 = z4.get_x2();
        self.my2 = z4.get_y2();
        self.mz4 = z4.get_z4();
        self.mz1 = z2.get_z1();
        self.mz2 = z2.get_z2();
        self.mz3 = z2.get_z3();
        self.mxy1 = z2.get_xy1();
        self.mxy2 = z2.get_xy2();
        self.mxyz1 = z2.get_xyz1();

        Ok(())
    }

    /// Read mag data
    async fn read_mag_data(&mut self) -> Result<(f32, f32, f32), Error> {
        let mut data = mag::Data::default();
        self.read_register(&mut data).await?;

        let raw_x = data.get_x() as f32;
        let raw_y = data.get_y() as f32;
        let raw_z = data.get_z() as f32;
        let rhall = data.get_rhall() as f32;

        let mut ret_x;
        {
            let process_comp_x0 = (self.mxyz1 as f32) * 16384.0 / (rhall);
            ret_x = process_comp_x0 - 16384.0;
            let process_comp_x1 = (self.mxy2 as f32) * ret_x*ret_x/268435456.0;
            let process_comp_x2 = process_comp_x1 + ret_x*(self.mxy1 as f32) / 16384.0;
            let process_comp_x3 = self.mx2 as f32 + 160.0;
            let process_comp_x4 = (raw_x as f32) * (process_comp_x2 + 256.0) * process_comp_x3;
            ret_x = (process_comp_x4/8192.0 + (self.mx1 as f32) * 8.0) / 16.0;
        }

        let mut ret_y;
        {
            let process_comp_y0 = (self.mxyz1 as f32) * 16384.0 / (rhall);
            ret_y = process_comp_y0 - 16384.0;
            let process_comp_y1 = (self.mxy2 as f32) * ret_y*ret_y/268435456.0;
            let process_comp_y2 = process_comp_y1 + ret_x*(self.mxy1 as f32) / 16384.0;
            let process_comp_y3 = self.my2 as f32 + 160.0;
            let process_comp_y4 = (raw_y as f32) * (process_comp_y2 + 256.0) * process_comp_y3;
            ret_y = (process_comp_y4/8192.0 + (self.my1 as f32) * 8.0) / 16.0;
        }

        let ret_z;
        {
            let process_comp_z0 = raw_z - self.mz4 as f32;
            let process_comp_z1 = rhall - self.mxyz1 as f32;
            let process_comp_z2 = (self.mz3 as f32) * process_comp_z1;
            let process_comp_z3 = (self.mz1 as f32) * rhall/32768.0;
            let process_comp_z4 = self.mz2 as f32 + process_comp_z3;
            let process_comp_z5 = process_comp_z0 * 131072.0 - process_comp_z2;
            ret_z = process_comp_z5 / process_comp_z4 / 4.0 / 16.0;
        }

        Ok((ret_x, ret_y, ret_z))
    }
    
    //----------------------------------------------------------------------------
    // Accelerometer commands
    
    /// Check if you can communicate with the BMI088 Accelerometer exists
    async fn check_accelerometer_exists(&mut self) -> Result<bool, Error>  {
        let mut register = accel::AccId::default();
        self.read_register(&mut register).await?;
        Ok(register.is_id_correct())
    }

    /// enable to accelerometer in active mode
    async fn enable_accelerometer(&mut self) -> Result<(), Error> {
        let power_ctrl = accel::AccPowerCtrl::default();
        let power_conf = accel::AccPowerConf::default();

        self.write_register(&power_conf).await?;
        self.write_register(&power_ctrl).await?;

        Ok(())
    }

    async fn read_accel_data(&mut self) -> Result<(f32, f32, f32), Error> {
        let mut data = accel::AccData::default();
        self.read_register(&mut data).await?;

        Ok(data.get_data())
    }
    
    //----------------------------------------------------------------------------
    // Gyroscope commands
    
    /// Check if you can communicate with the BMI088 gyroscope exists
    async fn check_gyroscope_exists(&mut self) -> Result<bool, Error>  {
        let mut register = gyro::GyroId::default();
        self.read_register(&mut register).await?;
        Ok(register.is_id_correct())
    }
    
    /// Read the gyro data
    async fn read_gyro_data(&mut self) -> Result<(f32, f32, f32), Error> {
        let mut data = gyro::GyroData::default();
        self.read_register(&mut data).await?;
        Ok(data.read_gyro_data())
    }
    
    // //----------------------------------------------------------------------------
    // // environmental sensor commands
    // /// Check if you can communicate with the BME680 environmental sensor
    // async fn check_environmental_exists(&mut self) -> Result<bool, Error>  {
    //     let mut register = enviro::Id::default();
    //     self.read_register(&mut register).await?;
    //     Ok(register.is_id_correct())
    // }
    
    // /// Setup the environmental sensor
    // async fn setup_environmental(&mut self) -> Result<(), Error> {
    //     //read calibration registers
    //     let mut coeffs = enviro::Coeffs::default();
    //     self.read_register(&mut coeffs).await?;
    
    //     self.env_t1 = coeffs.t1();
    //     self.env_t2 = coeffs.t2();
    //     self.env_t3 = coeffs.t3(); 
    
    //     let mut ctrl_meas = enviro::CtrlMeas::default();
    //     self.read_register(&mut ctrl_meas).await?;
    //     ctrl_meas.set_temperature_oversample(enviro::Oversample::OS2);
    //     ctrl_meas.set_pressure_oversample(enviro::Oversample::OS8);
    //     ctrl_meas.start_sample(false);
    
    //     let mut ctrl_humidity = enviro::CtrlHumid::default();
    //     ctrl_humidity.set_humidity_oversample(enviro::Oversample::OS1);
    
    //     let ctrl_gas_0 = enviro::CtrlGas0::default();
    
    //     let mut config = enviro::Config::default();
    //     config.set_filter(enviro::Filter::F3);
    
    //     self.write_register(&ctrl_gas_0).await?;
    //     self.write_register(&ctrl_humidity).await?;
    //     self.write_register(&ctrl_meas).await?;
    //     self.write_register(&config).await?;
    
    //     Ok(())
    // }
    
    // /// Trigger the environmental sensor measurement
    // async fn trigger_environmental(&mut self) -> Result<(), Error> {
    //     let mut ctrl_meas = enviro::CtrlMeas::default();
    //     self.read_register(&mut ctrl_meas).await?;
    //     ctrl_meas.start_sample(true);
    //     self.write_register(&ctrl_meas).await?;
    
    //     Ok(())
    // }
    
    // /// Read the temperature
    // async fn read_env_temp(&mut self) -> Result<f32, Error> {
    //     let mut temp_reg = enviro::Temperature::default();
    //     self.read_register(&mut temp_reg).await?;
    //     let raw_temp = temp_reg.get_raw_temperature();
    
    //     /* calculate var1 data */
    //     let var1 = ((raw_temp as f32 / 16384.0) - (self.env_t1 / 1024.0)) * self.env_t2;
    
    //     /* calculate var2 data */
    //     let var2 =
    //         (((raw_temp as f32 / 131072.0) - (self.env_t1/ 8192.0)) *
    //         ((raw_temp as f32 / 131072.0) - (self.env_t1/ 8192.0))) * (self.env_t3 * 16.0);
    
    //     /* compensated temperature data*/
    //     let temperature = ((var1 + var2)) / 5120.0;
    
    //     Ok(temperature)
    // }
    
    //---------------------------------------------------------------------------
    // Utility commands
    
    /// Utility function for reading data from a set of registers
    async fn read_register<T: Register + ReadRegister>(&mut self, reg: &mut T) -> Result<(), Error> {
        match T::SIZE {
            1 => {
                let data = self.read_byte_register(T::DEVICE, T::REGISTER).await?;
                reg.set_raw_data(&data);
            },
            2 => {
                let data = self.read_2_byte_register(T::DEVICE, T::REGISTER).await?;
                reg.set_raw_data(&data);
            },
            3 => {
                let data = self.read_3_byte_register(T::DEVICE, T::REGISTER).await?;
                reg.set_raw_data(&data);
            },
            4 => {
                let data = self.read_4_byte_register(T::DEVICE, T::REGISTER).await?;
                reg.set_raw_data(&data);
            },
            6 => {
                let data = self.read_6_byte_register(T::DEVICE, T::REGISTER).await?;
                reg.set_raw_data(&data);
            },
            8 => {
                let data = self.read_8_byte_register(T::DEVICE, T::REGISTER).await?;
                reg.set_raw_data(&data);
            },
            10 => {
                let data = self.read_10_byte_register(T::DEVICE, T::REGISTER).await?;
                reg.set_raw_data(&data);
            },
            _ => error!("Data read size not implemented.")
        };
        Timer::after_millis(5).await;
        Ok(())
    }
    
    //Utility function for writing data to a single register.
    async fn write_register<T: Register + WriteRegister>(&mut self, reg: &T) -> Result<(), Error> {
        let result = self.write_byte_register(T::DEVICE, T::REGISTER, reg.get_raw_data()).await;
        Timer::after_millis(5).await;
        result
    }
    /// Utility function for reading a single byte.
    async fn read_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG) -> Result<[u8; 1], Error> {
        let mut buf: [u8; 1] = [0; 1];
        match self.i2c.write_read(device_address, &[register.into()], &mut buf).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        Ok(buf)
    }
    
    /// Utility function for reading a two bytes.
    async fn read_2_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG) -> Result<[u8; 2], Error> {
        let mut buf: [u8; 2] = [0; 2];
        match self.i2c.write_read(device_address, &[register.into()], &mut buf).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        Ok(buf)
    }
    
    /// Utility function for reading three bytes.
    async fn read_3_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG) -> Result<[u8; 3], Error> {
        let mut buf: [u8; 3] = [0; 3];
        match self.i2c.write_read(device_address, &[register.into()], &mut buf).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        Ok(buf)
    }
    
    /// Utility function for reading four bytes.
    async fn read_4_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG) -> Result<[u8; 4], Error> {
        let mut buf: [u8; 4] = [0; 4];
        match self.i2c.write_read(device_address, &[register.into()], &mut buf).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        Ok(buf)
    }

   /// Utility function for reading six bytes.
   async fn read_6_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG) -> Result<[u8; 6], Error> {
    let mut buf: [u8; 6] = [0; 6];
    match self.i2c.write_read(device_address, &[register.into()], &mut buf).await {
        Ok(_) => (),
        Err(e) => return Err(e)
    };
    
    Ok(buf)
}


    /// Utility function for reading eight bytes.
    async fn read_8_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG) -> Result<[u8; 8], Error> {
        let mut buf: [u8; 8] = [0; 8];
        match self.i2c.write_read(device_address, &[register.into()], &mut buf).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        Ok(buf)
    }

    /// Utility function for reading ten bytes.
    async fn read_10_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG) -> Result<[u8; 10], Error> {
        let mut buf: [u8; 10] = [0; 10];
        match self.i2c.write_read(device_address, &[register.into()], &mut buf).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        Ok(buf)
    }
    
    /// Utility function for writing a single byte
    async fn write_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG, data: u8) -> Result<(), Error> {
        let buf: [u8; 2] = [register.into(), data];
        self.i2c.write(device_address, &buf).await
    }
    
}


/// Run the click interface
#[embassy_executor::task]
pub async fn run_click_driver(mut click_driver: ClickDriver<'static>, event_channel: Sender<'static, ThreadModeRawMutex, ImuMessages, 64>) {
    match click_driver.power_on_magnetometer().await {
        Ok(_) => (),
        Err(e) => error!("Failed to power on Magnetometer: {:?}", e)
    };
    match click_driver.check_accelerometer_exists().await {
        Ok(val) => {
            if !val {
                error!("Accelerometer not detected.");
            }
        },
        Err(e) => error!("Failed to communicate with i2c bus {:?}", e)
    };
    match click_driver.check_gyroscope_exists().await {
        Ok(val) => {
            if !val {
                error!("Gyroscope not detected.");
            }
        },
        Err(e) => error!("Failed to communicate with i2c bus {:?}", e)
    };
    match click_driver.check_magnetometer_exists().await {
        Ok(val) => {
            if !val {
                error!("Magnetometer not detected.");
            }
        },
        Err(e) => error!("Failed to communicate with i2c bus {:?}", e)
    }
    
    //Setup the sensors
    click_driver.read_mag_trim().await.unwrap_or_else(|e| {error!("Failed to read mag trim values: {}", e)});
    click_driver.enable_accelerometer().await.unwrap_or_else(|e| {error!("Failed to enable accelerometer: {}", e)});
    loop {
        
        Timer::after_millis(100).await;

        let (mag_x, mag_y, mag_z) = click_driver.read_mag_data().await.unwrap_or_else(|e| {
            error!("Failed to read mag data: {}", e);
            (0.0, 0.0, 0.0)
        });
        let (acc_x, acc_y, acc_z) = click_driver.read_accel_data().await.unwrap_or_else(|e| {
            error!("Failed to read accel data: {}", e);
            (0.0, 0.0, 0.0)
        });
        let (gyro_x, gyro_y, gyro_z) = click_driver.read_gyro_data().await.unwrap_or_else(|e| {
            error!("Failed to read gyro data: {}", e);
            (0.0, 0.0, 0.0)
        });

        info!("mag: ({}, {}, {}), acc: ({}, {}, {}), gyro: ({}, {}, {})", mag_x, mag_y, mag_z, acc_x, acc_y, acc_z, gyro_x, gyro_y, gyro_z);

        let msg = ImuMessages::Imu(acc_x, acc_y, acc_z, gyro_x, gyro_y, gyro_z, mag_x, mag_y, mag_z);
        event_channel.send(msg).await;
    }
    
} 
