use super::*;

unsafe extern "C" fn appeal_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *WEAPON_KINETIC_TYPE_PIKMIN_PIKMIN_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(0),
        true,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    0.into()
}

unsafe extern "C" fn appeal_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    weapon.fastshift(L2CValue::Ptr(appeal_fastshift as *const () as _))
}

unsafe extern "C" fn appeal_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::pikmin_pikmin::status::APPEAL, appeal_pre);
    agent.status(Main, vars::pikmin_pikmin::status::APPEAL, appeal_main);
}