//! This module handles all communication, in and out, of the system through the serial port.

use atmega32u4_usb_serial::UsbSerial;
use arduino_hal::prelude::_embedded_hal_serial_Read;
use arduino_hal::prelude::_embedded_hal_serial_Write;

use postcard::{from_bytes, to_vec};
use heapless::Vec;

use kingfisher_data_types::microcontroller_types::{MicroControlMessages, MicroStatusMessages};

const MAX_VEC_LENGTH: usize = 100;

pub enum UsbCommsError {
    NothingRead,
    ParseError,
    BufferOverflow,
    DecodeError,
}

pub struct UsbComms {
    usb: UsbSerial,
    data_buffer: Vec<u8, 100>
}

impl UsbComms {

    /// Create a new USB device.
    pub fn new(usb: UsbSerial) -> Self {
        let data_buffer: Vec<u8, MAX_VEC_LENGTH> = Vec::new();

        UsbComms {
            usb,
            data_buffer
        }
    }

    /// Check for incoming data and return a packet if one is on the wire.
    pub fn poll(&mut self) -> Result<MicroControlMessages, UsbCommsError> {
         //read in serial data
         while self.usb.get_available() > 0 {
            let byte = nb::block!(self.usb.read()).unwrap();
            match self.data_buffer.push(byte) {
                Ok(_) => (),
                Err(_) => return Err(UsbCommsError::BufferOverflow)
            };
            if self.data_buffer.len() == MAX_VEC_LENGTH {
                self.data_buffer.clear();
                return Err(UsbCommsError::BufferOverflow);
            }
        }

        if self.data_buffer.len() == 0 {
            return Err(UsbCommsError::NothingRead);
        }

        let output = match from_bytes(&self.data_buffer) {
            Ok(val) => val,
            Err(_e) => {
                return Err(UsbCommsError::ParseError);
            }           
        };

        Ok(output)
    }

    /// Write a status packet to the topside.
    pub fn write_packet(&mut self, packet: &MicroStatusMessages) -> Result<(), UsbCommsError> {

        let output: Vec<u8, 100> = match  to_vec(packet) {
            Ok(val) => val,
            Err(_e) => {
                return Err(UsbCommsError::DecodeError);
            }
        };

        self.write_vec(&output);

        Ok(())
    }

    /// Write the contents of the vector over the USB serial port.
    fn write_vec(&mut self, data: &[u8]) {
        for c in data {
            nb::block!(self.usb.write(c.clone())).unwrap();
        }
    } 

    #[allow(unused)]
    /// Write the contents of the vector over the USB serial port.
    fn write_string(&mut self, data: &str) {
        ufmt::uwrite!(self.usb, "{}", data).unwrap();
    } 

}