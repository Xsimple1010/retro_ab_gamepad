use std::sync::{Arc, Mutex};

use gilrs::{Event, GamepadId, Gilrs};

use crate::retro_gamepad::RetroGamePad;

fn get_available_port(max_ports: &Arc<Mutex<usize>>, gamepads: &mut Vec<RetroGamePad>) -> i16 {
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

fn remove(id: GamepadId, gamepads: &Arc<Mutex<Vec<RetroGamePad>>>) {
    match gamepads.lock() {
        Ok(mut list) => {
            list.retain(|g| g.id != id);
        }
        Err(..) => {}
    }
}

pub fn handle_gamepad_events(
    gilrs_instance: Arc<Mutex<Gilrs>>,
    gamepads_list: Arc<Mutex<Vec<RetroGamePad>>>,
    max_ports: Arc<Mutex<usize>>,
) {
    let gilrs = &mut *gilrs_instance.lock().unwrap();

    while let Some(Event { id, event, time: _ }) = gilrs.next_event() {
        match event {
            gilrs::EventType::Connected => {
                let result = try_push(
                    id.clone(),
                    &mut gamepads_list.lock().unwrap(),
                    max_ports.clone(),
                    gilrs,
                );

                match result {
                    Ok(..) => {}
                    Err(..) => {}
                }
            }
            gilrs::EventType::Disconnected => remove(id, &gamepads_list),
            _ => {}
        }

        for gamepad_info in &mut *gamepads_list.lock().unwrap() {
            if gamepad_info.id == id {
                gamepad_info.pool(gilrs);
            }
        }
    }
}
