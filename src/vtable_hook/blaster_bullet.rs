use crate::imports::*;
use std::arch::asm;

#[skyline::hook(offset = 0xa633f4, inline)]
unsafe extern "C" fn blaster_bullet_generate_angle(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[20].x.as_ref() as *mut BattleObjectModuleAccessor;

    let rot = PostureModule::rot_z(module_accessor, 0);
    let motion = MotionModule::motion_kind(module_accessor);
    let situation = StatusModule::situation_kind(module_accessor);
    let new_angle = if situation == *SITUATION_KIND_GROUND {
        if [hash40("special_n_hi"), hash40("wolf_special_n_hi")].contains(&motion) {
            15.0
        }
        else if [hash40("special_n_lw"), hash40("wolf_special_n_lw")].contains(&motion) {
            -25.0
        } else {
            0.0
        }
    } else {
        if [hash40("special_air_n_hi"), hash40("wolf_special_air_n_hi")].contains(&motion) {
            15.0
        }
        else if [hash40("special_air_n_lw"), hash40("wolf_special_air_n_lw")].contains(&motion) {
            -45.0
        } else {
            0.0
        }
    };

    let lr = PostureModule::lr(module_accessor);
    let final_angle = rot + new_angle * lr;
    asm!("fmov s0, w8", in("w8") final_angle);
}

pub fn install() {
    // Patches offsetting the ecb
    let _ = skyline::patching::Patch::in_text(0x33faaec).data(0x1e2703e0);

    skyline::install_hooks!(
        blaster_bullet_generate_angle
    );
}