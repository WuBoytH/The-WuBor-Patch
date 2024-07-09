use super::*;

unsafe extern "C" fn sonic_special_air_s_end_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
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

unsafe extern "C" fn sonic_special_air_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
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
        0.001
    );
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        0.001
    );
    sv_kinetic_energy!(
        mul_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        0.0,
        0.0
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_air_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_air_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_s_launch") {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_s_launch") {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
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
        let count_down = VarModule::countdown_int(fighter.module_accessor, vars::sonic::status::int::SPECIAL_AIR_S_HOLD_COUNT_REMAIN, 0);
        if count_down {
            if VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_CHECK_HIT) {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("special_air_s_launch"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return 0.into();
            }
        }
    }
    
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::SPECIAL_AIR_S_END, sonic_special_air_s_end_pre);
    agent.status(Main, vars::sonic::status::SPECIAL_AIR_S_END, sonic_special_air_s_end_main);
}