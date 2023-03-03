use crate::imports::status_imports::*;

#[status_script(agent = "marth", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, marth::status::flag::DISABLE_STANCE_CHANGE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    marth_specials_reset_helper(fighter);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        marth_specials_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(marth_specials_substatus as *const () as _));
    WorkModule::set_int64(
        fighter.module_accessor,
        hash40("special_s1") as i64,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
    );
    WorkModule::set_int64(
        fighter.module_accessor,
        hash40("special_air_s1") as i64,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_specials_main_loop as *const () as _))
}

unsafe extern "C" fn marth_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            marth_specials_mot_helper(fighter);
        }
    }
    else {
        marth_specials_mot_helper(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    marth_specials_status_change_helper(fighter);
    0.into()
}

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_specials2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    marth_specials_reset_helper(fighter);
    ControlModule::reset_trigger(fighter.module_accessor);
    WorkModule::set_int(
        fighter.module_accessor,
        *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3,
        *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        marth_specials_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(marth_specials_substatus as *const () as _));
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s3_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s3_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s2_hi") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s2_hi") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    else {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s2_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s2_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW);
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_specials_main_loop as *const () as _))
}

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_specials3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s4_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s4_lw") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI) {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s4_hi") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s4_hi") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    else {
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_s4_s") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND
        );
        WorkModule::set_int64(
            fighter.module_accessor,
            hash40("special_air_s4_s") as i64,
            *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR
        );
    }
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW);
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_specials3_main_loop as *const () as _))
}

unsafe extern "C" fn marth_specials3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            marth_specials_mot_helper(fighter);
        }
    }
    else {
        marth_specials_mot_helper(fighter);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if sit != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S3, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STOP)]
unsafe fn marth_specials3_exec_stop(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn marth_specials_substatus(fighter: &mut L2CFighterCommon, _param_1: L2CValue) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_FAILURE) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS) {
            return 0.into();
        }
        if !ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            return 0.into();
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_FAILURE);
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS);
            let enable_hi_lw = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("enable_input_hi_lw"));
            if enable_hi_lw == 0 {
                return 0.into();
            }
            let stick_y = fighter.global_table[STICK_Y].get_f32();
            let squat_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("squat_stick_y"));
            if stick_y > -squat_stick_y {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI);
            }
            else if stick_y < squat_stick_y {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_LW);
            }
        }
    }
    0.into()
}

unsafe extern "C" fn marth_specials_reset_helper(fighter: &mut L2CFighterCommon) {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_FAILURE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
}

unsafe extern "C" fn marth_specials_mot_helper(fighter: &mut L2CFighterCommon) {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let correct;
    let mot;
    if sit != *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND_AIR);
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_MOTION_KIND);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT) {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    }
    else {
        MotionModule::change_motion_inherit_frame(
            fighter.module_accessor,
            Hash40::new_raw(mot),
            -1.0,
            1.0,
            0.0,
            false,
            false
        );
    }
}

unsafe extern "C" fn marth_specials_status_change_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_SUCCESS) {
        if !MotionModule::is_end(fighter.module_accessor) {
            return 0.into();
        }
        let sit = fighter.global_table[SITUATION_KIND].get_i32();
        if sit != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        return 0.into();
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
            if !MotionModule::is_end(fighter.module_accessor) {
                return 0.into();
            }
            let sit = fighter.global_table[SITUATION_KIND].get_i32();
            if sit != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            return 0.into();
        }
        let status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_WORK_INT_CHANGE_STATUS);
        fighter.change_status(status.into(), false.into());
    }
    1.into()
}

#[status_script(agent = "marth", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn marth_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let attack_type;
    let power_up;
    if !VarModule::is_flag(fighter.battle_object, marth::instance::flag::IS_STANCE) {
        attack_type = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64;
        power_up = *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32
    }
    else {
        attack_type = *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64;
        power_up = *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32;
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES),
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
        attack_type,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        power_up,
        0
    );
    0.into()
}

#[status_script(agent = "marth", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn marth_specialhi_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, marth::instance::flag::IS_STANCE) {
        original!(fighter);
    }
    0.into()
}

#[status_script(agent = "marth", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.battle_object, marth::status::flag::DISABLE_STANCE_CHANGE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    let is_stance = VarModule::is_flag(fighter.battle_object, marth::instance::flag::IS_STANCE);
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if is_stance {
            mot = Hash40::new("special_lw_special_hi");
        }
        else {
            mot = Hash40::new("special_hi");
        }
    }
    else {
        if is_stance {
            mot = Hash40::new("special_lw_special_hi");
        }
        else {
            mot = Hash40::new("special_air_hi");
        }
    }
    MotionModule::change_motion(
        fighter.module_accessor,
        mot,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        marth_specialhi_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(marth_specialhi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn marth_specialhi_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !VarModule::is_flag(fighter.battle_object, marth::instance::flag::IS_STANCE) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                fighter.set_situation(SITUATION_KIND_AIR.into());
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE_SET_ANGLE);
            }
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            fighter.set_situation(SITUATION_KIND_AIR.into());
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_KINETIC_CHANGE);
        }
    }
    0.into()
}

unsafe extern "C" fn marth_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL)
        && MotionModule::frame(fighter.module_accessor) >= 20.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "marth", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn marth_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_LANDING_CANCEL);
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
        let fall_x_mul_value = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul_value"));
        WorkModule::set_float(fighter.module_accessor, fall_x_mul_value, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
    }
    VarModule::off_flag(fighter.battle_object, marth::instance::flag::IS_STANCE);
    0.into()
}

pub fn install() {
    install_status_scripts!(
        marth_specials_main,
        marth_specials2_main,
        marth_specials3_main,
        marth_specials3_exec_stop,
        marth_specialhi_pre, marth_specialhi_main, marth_specialhi_exec, marth_specialhi_end
    );
}