//! Handler for the serial port.
use tokio::sync::mpsc;
use futures::stream::{StreamExt, SplitStream};
//use futures::sink::SinkExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use bytes::{BytesMut, Buf};
use postcard::from_bytes;
use tokio::io::{Error, ErrorKind};
use kingfisher_data_types::imu_types::ImuMessages;

pub struct SerialTask {
    //serial_sink: SplitSink<Framed<tokio_serial::SerialStream, MicroPacketCodec>, ImuMessages>,
    serial_source: SplitStream<Framed<tokio_serial::SerialStream, ImuPacketCodec>>,
    send_to_dds: mpsc::Sender<ImuMessages>,
}

impl SerialTask {
    /// Create a new serial task
    pub fn new(serial_port: tokio_serial::SerialStream, send_to_dds: mpsc::Sender<ImuMessages>) -> Self {
        let (_serial_sink, serial_source) = Framed::new(serial_port, ImuPacketCodec).split();
        SerialTask {
            //serial_sink,
            serial_source,
            send_to_dds,
        }
    }
    
    /// The main task that should be spawned in another thread.
    pub async fn run(&mut self) {
        loop {
            let val = self.serial_source.next().await; 
            
            //log::info!("Received serial data in serial task.");
            if let Some(packet) = val {
                match packet {
                    Ok(packet) => {
                        //log::info!("Sending parsed packet to DDS");
                        match self.send_to_dds.send(packet).await {
                            Ok(_) => (),
                            Err(e) => {
                                log::error!("Failed to send message to DDS task: {}", e);
                            }
                        };
                    },
                    Err(e) => {
                        log::error!("Unable to unpack packer: {}", e);
                    }
                }
                
            }
            
            
        }
    }
}


struct ImuPacketCodec;

impl Decoder for ImuPacketCodec {
    type Item = ImuMessages;
    type Error = tokio::io::Error;
    
    /// Take bytes, turn it into a MicroStatusMessage
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        //log::info!("data: {:?}", &src);
        match from_bytes(src) {
            Ok(val) => {
                src.advance(std::mem::size_of::<ImuMessages>());
                Ok(Some(val))
            }
            Err(e) => {
                match e {
                    postcard::Error::DeserializeUnexpectedEnd => Ok(None),
                    postcard::Error::SerializeBufferFull => {
                        log::error!("Unexpected buffer full error: {:?}", e);
                        src.clear();
                        Ok(None)
                    },
                    postcard::Error::SerdeDeCustom => {
                        src.clear();
                        Ok(None)
                    },
                    _ => {
                        //clear the buffer.
                        src.clear();
                        Err(Error::new(ErrorKind::Other, format!("Decoding Error: {:?}", e)))
                    }
                }
            }
        }
    }
}

/// Take a Microcontroller Control packet and turn it into bytes
impl Encoder<ImuMessages> for ImuPacketCodec {
    type Error = tokio::io::Error;
    
    fn encode(&mut self, item: ImuMessages, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match postcard::to_vec::<ImuMessages, 64>(&item) {
            Ok(val) => {
                dst.clear();
                dst.extend_from_slice(val.as_slice());
                Ok(())
            },
            Err(e) => {
                Err(Error::new(ErrorKind::Other, format!("Encoding Error: {:?}", e)))
            }
        }
    }
}