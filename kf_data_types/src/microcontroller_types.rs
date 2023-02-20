use serde::{Serialize, Deserialize};

/// These structs are use to communicate over a serial connection with the ocntrol microcontroller. They are then rebroadcast on their own topics through DDS.

/// The main enum for packing individual control or response messages.
/// This us used for dispatching the messages both bottom and top side.
/// State and Error are sent up from the uC, RequestState and SetOutput are sent down.
#[derive(Serialize, Deserialize, Debug)]
pub enum MicroMessages {
    State (State),
    ControllerState (ControllerState),
    RequestState,
    RequestControllerState,
    SetOutput(Vec<Output>),
    Error(Vec<u8>),
}

/// Enum containing all the controllable output, including analog ones (individual motor control)
#[derive(Serialize, Deserialize, Debug)]
pub enum Output {
    StarboardLight(bool),
    PortLight(bool),
    Fan(bool),
    StarboardThrottle(i8),
    PortThrottle(i8),
}

/// Structure containing the current state, excluding the controller state.
#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    starboard_light: bool,
    port_lights: bool,
    fan: bool,
    starboard_throttle: i8,
    port_throttle: i8
}

/// Structure containing the raw controller stick position state.
#[derive(Serialize, Deserialize, Debug)]
pub struct ControllerState {
    overridden: bool,
    throttle: u8,
    turn: u8,
    switch: u8,
}