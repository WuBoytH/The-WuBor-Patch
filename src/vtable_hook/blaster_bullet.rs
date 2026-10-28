use wubor_utils::offsets;

extern "C" {
    #[link_name = "blaster_bullet_generate_angle_inline"]
    fn blaster_bullet_generate_angle_inline(ctx: &mut skyline::hooks::InlineCtx);
}

#[skyline::hook(offset = offsets::weapon::blaster_bullet::GENERATE_ANGLE, inline)]
unsafe extern "C" fn blaster_bullet_generate_angle(ctx: &mut skyline::hooks::InlineCtx) {
    blaster_bullet_generate_angle_inline(ctx);
}

pub fn install() {
    skyline::install_hooks!(
        blaster_bullet_generate_angle
    );
}
