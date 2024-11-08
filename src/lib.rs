extern crate gilrs;
#[macro_use]
extern crate lazy_static;

mod constants;
mod devices_manager;
mod gamepad;
mod retro_ab_controller;
mod state_thread;

pub use devices_manager::{Device, DeviceState, DeviceStateListener};
pub use retro_ab_controller::{
    input_poll_callback, input_state_callback, rumble_callback, RetroAbController,
};
