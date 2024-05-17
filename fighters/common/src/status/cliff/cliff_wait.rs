use super::*;

#[skyline::hook(replace = L2CFighterCommon_get_cliff_wait_hit_xlu_frame)]
unsafe extern "C" fn get_cliff_wait_hit_xlu_frame(fighter: &mut L2CFighterCommon) -> L2CValue {
    let air_xlu_max_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_air_xlu_max_frame")
    ) as f32;
    let air_xlu_max_air_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_air_xlu_max_air_frame")
    ) as f32;
    let damage_xlu_max_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_damage_xlu_max_frame")
    ) as f32;
    let damage_xlu_max_damage = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_damage_xlu_max_damage")
    ) as f32;
    let xlu_min_frame = WorkModule::get_param_int(
        fighter.module_accessor, hash40("common"), hash40("cliff_wait_xlu_min_frame")
    ) as f32;

    // Vanilla Ultimate's Ledge Intangibility Formula
    let air_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_FRAME_IN_AIR) as f32;
    let xlu_from_air = (air_xlu_max_frame * (air_frame / air_xlu_max_air_frame)).clamp(0.0, air_xlu_max_frame);
    // println!("xlu from air: {}", xlu_from_air);
    let damage = DamageModule::damage(fighter.module_accessor, 0);
    let xlu_from_damage = (damage_xlu_max_frame - ((damage / damage_xlu_max_damage) * damage_xlu_max_frame)).clamp(0.0, damage_xlu_max_frame);
    // println!("xlu from damage: {}", xlu_from_damage);
    let xlu_frame = xlu_from_air + xlu_from_damage;
    let cliff_hang_invincible_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("cliff_hang_invincible_frame"), 0);
    // println!("cliff hang invincible frame? {}", cliff_hang_invincible_frame);
    // println!("final xlu: {}", (xlu_frame * cliff_hang_invincible_frame).clamp(xlu_min_frame, f32::MAX));
    ((xlu_frame * cliff_hang_invincible_frame).clamp(xlu_min_frame, f32::MAX)).into()
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            get_cliff_wait_hit_xlu_frame
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}