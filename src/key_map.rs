use gilrs::Button;
use retro_ab::retro_sys;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct KeyMap {
    pub native: Button,
    pub retro: u32,
    pub pressed: bool,
}

pub fn get_key_name_from_native_button<'a>(native: &Button) -> &'a str {
    match native {
        //DPads
        Button::DPadUp => "DPad-up",
        Button::DPadDown => "DPad-down",
        Button::DPadLeft => "DPad-left",
        Button::DPadRight => "DPad-right",

        //buttons
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
        _ => "Chave desconhecida",
    }
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

impl KeyMap {
    pub fn get_key_native_name(&self) -> &str {
        get_key_name_from_native_button(&self.native)
    }

    pub fn get_key_retro_name(&self) -> &str {
        get_key_name_from_retro_button(self.retro)
    }
}
