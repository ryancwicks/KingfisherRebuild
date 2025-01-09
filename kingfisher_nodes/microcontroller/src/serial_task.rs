//! Handler for the serial port.
use tokio::sync::mpsc;
use futures::stream::{StreamExt, SplitSink, SplitStream};
use futures::sink::SinkExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use bytes::BytesMut;
use postcard::from_bytes;
use tokio::io::{Error, ErrorKind};
use kingfisher_data_types::microcontroller_types::{MicroControlMessages, MicroStatusMessages};

pub struct SerialTask {
    serial_sink: SplitSink<Framed<tokio_serial::SerialStream, MicroPacketCodec>, MicroControlMessages>,
    serial_source: SplitStream<Framed<tokio_serial::SerialStream, MicroPacketCodec>>,
    read_into_serial: mpsc::Receiver<MicroControlMessages>,
    send_to_dds: mpsc::Sender<MicroStatusMessages>,
}

impl SerialTask {
    /// Create a new serial task
    pub fn new(serial_port: tokio_serial::SerialStream, read_into_serial: mpsc::Receiver<MicroControlMessages>, send_to_dds: mpsc::Sender<MicroStatusMessages>) -> Self {
        let (serial_sink, serial_source) = Framed::new(serial_port, MicroPacketCodec).split();
        SerialTask {
            serial_sink,
            serial_source,
            read_into_serial,
            send_to_dds,
        }
    }

    /// The main task that should be spawned in another thread.
    pub async fn run(&mut self) {
        loop {
            tokio::select! {
                val = self.read_into_serial.recv() => {
                    log::info!("Received message from DDS task.");
                    if let Some(packet) = val {
                        match self.serial_sink.send(packet).await {
                            Ok(_) => (),
                            Err(e) => {
                                log::error!("Failed to send message to serial port: {:?}", e);
                            }
                        };
                    }
                }
                val = self.serial_source.next() => {
                    log::info!("Received serial data in serial task.");
                    if let Some(packet) = val {
                        match packet {
                            Ok(packet) => {
                                log::info!("Sending parsed packet to DDS");
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
    }
}


struct MicroPacketCodec;

impl Decoder for MicroPacketCodec {
    type Item = MicroStatusMessages;
    type Error = tokio::io::Error;

    /// Take bytes, turn it into a MicroStatusMessage
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match from_bytes(src) {
            Ok(val) => Ok(Some(val)),
            Err(e) => {
                Err(Error::new(ErrorKind::Other, format!("Decoding Error: {:?}", e)))
            }
        }
    }
}

/// Take a Microcontroller Control packet and turn it into bytes
impl Encoder<MicroControlMessages> for MicroPacketCodec {
    type Error = tokio::io::Error;

    fn encode(&mut self, item: MicroControlMessages, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match postcard::to_vec::<MicroControlMessages, 250>(&item) {
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