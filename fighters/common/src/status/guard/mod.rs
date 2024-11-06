use super::*;

pub unsafe extern "C" fn add_shield_health(fighter: &mut L2CFighterCommon, percent: f32) {
    let shield = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    // println!("shield health: {}", shield);
    let shield_max = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
    let new_health = (shield + (shield_max * percent)).clamp(0.0, shield_max);
    // println!("new shield health: {}", new_health);
    WorkModule::set_float(fighter.module_accessor, new_health, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
}

mod guard_on;
mod guard;
mod guard_off;
mod guard_damage;

mod guard_cancel;

mod shield_break_fly;

pub fn install() {
    guard_on::install();
    guard::install();
    guard_off::install();
    guard_damage::install();

    guard_cancel::install();

    shield_break_fly::install();
}