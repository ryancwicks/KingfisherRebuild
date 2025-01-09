use embassy_stm32::i2c::I2c;
use embassy_stm32::{mode, i2c::Error};

use crate::click_driver::registers::{
    EnvId, EnvCtrlMeas, EnvOversample, EnvCtrlHumid, EnvCtrlGas0,
    AccId, 
    GyroId,
    MagId, MagPower};
use crate::click_driver::types::{Register, ReadRegister, WriteRegister};

use defmt::error;

/// Mikro click driver interface
pub struct ClickDriver<'a> {
    i2c: I2c<'a, mode::Async>
}

impl <'a> ClickDriver <'a>{
    
    /// Create a new Click Driver instance
    pub fn new(i2c: I2c<'a, mode::Async>) -> Self {
        ClickDriver {
            i2c
        }
    }
    
    //----------------------------------------------------------------------------
    // Magnetometer commands
    
    /// Check if you can communicate with the BMM150 Magnetic sensor
    pub async fn check_magnetometer_exists(&mut self) -> Result<bool, Error>  {
        let mut register = MagId::default();
        self.read_register(&mut register).await?;
        Ok(register.is_id_correct())
    }
    
    /// Send the power on command to the gyroscope
    pub async fn power_on_magnetometer(&mut self) -> Result<(), Error> {
        let register = MagPower::default();
        self.write_register(&register).await?;
        Ok(())
    }
    
    
    //----------------------------------------------------------------------------
    // Accelerometer commands
    
    /// Check if you can communicate with the BMI088 Accelerometer exists
    pub async fn check_accelerometer_exists(&mut self) -> Result<bool, Error>  {
        let mut register = AccId::default();
        self.read_register(&mut register).await?;
        Ok(register.is_id_correct())
    }
    
    //----------------------------------------------------------------------------
    // Gyroscope commands
    
    /// Check if you can communicate with the BMI088 gyroscope exists
    pub async fn check_gyroscope_exists(&mut self) -> Result<bool, Error>  {
        let mut register = GyroId::default();
        self.read_register(&mut register).await?;
        Ok(register.is_id_correct())
    }
    
    
    //----------------------------------------------------------------------------
    // Environmental sensor commands
    /// Check if you can communicate with the BME680 environmental sensor
    pub async fn check_environmental_exists(&mut self) -> Result<bool, Error>  {
        let mut register = EnvId::default();
        self.read_register(&mut register).await?;
        Ok(register.is_id_correct())
    }
    /// Setup the environmental sensor
    pub async fn setup_environmental(&mut self) -> Result<(), Error> {
        let mut ctrl_meas = EnvCtrlMeas::default();
        self.read_register(&mut ctrl_meas).await?;
        ctrl_meas.set_temperature_oversample(EnvOversample::OS2);
        ctrl_meas.set_pressure_oversample(EnvOversample::OS8);
        ctrl_meas.start_sample(false);

        let mut ctrl_humidity = EnvCtrlHumid::default();
        ctrl_humidity.set_humidity_oversample(EnvOversample::OS1);

        let ctrl_gas_0 = EnvCtrlGas0::default();

        self.write_register(&ctrl_gas_0).await?;
        self.write_register(&ctrl_humidity).await?;
        self.write_register(&ctrl_meas).await?;

        Ok(())
    }
    
    /// Trigger the environmental sensor measurement
    pub async fn trigger_environmental(&mut self) -> Result<(), Error> {
        let mut ctrl_meas = EnvCtrlMeas::default();
        self.read_register(&mut ctrl_meas).await?;
        ctrl_meas.start_sample(true);
        self.write_register(&ctrl_meas).await?;

        Ok(())
    }

    //---------------------------------------------------------------------------
    // Utility commands
    
    /// Utility function for reading data from a set of registers
    async fn read_register<T: Register + ReadRegister>(&mut self, reg: &mut T) -> Result<(), Error> {
        match T::SIZE {
            1 => {
                let data = self.read_byte_register(T::DEVICE, T::REGISTER).await?;
                reg.set_raw_data(&[data]);
            },
            _ => error!("Data read size not implemented.")
        };
        Ok(())
    }

    //Utility function for writing data to a single register.
    async fn write_register<T: Register + WriteRegister>(&mut self, reg: &T) -> Result<(), Error> {
        self.write_byte_register(T::DEVICE, T::REGISTER, reg.get_raw_data()).await
    }
    /// Utility function for reading a single byte.
    async fn read_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG) -> Result<u8, Error> {
        let mut buf: [u8; 1] = [0; 1];
        match self.i2c.write_read(device_address, &[register.into()], &mut buf).await {
            Ok(_) => (),
            Err(e) => return Err(e)
        };
        
        Ok(buf[0])
    }
    
    /// Utility function for writing a single byte
    async fn write_byte_register<REG: Into<u8>>(&mut self, device_address: u8, register: REG, data: u8) -> Result<(), Error> {
        let buf: [u8; 2] = [register.into(), data];
        self.i2c.write(device_address, &buf).await
    }
}