use super::*;

unsafe extern "C" fn free_guard_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("free_guard"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(free_guard_fastshift as *const () as _))
}

unsafe extern "C" fn free_guard_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let parent_status = WorkModule::get_int(weapon.module_accessor, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_INT_PARENT_STATUS_KIND);

    if parent_status == vars::fighter::status::GUARD_CANCEL_ATTACK {
        weapon.change_status(vars::rosetta_tico::status::GUARD_CANCEL_ATTACK.into(), false.into());
        return 1.into();
    }

    let func = smashline::api::get_target_function("lua2cpp_rosetta.nrs", 0x2d130).unwrap();
    let func : fn(&mut L2CValue, &mut L2CWeaponCommon, L2CValue, L2CValue, L2CValue, L2CValue) = std::mem::transmute(func);
    let ret = &mut L2CValue::Bool(false);
    func(ret, weapon, false.into(), true.into(), false.into(), false.into());
    if ret.get_bool() {
        return 1.into();
    }

    if ![
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE
    ].contains(&parent_status) {
        weapon.change_status(WEAPON_ROSETTA_TICO_STATUS_KIND_FREE_WAIT.into(), false.into());
        return 1.into();
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ROSETTA_TICO_STATUS_KIND_FREE_GUARD, free_guard_main);
}