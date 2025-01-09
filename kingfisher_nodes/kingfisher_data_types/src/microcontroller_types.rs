use serde::{Serialize, Deserialize};

#[cfg(not(feature = "std"))]
use heapless::Vec;

/// These structs are use to communicate over a serial connection with the ocntrol microcontroller. They are then rebroadcast on their own topics through DDS.

/// The main enum for packing individual control messages. (to microcontroller)
/// This is read by the bottomside to conrol vehicle state
#[derive(Serialize, Deserialize, Debug)]
pub enum MicroControlMessages {
    RequestState,
    RequestControllerState,

    #[cfg(not(feature = "std"))]
    SetOutput(Vec<Output, 7>),

    #[cfg(feature = "std")]
    SetOutput(Vec<Output>),
}

/// The main enum for packing individual status messages (from microcontroller).
/// These are sent to the device to change the state.
#[derive(Serialize, Deserialize, Debug)]
pub enum MicroStatusMessages {
    State (State),
    ControllerState (ControllerState),

    #[cfg(not(feature = "std"))]
    Debug(Vec<u8, 250>),

    #[cfg(feature = "std")]
    Debug(Vec<u8>),
}

/// Enum containing all the controllable output, including analog ones (individual motor control)
#[derive(Serialize, Deserialize, Debug)]
pub enum Output {
    StarboardLight(bool),
    PortLight(bool),
    StarboardPower(bool),
    PortPower(bool),
    StarboardThrottle(i8),
    PortThrottle(i8),
}

/// Structure containing the current state, excluding the controller state.
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub starboard_light: bool,
    pub port_lights: bool,
    pub starboard_power: bool,
    pub port_power: bool,
    pub starboard_throttle: u8,
    pub port_throttle: u8
}

/// Structure containing the raw controller stick position state.
#[derive(Serialize, Deserialize, Debug)]
pub struct ControllerState {
    pub overridden: bool,
    pub throttle: u16,
    pub turn: u16,
    pub switch: u16,
}