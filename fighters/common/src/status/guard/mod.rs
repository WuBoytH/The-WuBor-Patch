use super::*;

pub unsafe extern "C" fn init_shield_hurtbox(fighter: &mut L2CFighterCommon) {
    FighterUtil::set_hit_data(
        fighter.module_accessor,
        0,
        0,
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        &Vector3f{x: 0.0, y: 0.0, z: 0.0},
        0.9,
        Hash40::new("throw"),
        CollisionPart(*COLLISION_PART_BODY),
        HitHeight(*HIT_HEIGHT_CENTER),
        HitStatus(*HIT_STATUS_NORMAL),
        CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE)
    );
}

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

pub mod guard_cancel;

pub mod guard_crush;

mod shield_break_fly;

pub fn install() {
    guard_on::install();
    guard::install();
    guard_off::install();
    guard_damage::install();

    shield_break_fly::install();
}