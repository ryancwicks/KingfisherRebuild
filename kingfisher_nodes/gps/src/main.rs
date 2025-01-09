//! Program that publishes GPS data to DDS
use kingfisher_data_types::{dds_topics::{GpsData, GpsFix, GPS_TOPIC}, DEFAULT_ID};
use dust_dds::{
    domain::domain_participant_factory::DomainParticipantFactory,
    infrastructure::{qos::QosKind, status::NO_STATUS},
};
use gpsd_client::*;

const UPDATE_RATE: u64 = 1;

fn main() {
    //switch this to syslog later: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-to-the-unix-syslog
    env_logger::init();
    log::info!("Starting GPS publisher.");
    
    // Connecting to the gpsd socket server.
    let mut gps: GPS = match GPS::connect() {
        Ok(t) => t,
        Err(e) => {
            log::error!("Failed to connect to gpsd daemon: {e}");
            panic!("Failed to connect to gpsd_daemon");
        }
    };
    
    //Set up DDS topic and participant.
    let domain_id = kingfisher_data_types::DEFAULT_DOMAIN;
    let participant_factory = DomainParticipantFactory::get_instance();
    
    let participant = participant_factory
    .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
    .unwrap();
    
    let topic_gps = participant
    .create_topic::<GpsData>(GPS_TOPIC, "GpsData", QosKind::Default, None, NO_STATUS)
    .unwrap();
    
    let publisher = participant
    .create_publisher(QosKind::Default, None, NO_STATUS)
    .unwrap();
    
    let gps_writer = publisher
    .create_datawriter::<GpsData>(&topic_gps, QosKind::Default, None, NO_STATUS)
    .unwrap();
    
    loop {
        // Getting the data from the gps device.
        match gps.current_data() {
            Ok(data) => {

                log::info!("{:?}", data);
                let fix = match data.mode {
                    Fix::None => GpsFix::None,
                    Fix::Fix2D => GpsFix::Fix2D,
                    Fix::Fix3D => GpsFix::Fix3D
                };
                
                let gps_data = GpsData {
                    id: DEFAULT_ID.into(),
                    latitude: data.lat,
                    longitude: data.lon,
                    altitude: data.alt_hae,
                    velocity: data.speed,
                    direction: data.track,
                    fix: fix,
                    good_satellites: data.sats_valid 
                };
                
                match gps_writer.write(&gps_data, None) {
                    Ok(_) => {
                        log::info!("GPS data published.");
                    } Err(e) => {
                        log::error!("Failed to write GPS data to DDS: {:?}", e);
                    }
                };
            },
            Err(e) => {
                log::error!("Could not read GPS data from gpsd: {:?}", e);
            }
        }
        
        
        std::thread::sleep(std::time::Duration::from_secs(UPDATE_RATE));
    }
    
    //gps.close(); //unreachable
}