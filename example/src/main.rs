extern crate retro_ab;
extern crate retro_ab_av;
extern crate retro_ab_gamepad;
use retro_ab::{
    core::{self, RetroEnvCallbacks},
    retro_sys, test_tools,
};
use retro_ab_av::{
    audio_sample_batch_callback, audio_sample_callback, context::RetroAvCtx,
    video_refresh_callback, Event, Keycode,
};
use retro_ab_gamepad::{input_poll_callback, input_state_callback, GamePad};
use std::sync::Arc;

fn main() {
    let core_ctx = core::load(
        "C:/WFL/cores/test.dll",
        test_tools::paths::get_paths(),
        RetroEnvCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
        },
    )
    .expect("Erro ao tentar criar RetroContext: ");

    core::init(&core_ctx).expect("Erro ao tentar inicializar o contexto");
    core::load_game(&core_ctx, "C:/WFL/roms/sno.sfc").expect("Erro ao tentar carrega a rom");
    core::connect_controller(&core_ctx, 0, retro_sys::RETRO_DEVICE_JOYPAD);

    let mut gamepad = GamePad::new(core_ctx.core.system.ports.lock().unwrap().len());
    let gamepads = gamepad.search();

    for gm in gamepads {
        gamepad.vibrate(&gm.id);
    }

    // println!("{:?}", gamepads);

    // let (mut av_ctx, mut event_pump) =
    //     RetroAvCtx::new(Arc::clone(&core_ctx.core.av_info)).expect("erro");

    // 'running: loop {}

    // 'running: loop {
    //     core::run(&core_ctx).expect("msg");
    //     av_ctx.get_new_frame().expect("");

    //     for event in event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => break 'running,
    //             _ => {}
    //         }
    //     }
    // }

    // let _ = core::de_init(core_ctx);
    // retro_ab_av::de_init(av_ctx);
}
