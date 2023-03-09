#[skyline::from_offset(0x23ecb70)]
unsafe fn music_function1(arg: u64);

#[skyline::from_offset(0x23ed420)]
unsafe fn music_function2(arg: u64, arg2: u64);

#[skyline::hook(offset = 0x14f97bc, inline)]
unsafe fn training_reset_music2(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function2(*ctx.registers[0].x.as_ref(), *ctx.registers[1].x.as_ref());
    }
}

#[skyline::hook(offset = 0x1509dc4, inline)]
unsafe fn training_reset_music1(ctx: &skyline::hooks::InlineCtx) {
    if !smash::app::smashball::is_training_mode() {
        music_function1(*ctx.registers[0].x.as_ref());
    }
}

pub fn install() {
    skyline::patching::Patch::in_text(0x14f97bc).nop().unwrap();
    skyline::patching::Patch::in_text(0x1509dc4).nop().unwrap();
    skyline::install_hooks!(
        training_reset_music2,
        training_reset_music1
    );
}