use std::sync::{Arc, Mutex};

use gilrs::{Event, GamepadId, Gilrs};

use crate::{key_map::KeyMap, retro_gamepad::RetroGamePad};

pub type GamepadStateListener = fn(GamePadState, RetroGamePad);

fn get_available_port(max_ports: &Arc<Mutex<usize>>, gamepads: &mut [RetroGamePad]) -> i16 {
    gamepads.sort_by(|gmp, f_gmp| gmp.retro_port.cmp(&f_gmp.retro_port));

    if let Some(gamepad) = gamepads.last() {
        let current_port = gamepad.retro_port + 1;

        if current_port as usize > *max_ports.lock().unwrap() {
            return -1;
        }

        return current_port;
    }

    0
}

fn try_push(
    id: GamepadId,
    gamepads: &mut Vec<RetroGamePad>,
    max_ports: Arc<Mutex<usize>>,
    gilrs: &mut Gilrs,
) -> Result<RetroGamePad, ()> {
    if let Some(gamepad) = gilrs.connected_gamepad(id) {
        let retro_port = get_available_port(&max_ports, gamepads);

        let gm = RetroGamePad::new(
            id,
            gamepad.name().to_string(),
            retro_port,
            retro_ab::retro_sys::RETRO_DEVICE_JOYPAD,
        );

        gamepads.push(gm.clone());

        return Ok(gm);
    }

    Err(())
}

fn remove(id: GamepadId, gamepads: &Arc<Mutex<Vec<RetroGamePad>>>) -> Result<RetroGamePad, ()> {
    match gamepads.lock() {
        Ok(mut list) => {
            let mut gm_list = list.clone();
            gm_list.retain(|gm| gm.id == id);

            list.retain(|g| g.id != id);

            return if gm_list.is_empty() {
                Err(())
            } else {
                Ok(gm_list.first().unwrap().to_owned())
            }
        }
        Err(..) => Err(()),
    }
}

#[derive(Debug)]
pub enum GamePadState {
    Connected,
    Disconnected,
    ButtonPressed(String),
}

pub fn gamepad_events_handle(
    gilrs_instance: Arc<Mutex<Gilrs>>,
    gamepad_list: Arc<Mutex<Vec<RetroGamePad>>>,
    max_ports: Arc<Mutex<usize>>,
    listener: Arc<Mutex<GamepadStateListener>>,
) {
    let gilrs = &mut *gilrs_instance.lock().unwrap();

    while let Some(Event { id, event, time: _ }) = gilrs.next_event() {
        match event {
            gilrs::EventType::Connected => {
                let result = try_push(
                    id,
                    &mut gamepad_list.lock().unwrap(),
                    max_ports.clone(),
                    gilrs,
                );

                if let Ok(gm) = result {
                   if let Ok(listener)  =listener.lock() {
                       listener(GamePadState::Connected, gm)
                   }
                }
            }
            gilrs::EventType::Disconnected => {
                let result = remove(id, &gamepad_list);

                match result {
                    Ok(gm) => match listener.lock() {
                        Ok(listener) => listener(GamePadState::Disconnected, gm),
                        Err(..) => {}
                    },
                    Err(..) => {}
                }
            }
            gilrs::EventType::ButtonPressed(button, _) => {
                for gamepad_info in &mut *gamepad_list.lock().unwrap() {
                    if gamepad_info.id == id {
                        match listener.lock() {
                            Ok(listener) => {
                                listener(
                                    GamePadState::ButtonPressed(
                                        KeyMap::get_key_name_from_native_button(&button).to_owned(),
                                    ),
                                    gamepad_info.clone(),
                                );
                            }
                            Err(..) => {}
                        }
                    }
                }
            }
            _ => {}
        }

        for gamepad_info in &mut *gamepad_list.lock().unwrap() {
            if gamepad_info.id == id {
                gamepad_info.pool(gilrs);
            }
        }
    }
}
