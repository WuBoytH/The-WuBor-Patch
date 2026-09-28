use crate::offsets;

#[skyline::from_offset(offsets::system::music::MUSIC_FUNCTION_1)]
unsafe fn music_function1(arg: u64);

#[skyline::from_offset(offsets::system::music::MUSIC_FUNCTION_2)]
unsafe fn music_function2(arg: u64, arg2: u64);

#[skyline::hook(offset = offsets::system::music::TRAINING_RESET_MUSIC_2, inline)]
unsafe fn training_reset_music2(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function2(ctx.registers[0].x(), ctx.registers[1].x());
    }
}

#[skyline::hook(offset = offsets::system::music::TRAINING_RESET_MUSIC_1, inline)]
unsafe fn training_reset_music1(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function1(ctx.registers[0].x());
    }
}

pub fn install() {
    unsafe {
        skyline::patching::Patch::in_text(offsets::system::music::TRAINING_RESET_MUSIC_2).nop().unwrap();
        skyline::patching::Patch::in_text(offsets::system::music::TRAINING_RESET_MUSIC_1).nop().unwrap();
        skyline::install_hooks!(
            training_reset_music2,
            training_reset_music1
        );
    }
}
