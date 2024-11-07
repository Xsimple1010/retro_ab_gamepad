use retro_ab::erro_handle::ErroHandle;
use retro_ab::erro_handle::RetroLogLevel::RETRO_LOG_ERROR;

use crate::{constants::EVENT_THREAD_SLEEP_TIME, devices_manager::DevicesManager};
use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

pub struct EventThread {
    event_thread_can_run: Arc<Mutex<bool>>,
}

impl EventThread {
    pub fn new() -> Self {
        EventThread {
            event_thread_can_run: Arc::new(Mutex::new(false)),
        }
    }

    pub fn stop(&mut self) {
        match self.event_thread_can_run.lock() {
            Ok(mut event_thread_can_run) => {
                *event_thread_can_run = false;
            }
            Err(poison) => {
                *poison.into_inner() = false;
            }
        }
    }

    pub fn resume(&mut self, devices: Arc<Mutex<DevicesManager>>) -> Result<(), ErroHandle> {
        if let Err(_need_try_again) = self.try_enable_thread() {
            if self.try_enable_thread().is_err() {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: "Não foi possível iniciar a thread de eventos do gamepad".to_string(),
                });
            }
        }

        self.create_update_devices_state_thread(devices, self.event_thread_can_run.clone());

        Ok(())
    }

    fn try_enable_thread(&self) -> Result<(), bool> {
        let mut need_try_again = false;

        match self.event_thread_can_run.lock() {
            Ok(mut event_thread_can_run) => {
                if !(*event_thread_can_run) {
                    *event_thread_can_run = true;
                }
            }
            Err(poison) => {
                let mut _is_enable = *poison.into_inner();

                if _is_enable {
                    _is_enable = false;
                    need_try_again = true;
                } else {
                    _is_enable = true;
                }
            }
        }

        if need_try_again {
            // A thread gamepad_listener precisa de um tempo para ler o mutex novamente
            sleep(Duration::from_millis(EVENT_THREAD_SLEEP_TIME));
            return Err(need_try_again);
        }

        Ok(())
    }

    /// # gamepad event listener thread
    ///
    /// Isso é util se quando não há uma *rom* em execução, mas ainda é necessário ouvir os eventos do
    /// gamepad. Por exemplo, a *rom* foi fechada, mas a interface do usuário ainda precisa ser
    /// notificada sobre os eventos do gamepad.
    ///
    /// Aviso: para evitar uso desnecessário de CPU use isso somente quando não hover uma
    /// *rom* em execução!. Use o terceiro parâmetro "event_thread_is_enabled" para encerar a
    /// execução da thread quando não precisar mais dela.
    fn create_update_devices_state_thread(
        &mut self,
        devices: Arc<Mutex<DevicesManager>>,
        event_thread_is_enabled: Arc<Mutex<bool>>,
    ) {
        thread::spawn(move || {
            while *event_thread_is_enabled.lock().unwrap_or_else(|poison| {
                let mut can_run = poison.into_inner();
                *can_run = false;
                can_run
            }) {
                //WITHOUT THIS, WI HAVE A HIGH CPU UTILIZATION!
                sleep(Duration::from_millis(EVENT_THREAD_SLEEP_TIME));

                if let Ok(devices) = &mut devices.lock() {
                    devices.update_state();
                }
            }
        });
    }
}
