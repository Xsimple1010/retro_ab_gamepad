extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;
use retro_ab::{
    core::{self, RetroContext, RetroEnvCallbacks},
    test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, context::RetroAvCtx,
    video_refresh_callback, Event, Keycode,
};
use retro_ab_gamepad::{
    context::{input_poll_callback, input_state_callback, rumble_callback, GamepadContext},
    retro_gamepad::RetroGamePad,
    GamePadState,
};
use std::{ptr::addr_of, sync::Arc};

static mut CORE_CTX: Option<Arc<RetroContext>> = None;

fn state_listener(state: GamePadState, gamepad: RetroGamePad) {
    match state {
        GamePadState::Connected => unsafe {
            println!("{:?}", gamepad.name);
            if let Some(ctx) = &*addr_of!(CORE_CTX) {
                let _ =
                    core::connect_controller(ctx, gamepad.retro_port as u32, gamepad.retro_type);
            }
        },
        GamePadState::Disconnected => {}
        GamePadState::ButtonPressed(b) => println!("{:?}", b),
    }
}

fn main() {
    let core_ctx = core::load(
        "./cores/snes9x_libretro.dll",
        test_tools::paths::get_paths(),
        RetroEnvCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
            rumble_callback,
        },
    )
    .expect("Erro ao tentar criar RetroContext");

    unsafe {
        CORE_CTX = Some(core_ctx);

        if let Some(core_ctx) = &*addr_of!(CORE_CTX) {
            core::init(core_ctx).expect("Erro ao tentar inicializar o contexto");
            core::load_game(core_ctx, "./roms/Mega Man X (E).smc")
                .expect("Erro ao tentar carrega a rom");

            let mut gamepad_ctx = GamepadContext::new(Some(state_listener));

            gamepad_ctx.stop_thread_events();

            let (mut av_ctx, mut event_pump) =
                RetroAvCtx::new(Arc::clone(&core_ctx.core.av_info)).expect("erro");

            'running: loop {
                core::run(core_ctx).expect("msg");
                av_ctx.get_new_frame().expect("");

                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'running,
                        _ => {}
                    }
                }
            }

            gamepad_ctx.resume_thread_events();
            let _ = core::de_init(core_ctx.to_owned());
        }
    }
}
