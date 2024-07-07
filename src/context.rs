use crate::{
    handle_event::{gamepad_events_handle, GamePadState, GamepadStateListener},
    retro_gamepad::RetroGamePad,
    gamepad_event_listener_thread::create_gamepad_listener_thread,
};
use gilrs::Gilrs;
use retro_ab::retro_sys::{retro_rumble_effect, RETRO_DEVICE_ID_JOYPAD_MASK};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use retro_ab::erro_handle::ErroHandle;
use retro_ab::erro_handle::RetroLogLevel::RETRO_LOG_ERROR;
use crate::constants::GAMEPAD_EVENT_THREAD_SLEEP_TIME;

lazy_static! {
    static ref GAMEPADS: Arc<Mutex<Vec<RetroGamePad>>> = Arc::new(Mutex::new(Vec::new()));
    static ref GILRS_INSTANCE: Arc<Mutex<Gilrs>> = Arc::new(Mutex::new(Gilrs::new().unwrap()));
    static ref MAX_PORTS: Arc<Mutex<usize>> = Arc::new(Mutex::new(2));
    static ref LISTENER: Arc<Mutex<GamepadStateListener>> = Arc::new(Mutex::new(none));
}

fn none(_gs: GamePadState, _rg: RetroGamePad) {}

pub struct GamepadContext {
    event_thread_can_run: Arc<Mutex<bool>>,
}

impl Drop for GamepadContext {
    fn drop(&mut self) {
        GAMEPADS.lock().unwrap().clear();
        *self.event_thread_can_run.lock().unwrap() = false;
    }
}

impl GamepadContext {
    pub fn new(listener: Option<GamepadStateListener>) -> GamepadContext {
        let event_thread_is_enabled = Arc::new(Mutex::new(true));

        if let Some(listener) = listener {
            *LISTENER.lock().unwrap() = listener;
        }

        create_gamepad_listener_thread(
            GAMEPADS.clone(),
            GILRS_INSTANCE.clone(),
            event_thread_is_enabled.clone(),
            MAX_PORTS.clone(),
            LISTENER.clone(),
        );

        Self {
            event_thread_can_run: event_thread_is_enabled,
        }
    }

    #[doc = "retorna uma lista de gamepad disponíveis"]
    pub fn get_list(&self) -> Arc<Mutex<Vec<RetroGamePad>>> {
        GAMEPADS.clone()
    }

    #[doc = "Para que o CORE possa 'tomar posse' com existo dos eventos do gamepad é necessário interromper o a thread de eventos"]
    pub fn stop_thread_events(&mut self) {
        match self.event_thread_can_run.lock() {
            Ok(mut event_thread_can_run) => {
                *event_thread_can_run = false;
            }
            Err(poison) => {
                *poison.into_inner() = false;
            }
        }
    }

    #[doc = "Devolve a 'posse' dos eventos do gamepad dada ao CORE para a thread de eventos. chame isso quando nao houve nenhuma rom em execução"]
    pub fn resume_thread_events(&mut self) -> Result<(), ErroHandle> {
        if let Err(_need_try_again) = self.try_enable_thread() {
            if self.try_enable_thread().is_err() {
                return Err(ErroHandle {
                    level: RETRO_LOG_ERROR,
                    message: "Não foi possível iniciar a thread de eventos do gamepad".to_string(),
                })
            }
        }

        create_gamepad_listener_thread(
            GAMEPADS.clone(),
            GILRS_INSTANCE.clone(),
            self.event_thread_can_run.clone(),
            MAX_PORTS.clone(),
            LISTENER.clone(),
        );

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
            sleep(Duration::from_millis(GAMEPAD_EVENT_THREAD_SLEEP_TIME));
            return Err(need_try_again);
        }

        Ok(())
    }
}

//***********ENVIE ESSAS CALLBACKS PARA CORE****************/
pub fn input_poll_callback() {
    gamepad_events_handle(
        GILRS_INSTANCE.clone(),
        GAMEPADS.clone(),
        MAX_PORTS.clone(),
        LISTENER.clone(),
    );
}

pub fn input_state_callback(port: i16, _device: i16, _index: i16, id: i16) -> i16 {
    for gamepad_info in &*GAMEPADS.lock().unwrap() {
        if gamepad_info.retro_port == port {
            return if id as u32 != RETRO_DEVICE_ID_JOYPAD_MASK {
                let pressed = gamepad_info.key_pressed(id);

                if pressed {
                    1
                } else {
                    0
                }
            } else {
                gamepad_info.retro_bitmask() as i16
            }
        }
    }

    0
}

pub fn rumble_callback(
    port: std::os::raw::c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    println!(
        "port:{:?} effect:{:?} strength:{:?}",
        port, effect, strength
    );
    true
}
//****************************************************/
