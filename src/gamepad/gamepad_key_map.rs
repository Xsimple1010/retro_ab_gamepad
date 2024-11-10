use gilrs::Button;
use retro_ab::retro_sys;

#[derive(Debug, Clone, PartialEq)]
pub struct GamepadKeyMap {
    pub native: Button,
    pub retro: u32,
    pub pressed: bool,
}

impl GamepadKeyMap {
    pub fn get_default_key_maps() -> Vec<GamepadKeyMap> {
        vec![
            GamepadKeyMap {
                native: gilrs::Button::DPadDown,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_DOWN,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::DPadLeft,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_LEFT,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::DPadUp,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_UP,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::DPadRight,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_RIGHT,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::South,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_B,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::East,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_A,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::North,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_X,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::West,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_Y,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::LeftThumb,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_L,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::RightThumb,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_R,
                pressed: false,
            },
            GamepadKeyMap {
                native: gilrs::Button::Start,
                retro: retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_START,
                pressed: false,
            },
        ]
    }

    pub fn get_key_name_from_retro_button<'a>(retro: u32) -> &'a str {
        match retro {
            //DPads
            retro_sys::RETRO_DEVICE_ID_JOYPAD_DOWN => "Retro DPad-down",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_UP => "Retro DPad-up",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_LEFT => "Retro DPad-left",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_RIGHT => "Retro DPad-right",

            //buttons
            retro_sys::RETRO_DEVICE_ID_JOYPAD_B => "Retro B",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_A => "Retro A",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_X => "Retro X",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_Y => "Retro Y",

            //Trigger
            retro_sys::RETRO_DEVICE_ID_JOYPAD_L => "Retro L",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_R => "Retro R",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_L2 => "Retro L2",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_R2 => "Retro R2",

            retro_sys::RETRO_DEVICE_ID_JOYPAD_START => "Retro Start",
            retro_sys::RETRO_DEVICE_ID_JOYPAD_SELECT => "Retro Select",
            _ => "Chave desconhecida",
        }
    }

    pub fn get_key_name_from_native_button<'a>(native: &Button) -> &'a str {
        match native {
            //DPads
            Button::DPadUp => "DPad-up",
            Button::DPadDown => "DPad-down",
            Button::DPadLeft => "DPad-left",
            Button::DPadRight => "DPad-right",

            //Buttons
            Button::South => "B",
            Button::East => "A",
            Button::North => "X",
            Button::West => "Y",

            //Trigger
            Button::LeftTrigger => "L",
            Button::RightTrigger => "R",
            Button::LeftTrigger2 => "L2",
            Button::RightTrigger2 => "R2",

            Button::Start => "Start",
            Button::Select => "Select",
            Button::Mode => "mode",

            //Analógicos
            Button::LeftThumb => "LeftThumb",
            Button::RightThumb => "RightThumb",
            _ => "Chave desconhecida",
        }
    }
}
