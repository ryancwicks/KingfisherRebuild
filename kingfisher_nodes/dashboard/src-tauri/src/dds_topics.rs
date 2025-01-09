use dust_dds::{
    dds_async::{domain_participant_factory::DomainParticipantFactoryAsync, data_reader::DataReaderAsync}, 
    infrastructure::{error::DdsError, qos::QosKind, status::NO_STATUS}, 
    subscription::sample_info::{ANY_INSTANCE_STATE, ANY_SAMPLE_STATE, ANY_VIEW_STATE}, 
};
use kingfisher_data_types::dds_topics::{
    GPS_TOPIC, SYSTEM_STATUS_CPU_TOPIC, SYSTEM_STATUS_DISK_TOPIC, 
    SYSTEM_STATUS_MEMORY_TOPIC, SYSTEM_STATUS_NETWORK_TOPIC,
    SystemStatusMemory, SystemStatusCpu, SystemStatusNetwork, 
    SystemStatusDisk, GpsData
};

use rerun;

pub fn setup_dds_topics() {
    

    let rrd = rerun::RecordingStreamBuilder::new("kingfisher")
    .serve_web("127.0.0.1", Default::default(),
    re_ws_comms::RerunServerPort(4321),
    rerun::MemoryLimit::from_fraction_of_total(0.25),
    false).unwrap();
    
    tauri::async_runtime::spawn(async move {
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
        .await.unwrap();
        let topic_cpu = participant
        .create_topic::<SystemStatusCpu>(SYSTEM_STATUS_CPU_TOPIC, "SystemStatusCpu", QosKind::Default, None, NO_STATUS)
        .await.unwrap();
        let topic_network = participant
        .create_topic::<SystemStatusCpu>(SYSTEM_STATUS_NETWORK_TOPIC, "SystemStatusNetwork", QosKind::Default, None, NO_STATUS)
        .await.unwrap();
        let topic_disk = participant
        .create_topic::<SystemStatusDisk>(SYSTEM_STATUS_DISK_TOPIC, "SystemStatusDisk", QosKind::Default, None, NO_STATUS)
        .await.unwrap();
        let topic_gps = participant
        .create_topic::<GpsData>(GPS_TOPIC, "GpsData", QosKind::Default, None, NO_STATUS)
        .await.unwrap();

        let subscriber = participant
        .create_subscriber(QosKind::Default, None, NO_STATUS)
        .await
        .unwrap();
        
        let reader_memory = subscriber.create_datareader::<SystemStatusMemory>(&topic_memory, QosKind::Default, None, NO_STATUS).await.unwrap();
        let reader_cpu = subscriber.create_datareader::<SystemStatusCpu>(&topic_cpu, QosKind::Default, None, NO_STATUS).await.unwrap();
        let reader_network = subscriber.create_datareader::<SystemStatusNetwork>(&topic_network, QosKind::Default, None, NO_STATUS).await.unwrap();
        let reader_disk = subscriber.create_datareader::<SystemStatusDisk>(&topic_disk, QosKind::Default, None, NO_STATUS).await.unwrap();
        let reader_gps = subscriber.create_datareader::<GpsData>(&topic_gps, QosKind::Default, None, NO_STATUS).await.unwrap();
        
        let rrd_mem = rrd.clone();
        tokio::spawn (async move {
            handle_memory_topic(reader_memory, rrd_mem).await;
        });
        let rrd_cpu = rrd.clone();
        tokio::spawn (async move {
            handle_cpu_topic(reader_cpu, rrd_cpu).await;
        });
        let rrd_network = rrd.clone();
        tokio::spawn (async move {
            handle_network_topic(reader_network, rrd_network).await;
        });
        let rrd_disk = rrd.clone();
        tokio::spawn (async move {
             handle_disk_topic(reader_disk, rrd_disk).await;
        });
        tokio::spawn (async move {
            handle_gps_topic(reader_gps, rrd.clone()).await;
        });
    });
}

// Topic handlers
// Function to handle reading topics from dds and sending them along via rerun
async fn handle_memory_topic (reader: DataReaderAsync<SystemStatusMemory>, rrd: rerun::RecordingStream) {
    loop {
        let data = match reader.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
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
            //log::info!("{:?}", sample);
            let sample_data =  match sample.data() {
                Ok(val) => val,
                Err(e) => {
                    log::error!("Failed unpack DDS system memory sample: {:?}", e);
                    continue;
                }
            };
            let now = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
                Ok(val)=> val.as_secs_f64(),
                Err(e) => {
                    log::error!("Failed to get system time for memory update: {:?}", e);
                    continue;
                }

            };
            rrd.set_time_seconds("system_time", now);
            //Send to existing rerun instance.
            rrd.log("system/memory/percent", &rerun::Scalar::new(sample_data.used_memory as f64 / sample_data.total_memory as f64 * 100.0)).unwrap();  
            rrd.log("system/memory/swap_percent", &rerun::Scalar::new(sample_data.used_swap as f64 / sample_data.total_swap as f64 * 100.0)).unwrap();  
            rrd.log("system/memory", &rerun::TextDocument::new(format!("Memory Usage: {}/{} MB\nSwap Usage: {}/{} MB", 
                        sample_data.used_memory/1024/1024, sample_data.total_memory/1024/1024, sample_data.used_swap/1024/1024, sample_data.total_swap/1024/1024))).unwrap();
        }
    }
}

// Function to handle reading topics from dds and sending them along via mpsc channels
async fn handle_cpu_topic (reader: DataReaderAsync<SystemStatusCpu>, rrd: rerun::RecordingStream) {
    loop {
        let data = match reader.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
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
            //log::info!("{:?}", sample);
            let sample_data =  match sample.data() {
                Ok(val) => val,
                Err(e) => {
                    log::error!("Failed unpack DDS system cpu sample: {:?}", e);
                    continue;
                }
            };
            let now = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
                Ok(val)=> val.as_secs_f64(),
                Err(e) => {
                    log::error!("Failed to get system time for cpu update: {:?}", e);
                    0.0
                }
            };
            rrd.set_time_seconds("system_time", now);

            let mut cpu_usage = Vec::new();
            for cpu_info in sample_data.cpus {
                cpu_usage.push(cpu_info.usage);
            }

            rrd.log("system/cpu", &rerun::BarChart::new(cpu_usage)).unwrap();
        }
    }
}

// Function to handle reading topics from dds and sending them along via mpsc channels
async fn handle_disk_topic (reader: DataReaderAsync<SystemStatusDisk>, rrd: rerun::RecordingStream) {
    loop {
        let data = match reader.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
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
            //log::info!("{:?}", sample);
            let sample_data =  match sample.data() {
                Ok(val) => val,
                Err(e) => {
                    log::error!("Failed unpack DDS system disk sample: {:?}", e);
                    continue;
                }
            };
            let now = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
                Ok(val)=> val.as_secs_f64(),
                Err(e) => {
                    log::error!("Failed to get system time for disk update: {:?}", e);
                    0.0
                }
            };
            rrd.set_time_seconds("system_time", now);

            let mut disk_usage = String::new();
            for disk_info in sample_data.disk_info {
                disk_usage += format!("{}: {}/{} GB ({}%)\n", disk_info.name, disk_info.bytes_used/1024/1024/1024, (disk_info.bytes_available+disk_info.bytes_used)/1024/1024/1024, disk_info.bytes_used as f32/(disk_info.bytes_used+disk_info.bytes_available) as f32*100.0 as f32).as_str();
            }

            rrd.log("system/disks", &rerun::TextDocument::new(disk_usage)).unwrap();
        }
    }
}

// Function to handle reading topics from dds and sending them along via mpsc channels
async fn handle_network_topic (reader: DataReaderAsync<SystemStatusNetwork>, rrd: rerun::RecordingStream) {
    loop {
        let data = match reader.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
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
            //log::info!("{:?}", sample);
            let sample_data =  match sample.data() {
                Ok(val) => val,
                Err(e) => {
                    log::error!("Failed unpack DDS system network sample: {:?}", e);
                    continue;
                }
            };
            let now = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
                Ok(val)=> val.as_secs_f64(),
                Err(e) => {
                    log::error!("Failed to get system time for network update: {:?}", e);
                    0.0
                }
            };
            rrd.set_time_seconds("system_time", now);

            let mut network_usage = String::new();
            for network_info in sample_data.network_info {
                network_usage += 
                    format!("{}: \n\tIP Address: {:?} \n\t {} MB Sent, {} MB Received\n\tErrors: {} TX, {} RX \n ", 
                    network_info.name, network_info.ip_address, network_info.bytes_sent/1024/1024, network_info.bytes_received/1024/1024, network_info.transmit_errors, network_info.receive_errors).as_str();
            }

            rrd.log("system/network", &rerun::TextDocument::new(network_usage)).unwrap();
        }
    }
}

// Function to handle reading topics from dds and sending them along via mpsc channels
async fn handle_gps_topic (reader: DataReaderAsync<GpsData>, rrd: rerun::RecordingStream) {
    loop {
        let data = match reader.take(10, ANY_SAMPLE_STATE, ANY_VIEW_STATE, ANY_INSTANCE_STATE).await {
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
            //log::info!("{:?}", sample);
            let sample_data =  match sample.data() {
                Ok(val) => val,
                Err(e) => {
                    log::error!("Failed unpack DDS system gps sample: {:?}", e);
                    continue;
                }
            };
            let now = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
                Ok(val)=> val.as_secs_f64(),
                Err(e) => {
                    log::error!("Failed to get system time for gps update: {:?}", e);
                    0.0
                }
            };
            rrd.set_time_seconds("system_time", now);

            rrd.log("gps/status", &rerun::TextDocument::new(format!("({}°, {}°, {}m)\n{} m/s, {}°\n Fix: {:?} - {} satellites",
                                    sample_data.latitude, sample_data.longitude, sample_data.altitude, sample_data.velocity, 
                                    sample_data.direction, sample_data.fix, sample_data.good_satellites))).unwrap();
            rrd.log("gps/position", &rerun::GeoPoints::from_lat_lon([(sample_data.latitude, sample_data.longitude)])).unwrap();
        }
    }
}