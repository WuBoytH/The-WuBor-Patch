use crate::imports::*;
use std::arch::asm;

unsafe extern "C" fn check_wolf(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    let object_id = (*module_accessor).battle_object_id;
    let object = MiscModule::get_battle_object_from_id(object_id);
    let kind = (*object).kind as i32;
    kind == *FIGHTER_KIND_WOLF
}

#[skyline::hook(offset = 0xa633f4, inline)]
unsafe extern "C" fn blaster_bullet_generate_angle(ctx: &mut skyline::hooks::InlineCtx) {
    let module_accessor = *ctx.registers[20].x.as_ref() as *mut BattleObjectModuleAccessor;
    if !check_wolf(module_accessor) {
        return;
    }

    let rot = PostureModule::rot_z(module_accessor, 0);
    let motion = MotionModule::motion_kind(module_accessor);
    let situation = StatusModule::situation_kind(module_accessor);
    let new_angle = if situation == *SITUATION_KIND_GROUND {
        if motion == hash40("special_n_hi") {
            15.0
        }
        else if motion == hash40("special_n_lw") {
            -25.0
        } else {
            0.0
        }
    } else {
        if motion == hash40("special_air_n_hi") {
            15.0
        }
        else if motion == hash40("special_air_n_lw") {
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
    skyline::install_hooks!(
        blaster_bullet_generate_angle
    );
}