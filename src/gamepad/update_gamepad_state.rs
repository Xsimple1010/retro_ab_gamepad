use super::{gamepad_key_map::GamepadKeyMap, retro_gamepad::RetroGamePad};
use crate::devices_manager::{Device, DeviceState, DeviceStateListener};
use gilrs::{Button, Event, GamepadId, Gilrs};
use retro_ab::retro_sys::RETRO_DEVICE_JOYPAD;
use std::sync::{Arc, Mutex};

fn get_available_port(
    max_ports: &Arc<Mutex<usize>>,
    gamepads: &Arc<Mutex<Vec<RetroGamePad>>>,
) -> i16 {
    let mut gamepads = gamepads.lock().unwrap();

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

fn remove(id: GamepadId, gamepads: &Arc<Mutex<Vec<RetroGamePad>>>) -> Result<RetroGamePad, ()> {
    if let Ok(list) = &mut gamepads.lock() {
        let mut gm_list = list.clone();
        gm_list.retain(|gm| gm.inner_id == id);

        list.retain(|g| g.inner_id != id);

        return if gm_list.is_empty() {
            Err(())
        } else {
            Ok(gm_list.first().unwrap().to_owned())
        };
    }

    Err(())
}

pub fn connect_handle(
    gamepad_id: GamepadId,
    gilrs: &mut Gilrs,
    gamepad_list: &Arc<Mutex<Vec<RetroGamePad>>>,
    max_ports: &Arc<Mutex<usize>>,
    listener: &Option<Arc<Mutex<DeviceStateListener>>>,
) {
    if let Some(gamepad) = gilrs.connected_gamepad(gamepad_id) {
        let port = get_available_port(&max_ports, &gamepad_list);

        let gamepad = RetroGamePad::new(
            gamepad_id,
            gamepad.name().to_string(),
            port,
            RETRO_DEVICE_JOYPAD,
        );

        let mut gamepads = gamepad_list.lock().unwrap();
        gamepads.push(gamepad.clone());

        if let Some(listener) = listener {
            let listener = listener.lock().unwrap();
            listener(DeviceState::Connected, Device::from_gamepad(&gamepad));
        }
    }
}

fn disconnect_handle(
    id: GamepadId,
    gamepad_list: &Arc<Mutex<Vec<RetroGamePad>>>,
    listener: &Option<Arc<Mutex<DeviceStateListener>>>,
) {
    if let Ok(gamepad) = remove(id, &gamepad_list) {
        if let Some(listener) = listener {
            let listener = listener.lock().unwrap();
            listener(DeviceState::Disconnected, Device::from_gamepad(&gamepad));
        }
    }
}

fn pressed_button_handle(
    button: &Button,
    gamepad_id: GamepadId,
    gamepad_list: &Arc<Mutex<Vec<RetroGamePad>>>,
    listener: &Option<Arc<Mutex<DeviceStateListener>>>,
) {
    for gamepad in &mut *gamepad_list.lock().unwrap() {
        if gamepad.inner_id != gamepad_id {
            return;
        }

        if let Some(listener) = listener {
            let listener = listener.lock().unwrap();

            listener(
                DeviceState::ButtonPressed(
                    GamepadKeyMap::get_key_name_from_native_button(&button).to_owned(),
                ),
                Device::from_gamepad(&gamepad),
            );
        }
    }
}

pub fn gamepad_events_handle(
    gilrs_instance: &Arc<Mutex<Gilrs>>,
    gamepad_list: &Arc<Mutex<Vec<RetroGamePad>>>,
    max_ports: &Arc<Mutex<usize>>,
    listener: &Option<Arc<Mutex<DeviceStateListener>>>,
) {
    let gilrs = &mut *gilrs_instance.lock().unwrap();

    while let Some(Event {
        id, event, time: _, ..
    }) = gilrs.next_event()
    {
        match event {
            gilrs::EventType::Connected => {
                connect_handle(id, gilrs, &gamepad_list, &max_ports, &listener);
            }
            gilrs::EventType::Disconnected => disconnect_handle(id, &gamepad_list, &listener),
            gilrs::EventType::ButtonPressed(button, _) => {
                pressed_button_handle(&button, id, &gamepad_list, &listener)
            }
            _ => {}
        }

        for gamepad_info in &mut *gamepad_list.lock().unwrap() {
            if gamepad_info.inner_id == id {
                gamepad_info.pool(&gilrs);
            }
        }
    }
}
