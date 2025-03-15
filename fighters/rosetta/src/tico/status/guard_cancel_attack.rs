use super::*;

unsafe extern "C" fn guard_cancel_attack_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *WEAPON_KINETIC_TYPE_NONE,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        true,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    0.into()
}

unsafe extern "C" fn guard_cancel_attack_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    HitModule::set_status_all(weapon.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);

    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_FLAG_FREE) {
        GroundModule::set_passable_check(weapon.module_accessor, true);
    }

    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("guard_cancel_attack"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    WorkModule::set_int64(weapon.module_accessor, hash40("guard_cancel_attack") as i64, *WEAPON_ROSETTA_TICO_STATUS_ATTACK_COMMON_WORK_INT_PARENT_MOTION);

    weapon.fastshift(L2CValue::Ptr(guard_cancel_attack_fastshift as *const () as _))
}

unsafe extern "C" fn guard_cancel_attack_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let func = smashline::api::get_target_function("lua2cpp_rosetta.nrs", 0x3dbb0).unwrap();
    let func : fn(&mut L2CWeaponCommon) -> L2CValue = std::mem::transmute(func);
    func(weapon)
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::rosetta_tico::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_pre);
    agent.status(Main, vars::rosetta_tico::status::GUARD_CANCEL_ATTACK, guard_cancel_attack_main);
}