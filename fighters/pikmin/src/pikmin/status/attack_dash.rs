use super::*;

unsafe extern "C" fn attack_dash_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        // *WEAPON_KINETIC_TYPE_PIKMIN_PIKMIN_MOTION,
        *WEAPON_KINETIC_TYPE_PIKMIN_PIKMIN_ACTION_COMP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(0),
        false,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *WEAPON_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    0.into()
}

unsafe extern "C" fn attack_dash_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(
        weapon.module_accessor,
        Hash40::new("attack_dash"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    weapon.fastshift(L2CValue::Ptr(attack_dash_fastshift as *const () as _))
}

unsafe extern "C" fn attack_dash_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    // if weapon.global_table[STATUS_FRAME].get_f32() != 0.0 {
    //     KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_PIKMIN_PIKMIN_MOTION);
    // }

    if MotionModule::is_end(weapon.module_accessor) {
        weapon.change_status(WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::pikmin_pikmin::status::ATTACK_DASH, attack_dash_pre);
    agent.status(Main, vars::pikmin_pikmin::status::ATTACK_DASH, attack_dash_main);
}