//! Program that publishes system statistic to DDS.
use sysinfo::{Disks, Networks, System};
use config::Config;
use clap::Parser;
use kingfisher_data_types::dds_topics::{
    SYSTEM_STATUS_CPU_TOPIC, SYSTEM_STATUS_DISK_TOPIC, SYSTEM_STATUS_MEMORY_TOPIC, 
    SYSTEM_STATUS_NETWORK_TOPIC, SystemStatusMemory,
    SystemStatusDisk, SystemStatusNetwork, DiskInfo,
    NetworkInfo, CpuInfo, SystemStatusCpu
};
use dust_dds::{
    domain::domain_participant_factory::DomainParticipantFactory,
    infrastructure::{qos::QosKind, status::NO_STATUS},
};


/// Parser for command line parameters
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CommandLineParameters {
    /// Path to the config file
    #[arg(short, long)]
    config_file: Option<String>,
}

fn main() {
    //switch this to syslog later: https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/log.html#log-to-the-unix-syslog
    env_logger::init();
    log::info!("Starting system status publisher.");
    
    let cli = CommandLineParameters::parse();
    let config_file = match cli.config_file {
        Some(val) => val,
        None => String::from("./system_status.toml")
    };

    let settings = Config::builder()
        .add_source(config::File::with_name(&config_file))
        .build()
        .unwrap();

    let network_interfaces: Vec<String> = settings.get_array("network_interface").unwrap_or(Vec::new()).iter().map(|x| {
        x.clone().into_string().unwrap()
    }).collect();
    let hard_drives: Vec<String> = settings.get_array("hard_drive").unwrap_or(Vec::new()).iter().map(|x| {
        x.clone().into_string().unwrap()
    }).collect();
    let update_rate = settings.get_int("update_rate").unwrap_or(1) as u64;

    // Please note that we use "new_all" to ensure that all lists of
    // CPUs and processes are filled!
    let mut sys = System::new_all();


    let domain_id = kingfisher_data_types::DEFAULT_DOMAIN;
    let participant_factory = DomainParticipantFactory::get_instance();

    let participant = participant_factory
        .create_participant(domain_id, QosKind::Default, None, NO_STATUS)
        .unwrap();

    let topic_memory = participant
        .create_topic::<SystemStatusMemory>(SYSTEM_STATUS_MEMORY_TOPIC, "SystemStatusMemory", QosKind::Default, None, NO_STATUS)
        .unwrap();
    let topic_disk = participant
        .create_topic::<SystemStatusDisk>(SYSTEM_STATUS_DISK_TOPIC, "SystemStatusDisk", QosKind::Default, None, NO_STATUS)
        .unwrap();
    let topic_network = participant.create_topic::<SystemStatusNetwork>(SYSTEM_STATUS_NETWORK_TOPIC, "SystemStatusNetwork", QosKind::Default, None, NO_STATUS).unwrap();
    let topic_cpu = participant.create_topic::<SystemStatusCpu>(SYSTEM_STATUS_CPU_TOPIC, "SystemStatusCpu", QosKind::Default, None, NO_STATUS).unwrap();

    let publisher = participant
        .create_publisher(QosKind::Default, None, NO_STATUS)
        .unwrap();

    let memory_writer = publisher
        .create_datawriter::<SystemStatusMemory>(&topic_memory, QosKind::Default, None, NO_STATUS)
        .unwrap();
    let disk_writer = publisher
    .create_datawriter::<SystemStatusDisk>(&topic_disk, QosKind::Default, None, NO_STATUS)
    .unwrap();
    let network_writer = publisher
    .create_datawriter::<SystemStatusNetwork>(&topic_network, QosKind::Default, None, NO_STATUS)
    .unwrap();
    let cpu_writer = publisher
    .create_datawriter::<SystemStatusCpu>(&topic_cpu, QosKind::Default, None, NO_STATUS)
    .unwrap();
    
    loop {
        
        //Memory update
        sys.refresh_memory();
        let mem = SystemStatusMemory {
            id: kingfisher_data_types::DEFAULT_ID.into(),
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap()
        };

        match memory_writer.write(&mem, None) {
            Ok(_)=> {
                log::info!("Sent memory update message.");
            },
            Err(e) => {
                log::error!("Failed to send memory message: {:?}", e);
            }
        }

        //CPU update
        sys.refresh_cpu_all();
        let mut cpus_info = Vec::new();
        for cpu in sys.cpus() {
            cpus_info.push(CpuInfo {
                name: cpu.name().into(),
                usage: cpu.cpu_usage()
            })
        }
        let cpu_msg = SystemStatusCpu {
            id: kingfisher_data_types::DEFAULT_ID.into(),
            cpus: cpus_info
        };
        match cpu_writer.write(&cpu_msg, None) {
            Ok(_)=> {
                log::info!("Sent CPU update message.");
            },
            Err(e) => {
                log::error!("Failed to send CPU message: {:?}", e);
            }
        }

        //Disk Update
        let mut disk_infos = Vec::new();
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            for name in &hard_drives {
                if disk.name().to_str().unwrap().contains(name) {
                    let disk_info = DiskInfo {
                        name: name.clone(),
                        bytes_used: disk.total_space() - disk.available_space(),
                        bytes_available: disk.available_space()
                    };

                    disk_infos.push(disk_info);
                }
            }
        }
        disk_infos.sort_by_key(|d| d.name.clone());
        let disk_status = SystemStatusDisk {
            id: kingfisher_data_types::DEFAULT_ID.into(),
            disk_info: disk_infos
        };

        match disk_writer.write(&disk_status, None) {
            Ok(_) => {
                log::info!("Sent disk status update.");
            }
            Err(e) => {
                log::error!("Failed to send disk status update: {:?}", e);
            }
        };
        
        //Network Update
        let mut net_infos = Vec::new();
        let networks = Networks::new_with_refreshed_list();
        for (interface_name, data) in &networks {
            for name in &network_interfaces {
                if interface_name.contains(name) {
                    let mut ip_addresses = Vec::new();
                    for ip in data.ip_networks() {
                        ip_addresses.push(format!("{}/{}", ip.addr, ip.prefix));
                    }
                    ip_addresses.sort();
                    let net_info = NetworkInfo {
                        name: interface_name.clone(),
                        ip_address: ip_addresses,
                        bytes_sent: data.total_transmitted(),
                        bytes_received: data.total_received(),
                        transmit_errors: data.total_errors_on_transmitted(),
                        receive_errors: data.total_errors_on_received()
                    };

                    net_infos.push(net_info);
                }
            }
        }
        net_infos.sort_by_key(|d| d.name.clone());

        let net_info = SystemStatusNetwork {
            id: kingfisher_data_types::DEFAULT_ID.into(),
            network_info: net_infos
        };

        match network_writer.write(&net_info, None) {
            Ok(_) => {
                log::info!("Sent network status update.");
            },
            Err(e) => {
                log::error!("Failed to send network status update: {:?}", e);
            }
        }


        // // Components temperature:
        // let components = Components::new_with_refreshed_list();
        // println!("=> components:");
        // for component in &components {
        //     println!("{component:?}");
        // }

        std::thread::sleep(std::time::Duration::from_secs(update_rate));
        
    }
    
}
