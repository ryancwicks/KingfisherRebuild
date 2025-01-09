//! This module contains the hard coded names of various DDS topics use by different systems.

use dust_dds::{
    topic_definition::type_support::DdsType,
};
use serde::Serialize;

pub const MICROCONTROLLER_STATUS_TOPIC: &str = "mcu_status";
pub const MICROCONTROLLER_CONTROL_TOPIC: &str = "mcu_control";
pub const GPS_TOPIC: &str = "gps_data";
pub const IMU_TOPIC: &str = "imu_data";

pub const SYSTEM_STATUS_CPU_TOPIC: &str = "system_status/cpu";
pub const SYSTEM_STATUS_MEMORY_TOPIC: &str = "system_status/memory";
pub const SYSTEM_STATUS_NETWORK_TOPIC: &str = "system_status/network";
pub const SYSTEM_STATUS_DISK_TOPIC: &str = "system_status/disk";

#[derive(DdsType, Debug, Clone, Serialize)]
pub enum GpsFix {
    None,
    Fix2D,
    Fix3D
}

///GPS Types
#[derive(DdsType, Debug, Clone, Serialize)]
pub struct GpsData {
    #[dust_dds(key)]
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub velocity: f32,
    pub direction: f32,
    pub fix: GpsFix,
    pub good_satellites: u8
}

///Types for System Status

#[derive(DdsType, Debug, Clone, Serialize)]
pub struct CpuInfo {
    pub name: String,
    pub usage: f32
}

#[derive(DdsType, Debug, Clone, Serialize)]
pub struct SystemStatusCpu {
    #[dust_dds(key)]
    pub id: String,
    pub cpus: Vec<CpuInfo>
}

#[derive(DdsType, Debug, Clone, Serialize)]
pub struct SystemStatusMemory {
    #[dust_dds(key)]
    pub id: String,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64
}

#[derive(DdsType, Debug, Clone, Serialize)]
pub struct NetworkInfo {
    pub name: String,
    pub ip_address: Vec<String>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub transmit_errors: u64,
    pub receive_errors: u64,
}

#[derive(DdsType, Debug, Clone, Serialize)]
pub struct SystemStatusNetwork {
    #[dust_dds(key)]
    pub id: String,
    pub network_info: Vec<NetworkInfo>    
}

#[derive(DdsType, Debug, Clone, Serialize)]
pub struct DiskInfo {
    pub name: String,
    pub bytes_used: u64,
    pub bytes_available: u64
}

#[derive(DdsType, Debug, Clone, Serialize)]
pub struct SystemStatusDisk {
    #[dust_dds(key)]
    pub id: String,
    pub disk_info: Vec<DiskInfo>,
}