use crate::imports::status_imports::*;

unsafe extern "C" fn luigi_special_hi_drop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fall_max_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_max_x"));
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    let stable_speed_y = sv_kinetic_energy::get_stable_speed_y(fighter.lua_state_agent);
    sv_kinetic_energy!(
        set_stable_speed,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_CONTROL,
        fall_max_x,
        stable_speed_y
    );
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    sv_kinetic_energy!(
        controller_set_accel_x_mul,
        fighter,
        fall_x_mul
    );
    sv_kinetic_energy!(
        controller_set_accel_x_add,
        fighter,
        0
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_fall_common_uniq(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(luigi_special_hi_drop_substatus as *const () as _));
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_hi_drop"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if VarModule::is_flag(fighter.module_accessor, luigi::instance::flag::SPECIAL_HI_CANCEL) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(luigi_special_hi_drop_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_special_hi_drop_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    fighter.sub_fall_common_uniq(param_1)
}

unsafe extern "C" fn luigi_special_hi_drop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL.into(), false.into());
        }
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_LANDING_FALL.into(), false.into());
        }
    }
    0.into()
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP, luigi_special_hi_drop_main);
}