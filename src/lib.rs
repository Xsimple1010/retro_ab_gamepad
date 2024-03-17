extern crate gilrs;
#[macro_use]
extern crate lazy_static;

mod handle_event;
mod thread_event;

pub use handle_event::{GamePadState, GamepadStateListener};
pub mod context;
pub mod key_map;
pub mod retro_gamepad;
