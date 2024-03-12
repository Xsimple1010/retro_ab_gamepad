use crate::retro_gamepad::RetroGamePad;
use gilrs::{Event, Gilrs};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

lazy_static! {
    static ref GAMEPADS: Arc<Mutex<Vec<RetroGamePad>>> = Arc::new(Mutex::new(Vec::new()));
    static ref GAMEPAD_INSTANCE: Mutex<Gilrs> = Mutex::new(Gilrs::new().unwrap());
}

pub fn input_poll_callback() {
    let gilrs: &mut Gilrs = &mut *GAMEPAD_INSTANCE.lock().unwrap();

    while let Some(Event {
        id,
        event: _,
        time: _,
    }) = gilrs.next_event()
    {
        for gamepad_info in &mut *GAMEPADS.lock().unwrap() {
            if gamepad_info.id == id {
                gamepad_info.pool(gilrs);
            }
        }
    }
}

pub fn input_state_callback(port: i16, _device: i16, _index: i16, id: i16) -> i16 {
    for gamepad_info in &*GAMEPADS.lock().unwrap() {
        if gamepad_info.retro_port == port {
            if id as u32 != retro_ab::retro_sys::RETRO_DEVICE_ID_JOYPAD_MASK {
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

pub struct GamepadContext {
    max_ports: usize,
}

impl Drop for GamepadContext {
    fn drop(&mut self) {
        GAMEPADS.lock().unwrap().clear();
    }
}

impl GamepadContext {
    pub fn search(&mut self) -> Arc<Mutex<Vec<RetroGamePad>>> {
        let start = Instant::now();

        let gilrs = &mut *GAMEPAD_INSTANCE.lock().unwrap();

        while !self.time_eq(start, 100) {
            while let Some(Event {
                id,
                event: _,
                time: _,
            }) = gilrs.next_event()
            {
                let retro_port = self.get_available_port();

                if let Some(gamepad) = gilrs.connected_gamepad(id) {
                    GAMEPADS.lock().unwrap().push(RetroGamePad::new(
                        id,
                        gamepad.name().to_string(),
                        retro_port,
                        retro_ab::retro_sys::RETRO_DEVICE_JOYPAD,
                    ));
                }
            }
        }

        GAMEPADS.clone()
    }

    fn get_available_port(&mut self) -> i16 {
        GAMEPADS
            .lock()
            .unwrap()
            .sort_by(|gmp, f_gmp| gmp.retro_port.cmp(&f_gmp.retro_port));

        if let Some(gamepad) = GAMEPADS.lock().unwrap().last() {
            let current_port = gamepad.retro_port + 1;

            if current_port as usize > self.max_ports {
                return -1;
            }

            return current_port;
        }

        0
    }

    pub fn vibrate(&self) {}

    fn time_eq(&self, time: Instant, end: u128) -> bool {
        (Instant::now() - time).as_millis() == end
    }

    pub fn new(max_ports: usize) -> GamepadContext {
        let _gilrs = GAMEPAD_INSTANCE.lock().unwrap();
        Self { max_ports }
    }
}
