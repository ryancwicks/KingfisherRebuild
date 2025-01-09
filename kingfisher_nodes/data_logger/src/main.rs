//! Utility for saving data from multiple DDS topics
use config::Config;
use clap::Parser;
use kingfisher_data_types::dds_topics::{
    SystemStatusCpu, SYSTEM_STATUS_CPU_TOPIC, SYSTEM_STATUS_DISK_TOPIC, SYSTEM_STATUS_MEMORY_TOPIC, 
    SYSTEM_STATUS_NETWORK_TOPIC, GPS_TOPIC, SystemStatusMemory,
    SystemStatusDisk, SystemStatusNetwork, GpsData
};

use dust_dds::{
    dds_async::domain_participant_factory::DomainParticipantFactoryAsync, 
    infrastructure::{error::DdsError, qos::QosKind, status::NO_STATUS}, 
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE}, 
};



/// Parser for command line parameters
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandLineParameters {
    /// Path to the config file
    #[arg(short, long)]
    config_file: Option<String>,
}

// struct Listener<T> {
//     listener: Sender<T>
// }

// impl<T: 'static> DataReaderListenerAsync<'_> for Listener<T>{
//     type Foo = T;

//     async fn on_data_available(&mut self, the_reader: DataReaderAsync<T>) {
//         if let Ok(samples) =
//             the_reader.take(1, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await
//         {
//             let sample = samples[0].data().unwrap();
//             log::info!("Read sample: {:?}", sample);
//         }
//     }
//     async fn on_subscription_matched(
//         &mut self,
//         _the_reader: DataReaderAsync<T>,
//         status: dust_dds::infrastructure::status::SubscriptionMatchedStatus,
//     ) {
//         if status.current_count == 0 {
//             log::info!("Connected.");
//         }
//     }

// }


// async fn run_reader_task<T: DdsDeserialize<'static> + Debug + 'static>(reader: &'static DataReaderAsync<T>) {
    
//     loop {
//         let data = match reader.read(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
//             Ok(val) => &val,
//             Err(e) => match e {
//                 DdsError::NoData => {
//                     tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; 
//                     continue;
//                 },
//                 _ => {
//                     log::error!("Unexpected error: {:?}", e);
//                     continue;
//                 }
//             }
//         };
        
//         for sample in data {        
//             log::info!("{:?}", sample.data().unwrap());
//         }
        
        
//     }
    
// }

#[tokio::main]
async fn main() {
    
    //switch this to syslog later: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-to-the-unix-syslog
    env_logger::init();
    log::info!("Starting data logger.");
    
    let cli = CommandLineParameters::parse();
    let config_file = match cli.config_file {
        Some(val) => val,
        None => String::from("./data_logger.toml")
    };
    
    let settings = Config::builder()
    .add_source(config::File::with_name(&config_file))
    .build()
    .unwrap();
    
    let save_root: String = settings.get_string("save_root").unwrap_or("/data".to_string());
    let auto_save: bool = settings.get_bool("auto_save").unwrap_or(false);
    
    //Setting up DDS
    let domain_id = kingfisher_data_types::DEFAULT_DOMAIN;
    let participant_factory = DomainParticipantFactoryAsync::new();
    
    let participant = participant_factory
    .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
    .await
    .unwrap();
    
    //Topics
    //System Status topics
    let topic_memory = participant
    .create_topic::<SystemStatusMemory>(SYSTEM_STATUS_MEMORY_TOPIC, "SystemStatusMemory", QosKind::Default, None, NO_STATUS)
    .await
    .unwrap();
    let topic_disk = participant
    .create_topic::<SystemStatusDisk>(SYSTEM_STATUS_DISK_TOPIC, "SystemStatusDisk", QosKind::Default, None, NO_STATUS)
    .await
    .unwrap();
    let topic_network = participant.create_topic::<SystemStatusNetwork>(SYSTEM_STATUS_NETWORK_TOPIC, "SystemStatusNetwork", QosKind::Default, None, NO_STATUS)
    .await
    .unwrap();
    let topic_cpu = participant.create_topic::<SystemStatusCpu>(SYSTEM_STATUS_CPU_TOPIC, "SystemStatusCpu", QosKind::Default, None, NO_STATUS)
    .await
    .unwrap();
    
    //GPS topics
    let topic_gps = participant.create_topic::<GpsData>(GPS_TOPIC, "GpsData", QosKind::Default, None, NO_STATUS).await.unwrap();
    
    let subscriber = participant
    .create_subscriber(QosKind::Default, None, NO_STATUS)
    .await
    .unwrap();
    
    let reader_memory = subscriber.create_datareader::<SystemStatusMemory>(&topic_memory, QosKind::Default, None, NO_STATUS).await.unwrap();
    let reader_disk = subscriber.create_datareader::<SystemStatusDisk>(&topic_disk, QosKind::Default, None, NO_STATUS).await.unwrap();
    let reader_network = subscriber.create_datareader::<SystemStatusNetwork>(&topic_network, QosKind::Default, None, NO_STATUS).await.unwrap();
    let reader_cpu = subscriber.create_datareader::<SystemStatusCpu>(&topic_cpu, QosKind::Default, None, NO_STATUS).await.unwrap();
    let reader_gps = subscriber.create_datareader::<GpsData>(&topic_gps, QosKind::Default, None, NO_STATUS).await.unwrap();
    
    tokio::spawn( async move {
        loop {
            let data = match reader_memory.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
                Ok(val) => val,
                Err(e) => match e {
                    DdsError::NoData => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; 
                        Vec::new()
                    },
                    _ => {
                        log::error!("Unexpected error: {:?}", e);
                        Vec::new()
                    }
                }
            };
            
            for sample in data {
                log::info!("{:?}", sample.data().unwrap());
            }
            
        }
    });
    

    tokio::spawn( async move {
        loop {
            let data = match reader_cpu.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
                Ok(val) => val,
                Err(e) => match e {
                    DdsError::NoData => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; 
                        Vec::new()
                    },
                    _ => {
                        log::error!("Unexpected error: {:?}", e);
                        Vec::new()
                    }
                }
            };
            
            for sample in data {
                log::info!("{:?}", sample.data().unwrap());
            }
            
        }
    });

    tokio::spawn( async move {
        loop {
            let data = match reader_disk.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
                Ok(val) => val,
                Err(e) => match e {
                    DdsError::NoData => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; 
                        Vec::new()
                    },
                    _ => {
                        log::error!("Unexpected error: {:?}", e);
                        Vec::new()
                    }
                }
            };
            
            for sample in data {
                log::info!("{:?}", sample.data().unwrap());
            }
            
        }
    });
    
    tokio::spawn( async move {loop {
        let data = match reader_network.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
            Ok(val) => val,
            Err(e) => match e {
                DdsError::NoData => {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; 
                    Vec::new()
                },
                _ => {
                    log::error!("Unexpected error: {:?}", e);
                    Vec::new()
                }
            }
        };
        
        for sample in data {
            log::info!("{:?}", sample.data().unwrap());
        }
        
    }
    });
    
    tokio::spawn( async move {
        loop {
            let data = match reader_gps.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
                Ok(val) => val,
                Err(e) => match e {
                    DdsError::NoData => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await; 
                        Vec::new()
                    },
                    _ => {
                        log::error!("Unexpected error: {:?}", e);
                        Vec::new()
                    }
                }
            };
            
            for sample in data {
                log::info!("{:?}", sample.data().unwrap());
            }
            
        }
    });
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
