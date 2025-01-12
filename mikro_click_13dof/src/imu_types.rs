use serde::{Serialize, Deserialize};

/// The main enum for packing individual control messages. (from imu)
/// This is send my the bottomside to stream data
#[derive(Serialize, Deserialize, Debug)]
pub enum ImuMessages {
    // Current implementation just sends all data at the same time in accel (g), gyro(degrees/s), mag(uT)
    Imu(f32, f32, f32, f32, f32, f32, f32, f32, f32)
}