use crate::gamepad::{retro_gamepad::RetroGamePad, update_gamepad_state::gamepad_events_handle};
use gilrs::Gilrs;
use retro_ab::retro_sys::{retro_rumble_effect, RETRO_DEVICE_ID_JOYPAD_MASK};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug)]
pub enum DeviceState {
    Connected,
    Disconnected,
    ButtonPressed(String),
}

#[derive(Debug, Clone, Copy)]
pub enum DeviceType {
    Gamepad,
    Keyboard,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub retro_port: i16,
    pub retro_type: u32,
    pub device_type: DeviceType,
}

impl Device {
    pub fn from_gamepad(gamepad: &RetroGamePad) -> Self {
        Self {
            id: gamepad.id.clone(),
            device_type: DeviceType::Gamepad,
            name: gamepad.name.clone(),
            retro_port: gamepad.retro_port,
            retro_type: gamepad.retro_type,
        }
    }
}

pub type DeviceStateListener = fn(DeviceState, Device);

#[derive(Debug, Clone)]
pub struct DevicesManager {
    gilrs_instance: Arc<Mutex<Gilrs>>,
    pub gamepad_list: Arc<Mutex<Vec<RetroGamePad>>>,
    max_ports: Arc<Mutex<usize>>,
    listener: Option<Arc<Mutex<DeviceStateListener>>>,
}

impl DevicesManager {
    pub fn new(listener: Option<Arc<Mutex<DeviceStateListener>>>) -> Self {
        Self {
            gilrs_instance: Arc::new(Mutex::new(Gilrs::new().unwrap())),
            gamepad_list: Arc::new(Mutex::new(Vec::new())),
            max_ports: Arc::new(Mutex::new(2)),
            listener,
        }
    }

    pub fn set_listener(&mut self, listener: Arc<Mutex<DeviceStateListener>>) {
        self.listener = Some(listener);
    }

    pub fn update_state(&mut self) {
        gamepad_events_handle(
            &mut self.gilrs_instance,
            &self.gamepad_list,
            &self.max_ports,
            &self.listener,
        );
    }

    pub fn input_state_callback(&self, port: i16, id: i16) -> i16 {
        for gamepad in &*self.gamepad_list.lock().unwrap() {
            if gamepad.retro_port == port {
                return if id as u32 != RETRO_DEVICE_ID_JOYPAD_MASK {
                    let pressed = gamepad.key_pressed(id);

                    if pressed {
                        1
                    } else {
                        0
                    }
                } else {
                    gamepad.retro_bitmask() as i16
                };
            }
        }

        0
    }

    pub fn apply_rumble(
        &self,
        port: std::os::raw::c_uint,
        effect: retro_rumble_effect,
        strength: u16,
    ) -> bool {
        println!(
            "port:{:?} effect:{:?} strength:{:?}",
            port, effect, strength
        );
        true
    }
}

pub trait DevicesRequireFunctions {
    fn key_pressed(&self, retro_id: i16) -> bool;

    fn retro_bitmask(&self) -> u32;
}
