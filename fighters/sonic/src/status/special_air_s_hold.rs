use super::*;

unsafe extern "C" fn sonic_special_air_s_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn sonic_special_air_s_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
    VarModule::set_int(fighter.module_accessor, vars::sonic::status::int::SPECIAL_AIR_S_HOLD_COUNT_REMAIN, 15);
    VarModule::on_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_FIRST);
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
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    let lr = PostureModule::lr(fighter.module_accessor);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        1.2 * lr,
        0.0
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0.01
    );
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        0.01
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        0.8
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_air_s_hold_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_air_s_hold_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }

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
        let count_down = VarModule::countdown_int(fighter.module_accessor, vars::sonic::status::int::SPECIAL_AIR_S_HOLD_COUNT_REMAIN, 0);
        if VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_TO_END) {
            fighter.change_status(vars::sonic::status::SPECIAL_AIR_S_END.into(), false.into());
            return 1.into();
        }
        if !VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_S_FIRST)
        && (ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
        || count_down) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            return 0.into();
        }
    }
    
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::SPECIAL_AIR_S_HOLD, sonic_special_air_s_hold_pre);
    agent.status(Main, vars::sonic::status::SPECIAL_AIR_S_HOLD, sonic_special_air_s_hold_main);
}