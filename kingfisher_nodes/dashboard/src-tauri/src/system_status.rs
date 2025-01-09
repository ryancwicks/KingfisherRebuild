use tauri::{AppHandle, ipc::Channel};
use serde::Serialize;
use tokio::sync::mpsc;
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
use tauri::Manager;
use std::sync::Arc;
use tokio::sync::Mutex;

use rerun;


// Structure to store all the dds topic receivers, to be used as part of the application state.
pub struct DdsTopicReceivers {
    
    rx_sys_mem: Arc<Mutex<mpsc::Receiver<SystemStatusMemory>>>,
    rx_sys_cpu: Arc<Mutex<mpsc::Receiver<SystemStatusCpu>>>,
    rx_sys_disk: Arc<Mutex<mpsc::Receiver<SystemStatusDisk>>>,
    rx_sys_network: Arc<Mutex<mpsc::Receiver<SystemStatusNetwork>>>,
    rx_gps: Arc<Mutex<mpsc::Receiver<GpsData>>>
}

//Sets up the App State
pub fn setup_app_state() -> DdsTopicReceivers {

    let rrd = rerun::RecordingStreamBuilder::new("kingfisher")
                .serve_web("0.0.0.0", Default::default(),
                Default::default(),
                rerun::MemoryLimit::from_fraction_of_total(0.25),
                false).unwrap();

    

    let (tx_sys_mem, rx_sys_mem) = mpsc::channel::<SystemStatusMemory>(1);
    let (tx_sys_cpu, rx_sys_cpu) = mpsc::channel::<SystemStatusCpu>(1);
    let (tx_sys_disk, rx_sys_disk) = mpsc::channel::<SystemStatusDisk>(1);
    let (tx_sys_network, rx_sys_network) = mpsc::channel::<SystemStatusNetwork>(1);
    let (tx_gps, rx_gps) = mpsc::channel::<GpsData>(1);
    
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
        .await
        .unwrap();
        
        let subscriber = participant
        .create_subscriber(QosKind::Default, None, NO_STATUS)
        .await
        .unwrap();
        
        let reader_memory = subscriber.create_datareader::<SystemStatusMemory>(&topic_memory, QosKind::Default, None, NO_STATUS).await.unwrap();
        
        tokio::spawn (async move {
            handle_memory_topic(reader_memory, tx_sys_mem).await;
        });
    });
    
    DdsTopicReceivers {
        rx_sys_mem: Arc::new(Mutex::new(rx_sys_mem)),
        rx_sys_cpu: Arc::new(Mutex::new(rx_sys_cpu)),
        rx_sys_disk: Arc::new(Mutex::new(rx_sys_disk)),
        rx_sys_network: Arc::new(Mutex::new(rx_sys_network)),
        rx_gps: Arc::new(Mutex::new(rx_gps))
    }
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum DataUpdate {
    #[serde(rename_all = "camelCase")]
    DataMemory {
        data: SystemStatusMemory,
    },
    DataDisk {
        data: SystemStatusDisk,
    },
    DataCpu {
        data: SystemStatusCpu,
    },
    DataNetwork {
        data: SystemStatusNetwork,
    },
    DataGps {
        data: GpsData,
    }
}

#[tauri::command]
pub fn connect_dds_topics(app: AppHandle, on_event: Channel<DataUpdate>) {
    
    let data = app.state::<DdsTopicReceivers>();
    
    //clone the receiver.
    let rx_sys_mem = data.rx_sys_mem.clone();

    
    tauri::async_runtime::spawn(async move {
        loop {
            let mut sender = rx_sys_mem.lock().await;
            match sender.recv().await {
                Some(val) => {
                    on_event.send(DataUpdate::DataMemory { data: val }).unwrap();
                },
                None => ()
            };
        }
    });
}

// Topic handlers
// Function to handle reading topics from dds and sending them along via mpsc channels
async fn handle_memory_topic (reader: DataReaderAsync<SystemStatusMemory>, sender: mpsc::Sender<SystemStatusMemory>) {
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

            //Send to existing rerun instance.
            
            match sender.send(sample_data).await {
                Ok(_) => (),
                Err(e) => {
                    log::error!("Unexpected error sending system memory data: {:?}", e);
                }
            };   
        }
    }
}