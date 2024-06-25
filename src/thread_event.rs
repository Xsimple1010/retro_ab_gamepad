use crate::{
    handle_event::{handle_gamepad_events, GamepadStateListener},
    retro_gamepad::RetroGamePad,
};
use gilrs::Gilrs;
use std::{
    sync::{Arc, Mutex},
    thread,
};

/// a thread deve espera por um dado momento antes de tentar ler o proximo evento.
/// quando um rom é iniciada a thread nao terá oportunidade de ler os eventos
// então nesse momento o único local que conseguira recebe esses evento está em "context.rs" mais especificamente na função "input_poll_callback"
pub fn create_gamepad_thread(
    gamepads: Arc<Mutex<Vec<RetroGamePad>>>,
    gilrs: Arc<Mutex<Gilrs>>,
    is_running: Arc<Mutex<bool>>,
    max_ports: Arc<Mutex<usize>>,
    listener: Arc<Mutex<GamepadStateListener>>,
) {
    thread::spawn(move || {
        let gamepads_list_ptr = gamepads;
        let gilrs_instance_ptr = gilrs;
        let is_running_ptr = is_running;
        let max_ports_ptr = max_ports;
        let listener_ptr = listener;

        while *is_running_ptr.lock().unwrap() {
            handle_gamepad_events(
                gilrs_instance_ptr.clone(),
                gamepads_list_ptr.clone(),
                max_ports_ptr.clone(),
                listener_ptr.clone(),
            );
        }
    });
}
