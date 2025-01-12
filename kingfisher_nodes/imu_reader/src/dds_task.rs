//! This task handles DDS communications 
use kingfisher_data_types::imu_types::ImuMessages;
use tokio::sync::mpsc;

use kingfisher_data_types::{dds_topics::{ImuData, IMU_TOPIC}, DEFAULT_ID};
use dust_dds::{
    domain::domain_participant_factory::DomainParticipantFactory,
    infrastructure::{qos::QosKind, status::NO_STATUS},
};
use std::time::SystemTime;

pub struct DDSTask {
    from_serial: mpsc::Receiver<ImuMessages>
}

impl DDSTask {
    
    /// Create a new DDS Task
    pub fn new (from_serial: mpsc::Receiver<ImuMessages>) -> Self {
        DDSTask {
            from_serial
        }
    }
    
    /// Create a new DDS task.
    pub async fn run(&mut self) {
        //Set up DDS topic and participant.
        let domain_id = kingfisher_data_types::DEFAULT_DOMAIN;
        let participant_factory = DomainParticipantFactory::get_instance();
        
        let participant = participant_factory
        .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
        .unwrap();
        
        let topic_imu = participant
        .create_topic::<ImuData>(IMU_TOPIC, "ImuData", QosKind::Default, None, NO_STATUS)
        .unwrap();
        
        let publisher = participant
        .create_publisher(QosKind::Default, None, NO_STATUS)
        .unwrap();
        
        let imu_writer = publisher
        .create_datawriter::<ImuData>(&topic_imu, QosKind::Default, None, NO_STATUS)
        .unwrap();

        loop {
            match self.from_serial.recv().await {
                Some(val) => {
                    //log::info! ("Incoming {:?}", val);
                    match val {
                        ImuMessages::Imu(ax, ay, az, gx, gy, gz, mx, my, mz) => {
                            let current_time = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                                Ok(val) => val.as_secs_f64(),
                                Err(e) => {
                                    log::error!("Failed to unpack system time: {:?}", e);
                                    0.0
                                }
                            };
                            let imu_data = ImuData {
                                id: DEFAULT_ID.into(),
                                time: current_time, 
                                accelerometer: vec!(ax, ay, az),
                                gyroscope: vec!(gx, gy, gz),
                                magnetometer: vec!(mx, my, mz)
                            };
                            log::info! ("{:?}", imu_data);
                            match imu_writer.write(&imu_data, None) {
                                Ok(_) => {
                                    //log::info!("IMU data published.");
                                } Err(e) => {
                                    log::error!("Failed to write IMU data to DDS: {:?}", e);
                                }
                            };
                        }
                    };
                },
                None => ()
            }
        }
        
        
    }
    
}