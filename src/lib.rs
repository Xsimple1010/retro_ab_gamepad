extern crate gilrs;

use std::time::Instant;

use gilrs::{Button, Event, GamepadId, Gilrs};

pub fn input_poll_callback() {}

pub fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
    println!("input_state_callback -> _port:{_port} device:{_device} index:{_index} id:{_id}");
    0
}

#[derive(Debug, Clone)]
pub struct GamePadInfo {
    pub id: GamepadId,
    pub name: String,
    #[doc = "indicar ao Core em qual porta o controle esta conectado, se o valor for -1 significa que todas as porta suportas pelo Core ja estão sendo usadas"]
    pub retro_port: i32,
    #[doc = "padrão RETRO_DEVICE_JOYPAD"]
    pub retro_type: u32,
}

pub struct GamePad {
    gilrs: Gilrs,
    gamepads: Vec<GamePadInfo>,
    max_ports: usize,
}

impl GamePad {
    pub fn search(&mut self) -> Vec<GamePadInfo> {
        let start = Instant::now();

        while !self.time_eq(start, 100) {
            while let Some(Event { id, event, time }) = self.gilrs.next_event() {
                // let mut current_retro_port = -1;

                // let ds = self.gamepads.sort_by(|g, gb| g.retro_port.partial_cmp(4));

                // if let Some(gp) = self.gilrs.connected_gamepad(id) {
                //     self.gamepads.push(GamePadInfo {
                //         id,
                //         name: gp.name().to_string(),
                //         retro_port: if self.used_ports < self.max_ports {
                //             0
                //         },
                //         retro_type: retro_ab::retro_sys::RETRO_DEVICE_JOYPAD,
                //     });
                // }
            }
        }

        self.gamepads.clone()
    }

    pub fn vibrate(&self, id: &GamepadId) {
        let gamepad = self.gilrs.connected_gamepad(id.clone());

        if let Some(gp) = gamepad {
            println!("state {:?}", gp.is_connected());
        }
    }

    fn time_eq(&self, time: Instant, end: u128) -> bool {
        (Instant::now() - time).as_millis() == end
    }

    pub fn new(max_ports: usize) -> GamePad {
        Self {
            gilrs: Gilrs::new().unwrap(),
            gamepads: Vec::new(),
            max_ports,
        }
    }
}
