use super::*;

unsafe extern "C" fn sonic_special_air_s_hit_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        (
            *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn sonic_special_air_s_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::set_int(fighter.module_accessor, vars::sonic::status::int::SPECIAL_AIR_S_HOLD_COUNT_REMAIN, 7);
    sv_kinetic_energy!(
        reset_energy,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        ENERGY_GRAVITY_RESET_TYPE_GRAVITY,
        0.0,
        0.0,
        0.0,
        0.0,
        0.0
    );
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.02
    );
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        0.02
    );
    sv_kinetic_energy!(
        mul_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.2,
        0.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.8,
        0.0
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_air_s_hit_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_air_s_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s_hold"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        VarModule::off_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_FIRST);
    }

    if VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_CHECK_END) {
        VarModule::off_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_CHECK_END);
        if VarModule::countdown_int(fighter.module_accessor, vars::sonic::status::int::SPECIAL_AIR_S_HOLD_COUNT_REMAIN, 0) {
            if VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_CHECK_HIT) {
                VarModule::off_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_CHECK_HIT);
                fighter.change_status(vars::sonic::status::SPECIAL_AIR_S_LAUNCH.into(), false.into());
            }
        }
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::SPECIAL_AIR_S_HIT, sonic_special_air_s_hit_pre);
    agent.status(Main, vars::sonic::status::SPECIAL_AIR_S_HIT, sonic_special_air_s_hit_main);
}
