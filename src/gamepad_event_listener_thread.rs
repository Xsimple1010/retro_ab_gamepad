use crate::{
    handle_event::{gamepad_events_handle, GamepadStateListener},
    retro_gamepad::RetroGamePad,
};
use gilrs::Gilrs;
use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};


/// # gamepad event listener thread
///
/// Isso é util se quando não há uma *rom* em execução, mas ainda é necessário ouvir os eventos do
/// gamepad. Por exemplo, a *rom* foi fechada, mas a interface do usuário ainda precisa ser
/// notificada sobre os eventos do gamepad.
///
/// Aviso: para evitar uso desnecessário de CPU use isso somente quando não hover uma
/// *rom* em execução!. Use o terceiro parâmetro "event_thread_is_enabled" para encerar a
/// execução da thread quando não precisar mais dela.
pub fn create_gamepad_listener_thread(
    gamepads: Arc<Mutex<Vec<RetroGamePad>>>,
    gilrs: Arc<Mutex<Gilrs>>,
    event_thread_is_enabled: Arc<Mutex<bool>>,
    max_ports: Arc<Mutex<usize>>,
    listener: Arc<Mutex<GamepadStateListener>>,
) {
    thread::spawn(move || {
        while *event_thread_is_enabled.lock().unwrap() {
            //sem isso há um grande consumo de cpu
            sleep(Duration::from_millis(16));

            gamepad_events_handle(
                gilrs.clone(),
                gamepads.clone(),
                max_ports.clone(),
                listener.clone(),
            );
        }
    });
}
