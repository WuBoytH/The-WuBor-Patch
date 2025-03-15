use super::*;

unsafe extern "C" fn follow_guard_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("follow_guard"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(follow_guard_fastshift as *const () as _))
}

unsafe extern "C" fn follow_guard_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let parent_status = WorkModule::get_int(weapon.module_accessor, *WEAPON_ROSETTA_TICO_INSTANCE_WORK_ID_INT_PARENT_STATUS_KIND);

    if parent_status == vars::fighter::status::GUARD_CANCEL_ATTACK {
        weapon.change_status(vars::rosetta_tico::status::GUARD_CANCEL_ATTACK.into(), false.into());
        return 1.into();
    }

    if ![
        *FIGHTER_STATUS_KIND_GUARD,
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE
    ].contains(&parent_status) {
        weapon.change_status(WEAPON_ROSETTA_TICO_STATUS_KIND_FOLLOW.into(), false.into());
        return 1.into();
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_ROSETTA_TICO_STATUS_KIND_FOLLOW_GUARD, follow_guard_main);
}