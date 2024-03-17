use crate::{
    handle_event::{handle_gamepad_events, GamePadState, GamepadStateListener},
    retro_gamepad::RetroGamePad,
    thread_event::create_gamepad_thread,
};
use gilrs::Gilrs;
use retro_ab::retro_sys::{retro_rumble_effect, RETRO_DEVICE_ID_JOYPAD_MASK};
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref GAMEPADS: Arc<Mutex<Vec<RetroGamePad>>> = Arc::new(Mutex::new(Vec::new()));
    static ref GILRS_INSTANCE: Arc<Mutex<Gilrs>> = Arc::new(Mutex::new(Gilrs::new().unwrap()));
    static ref MAX_PORTS: Arc<Mutex<usize>> = Arc::new(Mutex::new(2));
    static ref CALLBACK: Arc<Mutex<GamepadStateListener>> = Arc::new(Mutex::new(none));
}

fn none(_gs: GamePadState, _rg: RetroGamePad) {}

pub struct GamepadContext {
    _is_running: Arc<Mutex<bool>>,
}

impl Drop for GamepadContext {
    fn drop(&mut self) {
        GAMEPADS.lock().unwrap().clear();
        *self._is_running.lock().unwrap() = false;
    }
}

impl GamepadContext {
    pub fn get_list(&self) -> Arc<Mutex<Vec<RetroGamePad>>> {
        GAMEPADS.clone()
    }

    pub fn new(cb: Option<GamepadStateListener>) -> GamepadContext {
        let is_running = Arc::new(Mutex::new(true));

        if let Some(cb) = cb {
            *CALLBACK.lock().unwrap() = cb;
        }

        create_gamepad_thread(
            GAMEPADS.clone(),
            GILRS_INSTANCE.clone(),
            is_running.clone(),
            MAX_PORTS.clone(),
            CALLBACK.clone(),
        );

        Self {
            _is_running: is_running,
        }
    }
}

pub fn input_poll_callback() {
    handle_gamepad_events(
        GILRS_INSTANCE.clone(),
        GAMEPADS.clone(),
        MAX_PORTS.clone(),
        CALLBACK.clone(),
    );
}

pub fn input_state_callback(port: i16, _device: i16, _index: i16, id: i16) -> i16 {
    for gamepad_info in &*GAMEPADS.lock().unwrap() {
        if gamepad_info.retro_port == port {
            if id as u32 != RETRO_DEVICE_ID_JOYPAD_MASK {
                let pressed = gamepad_info.key_pressed(id);

                if pressed {
                    return 1;
                } else {
                    return 0;
                }
            } else {
                return gamepad_info.retro_bitmask() as i16;
            }
        }
    }

    0
}

pub fn rumble_callback(
    port: ::std::os::raw::c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    println!(
        "port:{:?} effect:{:?} strength:{:?}",
        port, effect, strength
    );
    true
}
