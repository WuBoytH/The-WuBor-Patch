use super::*;

unsafe extern "C" fn sonic_special_air_lw_loop_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
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
            *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
        ) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_loop_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    sv_kinetic_energy!(
        set_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -1.2
    );
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    sv_kinetic_energy!(
        set_accel,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        -air_accel_y * 2.5
    );
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable * 3.0
    );
    sv_kinetic_energy!(
        set_limit_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
        air_speed_y_stable * 3.0
    );
    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::fighter::instance::flag::DISABLE_SPECIAL_LW);
    fighter.sub_shift_status_main(L2CValue::Ptr(sonic_special_air_lw_loop_main_loop as *const () as _))
}

unsafe extern "C" fn sonic_special_air_lw_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }

    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent).abs();
        VarModule::set_float(fighter.module_accessor, vars::sonic::status::float::SPECIAL_AIR_LW_BOUND_SPEED_X, speed_x);
        VarModule::set_float(fighter.module_accessor, vars::sonic::status::float::SPECIAL_AIR_LW_BOUND_SPEED_Y, speed_y);
        fighter.change_status(vars::sonic::status::SPECIAL_AIR_LW_BOUND.into(), false.into());
        return 1.into();
    }

    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw_2_loop"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES.into());
    }
    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_check_attack(
    fighter: &mut L2CFighterCommon,
    _param_1: &L2CValue,
    _param_2: &L2CValue
) -> L2CValue {
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
    let speed_y = 2.4;
    VarModule::set_float(fighter.module_accessor, vars::sonic::status::float::SPECIAL_AIR_LW_BOUND_SPEED_X, speed_x);
    VarModule::set_float(fighter.module_accessor, vars::sonic::status::float::SPECIAL_AIR_LW_BOUND_SPEED_Y, speed_y);
    VarModule::on_flag(fighter.module_accessor, vars::sonic::status::flag::SPECIAL_AIR_LW_HIT);
    fighter.change_status(vars::sonic::status::SPECIAL_AIR_LW_BOUND.into(), false.into());
    0.into()
}

unsafe extern "C" fn sonic_special_air_lw_loop_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sonic_spinblur_max"),
        true,
        true
    );
    effect!(
        fighter,
        MA_MSC_CMD_EFFECT_EFFECT_OFF_KIND,
        Hash40::new("sonic_spintrace_max"),
        true,
        true
    );
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Pre, vars::sonic::status::SPECIAL_AIR_LW_LOOP, sonic_special_air_lw_loop_pre);
    agent.status(Init, vars::sonic::status::SPECIAL_AIR_LW_LOOP, sonic_special_air_lw_loop_init);
    agent.status(Main, vars::sonic::status::SPECIAL_AIR_LW_LOOP, sonic_special_air_lw_loop_main);
    agent.status(CheckAttack, vars::sonic::status::SPECIAL_AIR_LW_LOOP, sonic_special_air_lw_check_attack);
    agent.status(End, vars::sonic::status::SPECIAL_AIR_LW_LOOP, sonic_special_air_lw_loop_end);
}