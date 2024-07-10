use super::*;

unsafe extern "C" fn sonic_special_air_lw_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW |
            *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON
        ) as u64,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_start_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        1.7
    );
    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_LW);
    
    let jump_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    VarModule::set_int(fighter.module_accessor, vars::sonic::status::int::SPECIAL_AIR_LW_JUMP_RESERVE, jump_count);

    let escape_air = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR);
    VarModule::set_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_ESCAPE_AIR_RESERVE, escape_air);

    let special_n = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SONIC_INSTANCE_WORK_FLAG_SPECIAL_N_FALL);
    VarModule::set_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_SPECIAL_N_FALL_RESERVE, special_n);

    let special_s = VarModule::is_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_S);
    VarModule::set_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_SPECIAL_S_RESERVE, special_s);

    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_lw_2_start"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_air_lw_start_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_air_lw_start_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_TO_LOOP) {
        VarModule::off_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_TO_LOOP);
        fighter.change_status(vars::sonic::status::SPECIAL_AIR_LW_LOOP.into(), false.into());
    }
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::SPECIAL_AIR_LW_START, sonic_special_air_lw_start_pre);
    agent.status(Init, vars::sonic::status::SPECIAL_AIR_LW_START, sonic_special_air_lw_start_init);
    agent.status(Main, vars::sonic::status::SPECIAL_AIR_LW_START, sonic_special_air_lw_start_main);
}