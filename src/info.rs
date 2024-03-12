use crate::key_map::{self, KeyMap};
use gilrs::{GamepadId, Gilrs};

#[derive(Debug, Clone)]
pub struct GamePadInfo {
    pub id: GamepadId,
    pub name: String,
    #[doc = "indicar ao Core em qual porta o controle esta conectado, se o valor for -1 significa que todas as porta suportas pelo Core ja estão sendo usadas"]
    pub retro_port: i16,
    #[doc = "padrão RETRO_DEVICE_JOYPAD"]
    pub retro_type: u32,
    pub key_map: Vec<KeyMap>,
}

impl GamePadInfo {
    pub fn new(id: GamepadId, name: String, retro_port: i16, retro_type: u32) -> GamePadInfo {
        let mut key_map: Vec<KeyMap> = Vec::new();

        key_map.push(KeyMap {
            native: gilrs::Button::DPadDown,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_DOWN,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::DPadLeft,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_LEFT,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::DPadUp,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_UP,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::DPadRight,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_RIGHT,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::South,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_B,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::East,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_A,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::North,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_X,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::West,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_Y,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::Start,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_START,
            pressed: false,
        });

        key_map.push(KeyMap {
            native: gilrs::Button::Start,
            retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_START,
            pressed: false,
        });

        Self {
            id,
            name,
            retro_port,
            retro_type,
            key_map,
        }
    }

    pub fn pool(&mut self, gilrs: &Gilrs) {
        let gamepad = gilrs.gamepad(self.id);

        for key_info in &mut self.key_map {
            key_info.pressed = gamepad.is_pressed(key_info.native);
        }
    }

    pub fn get_key_state(&self, retro_id: i16) -> bool {
        for key_map in &self.key_map {
            if key_map.retro as i16 == retro_id {
                return key_map.pressed;
            }
        }

        false
    }
}
