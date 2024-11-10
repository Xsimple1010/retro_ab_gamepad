extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;
use retro_ab::retro_context::RetroContext;
use retro_ab::{core::RetroEnvCallbacks, test_tools};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, context::RetroAvCtx,
    video_refresh_callback, Event, Keycode,
};
use retro_ab_gamepad::devices_manager::{Device, DeviceState};
use retro_ab_gamepad::RetroAbController;
use retro_ab_gamepad::{input_poll_callback, input_state_callback, rumble_callback};
use std::{ptr::addr_of, sync::Arc};

static mut CORE_CTX: Option<Arc<RetroContext>> = None;

fn state_listener(state: DeviceState, device: Device) {
    match state {
        DeviceState::Connected => unsafe {
            println!("Connected -> {:?}", device.name);
            if let Some(ctx) = &*addr_of!(CORE_CTX) {
                let _ = ctx
                    .core
                    .connect_controller(device.retro_port as u32, device.retro_type);
            }
        },
        DeviceState::Disconnected => println!("Disconnected -> {:?}", device.name),
        DeviceState::ButtonPressed(b) => println!("ButtonPressed -> {:?}", b),
    }
}

fn main() {
    let core_ctx = RetroContext::new(
        "C:/projetos/retro_ab_gamepad/cores/test.dll",
        test_tools::paths::get_paths().unwrap(),
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
            core_ctx
                .core
                .load_game("C:/projetos/retro_ab_gamepad/roms/Mega Man X (E).smc")
                .expect("Erro ao tentar carrega a rom");

            let mut gamepad_ctx = RetroAbController::new(Some(state_listener)).unwrap();

            gamepad_ctx.stop_thread_events();

            let (mut av_ctx, mut event_pump) =
                RetroAvCtx::new(Arc::clone(&core_ctx.core.av_info)).expect("erro");

            'running: loop {
                if av_ctx.sync() {
                    core_ctx.core.run().expect("msg");
                    av_ctx.get_new_frame();
                }

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

            let _ = gamepad_ctx.resume_thread_events();
            let _ = core_ctx.core.de_init();
        }
    }
}
