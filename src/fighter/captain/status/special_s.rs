use crate::imports::status_imports::*;
use super::helper::*;

#[status_script(agent = "captain", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe extern "C" fn captain_special_s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT_CHECK_ONOFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_CLIFF_FALL_ONOFF);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_GRAVITY_ONOFF);
    let speed_coef = 1.0;
    WorkModule::set_float(fighter.module_accessor, speed_coef, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::set_int(
            fighter.module_accessor,
            *SITUATION_KIND_AIR,
            *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KNUCKLE_START_SITUATION
        );
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_s_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        captain_set_air(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        if !StopModule::is_stop(fighter.module_accessor) {
            captain_special_s_air_substatus(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(captain_special_s_air_substatus as *const () as _));
    }
    else {
        WorkModule::set_int(
            fighter.module_accessor,
            *SITUATION_KIND_GROUND,
            *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KNUCKLE_START_SITUATION
        );
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_s_start"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        captain_set_ground(fighter);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        if !StopModule::is_stop(fighter.module_accessor) {
            captain_special_s_ground_substatus(fighter, false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(captain_special_s_ground_substatus as *const () as _));
    }
    GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_CAPTAIN_CLIFF_HANG_DATA_SPECIAL_S as u32);
    fighter.sub_shift_status_main(L2CValue::Ptr(captain_special_s_main_loop as *const () as _))
}

unsafe extern "C" fn captain_special_s_ground_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_CLIFF_FALL_ONOFF) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    0.into()
}

unsafe extern "C" fn captain_special_s_air_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_GRAVITY_ONOFF) {
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
        else {
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    0.into()
}

unsafe extern "C" fn captain_special_s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let start_sit = WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KNUCKLE_START_SITUATION);
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    if start_sit != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
        if sit == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into();
        }
        if sit != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 1.into();
        }
    }
    let manual_input = VarModule::is_flag(fighter.module_accessor, captain::status::flag::SPECIAL_S_ENABLE_MANUAL_ATTACK)
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_KNUCKLE_HIT)
    || manual_input {
        fighter.change_status(FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END.into(), manual_input.into());
        return 1.into();
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_SPECIAL_S, captain_special_s_main);
}