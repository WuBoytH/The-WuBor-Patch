use super::*;

unsafe extern "C" fn special_air_lw_fall_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ALWAYS),
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn special_air_lw_fall_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticUtility::clear_unable_energy(*FIGHTER_KINETIC_ENERGY_ID_CONTROL, fighter.module_accessor);

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
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        vl::param_special_lw::ground_pound_fall_speed
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        vl::param_special_lw::ground_pound_fall_speed
    );
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -vl::param_special_lw::ground_pound_fall_speed
    );
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -0.2
    );

    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    0.into()
}

unsafe extern "C" fn special_air_lw_fall_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_lw_fall"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(special_air_lw_fall_main_loop as *const () as _))
}

unsafe extern "C" fn special_air_lw_fall_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(vars::mario::status::SPECIAL_AIR_LW_LANDING.into(), false.into());
        return 1.into();
    }

    if fighter.global_table[STICK_Y].get_f32() >= 0.5 {
        fighter.change_status(vars::mario::status::SPECIAL_AIR_LW_CANCEL.into(), false.into());
        return 1.into();
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::mario::status::SPECIAL_AIR_LW_FALL, special_air_lw_fall_pre);
    agent.status(Init, vars::mario::status::SPECIAL_AIR_LW_FALL, special_air_lw_fall_init);
    agent.status(Main, vars::mario::status::SPECIAL_AIR_LW_FALL, special_air_lw_fall_main);
}