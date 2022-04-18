use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    wubor_utils::table_const::*,
    custom_status::*,
    crate::fighter::common::common_status::attack::only_jabs,
    super::{vars::*, vl}
};

#[status_script(agent = "marth", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            if !(stick_y <= squat_stick_y) {
                if !(-stick_y <= squat_stick_y) {
                    return 0.into();
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_HI);
            }
            else {
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

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn marth_speciallw_hit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_hit_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_hit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            marth_speciallw_hit_mot_helper(fighter);
        }
    }
    let frame = fighter.global_table[MOTION_FRAME].get_f32();
    if frame > 1.0
    && !CancelModule::is_enable_cancel(fighter.module_accessor) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    if frame > 10.0
    && WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU) {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    }
    if marth_stance_cancel_helper(fighter).get_bool()
    || marth_stance_dash_cancel_helper(fighter, true).get_bool() {
        return 1.into();
    }
    marth_stance_mot_end_helper(fighter);
    0.into()
}

unsafe extern "C" fn marth_speciallw_hit_mot_helper(fighter: &mut L2CFighterCommon) {
    let sit = fighter.global_table[SITUATION_KIND].get_i32();
    let correct;
    let mot;
    if sit != *SITUATION_KIND_GROUND {
        correct = *GROUND_CORRECT_KIND_AIR;
        mot = Hash40::new("special_air_lw");
    }
    else {
        correct = *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP;
        mot = Hash40::new("special_lw");
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(correct));
    MotionModule::change_motion_inherit_frame(
        fighter.module_accessor,
        mot,
        -1.0,
        1.0,
        0.0,
        false,
        false
    );
}

#[status_script(agent = "marth", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn marth_speciallw_hit_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU)
    && ![
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK),
        CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3),
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    marth_speciallw_common_end(fighter);
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER

unsafe extern "C" fn marth_speciallw_enter_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        0,
        *FIGHTER_STATUS_ATTR_START_TURN as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_enter_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_enter"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            air_speed_x_stable * vl::param_stance::enter_air_speed_x_mul
        );
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        sv_kinetic_energy!(
            mul_x_speed_max,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::fall_speed_x_mul
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_enter_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_enter_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_ground_air_cancel_helper(fighter, true).get_bool() {
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            sv_kinetic_energy!(
                mul_x_speed_max,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                vl::param_stance::fall_speed_x_mul
            );
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
    }
    marth_stance_mot_end_helper(fighter);
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT

unsafe extern "C" fn marth_speciallw_wait_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        0,
        (*FIGHTER_STATUS_KIND_ITEM_ASSIST_HOIST | *FIGHTER_STATUS_ATTR_INTO_DOOR | *FIGHTER_STATUS_ATTR_ENABLE_ROCKETBELT_EJECT) as u32,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_wait"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        sv_kinetic_energy!(
            mul_x_speed_max,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            vl::param_stance::fall_speed_x_mul
        );
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_wait_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_wait_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_ground_air_cancel_helper(fighter, false).get_bool() {
        return 1.into();
    }
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
        let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT);
        fighter.change_status(status.into(), true.into());
        return 1.into();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            sv_kinetic_energy!(
                mul_x_speed_max,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                vl::param_stance::fall_speed_x_mul
            );
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        }
    }
    0.into()
}

// Used for both dashes

unsafe extern "C" fn marth_speciallw_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        0,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F

unsafe extern "C" fn marth_speciallw_dash_f_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(
        fighter.module_accessor,
        *FIGHTER_PAD_COMMAND_CATEGORY1,
        *FIGHTER_PAD_CMD_CAT1_DASH
    );
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.5
    );
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("final_dash_end"),
        0.0,
        2.5,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_dash_main_loop as *const () as _))
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B

unsafe extern "C" fn marth_speciallw_dash_b_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    ControlModule::clear_command_one(
        fighter.module_accessor,
        *FIGHTER_PAD_COMMAND_CATEGORY1,
        *FIGHTER_PAD_CMD_CAT1_TURN_DASH
    );
    sv_kinetic_energy!(
        set_speed_mul,
        fighter,
        FIGHTER_KINETIC_ENERGY_ID_MOTION,
        0.5
    );
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("escape_b"),
        0.0,
        2.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_dash_main_loop as *const () as _))
}

// Dash Main Loop

unsafe extern "C" fn marth_speciallw_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[MOTION_FRAME].get_f32() >= vl::param_stance::dash_attack_cancel_frame {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        if marth_stance_cancel_helper(fighter).get_bool() {
            return 1.into();
        }
        if fighter.global_table[MOTION_FRAME].get_f32() >= vl::param_stance::dash_redash_frame {
            if marth_stance_dash_cancel_helper(fighter, true).get_bool() {
                return 1.into();
            }
        }
        marth_stance_mot_end_helper(fighter);
    }
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT

unsafe extern "C" fn marth_speciallw_exit_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_NONE),
        *FIGHTER_KINETIC_TYPE_UNIQ,
        *GROUND_CORRECT_KIND_KEEP as u32,
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
        0,
        0,
        0,
        0
    );
    0.into()
}

unsafe extern "C" fn marth_speciallw_exit_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_exit"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_exit_main_loop as *const () as _))
}

unsafe extern "C" fn marth_speciallw_exit_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

// Jab/Tilt common pre function

unsafe extern "C" fn marth_speciallw_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW
            | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
            | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32,
        0
    );
    0.into()
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK

unsafe extern "C" fn marth_speciallw_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_11"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_main_loop as *const () as _))
}

// FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3

unsafe extern "C" fn marth_speciallw_attack_lw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_lw_attack_lw3"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(marth_speciallw_attack_main_loop as *const () as _))
}

// Jab/Tilt common main loop function

unsafe extern "C" fn marth_speciallw_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if marth_stance_cancel_helper(fighter).get_bool() {
        return 1.into();
    }
    if marth_stance_dash_cancel_helper(fighter, true).get_bool() {
        return 1.into();
    }
    marth_stance_mot_end_helper(fighter);
    0.into()
}

// Jab/Tilt common end function

unsafe extern "C" fn marth_speciallw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU);
    }
    marth_speciallw_common_end(fighter);
    0.into()
}

// Some common functions used for the stance

unsafe extern "C" fn marth_stance_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD)
    && !fighter.global_table[IN_HITLAG].get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if marth_stance_air_cancel_helper(fighter).get_bool() {
                    return true.into();
                }
            }
            else {
                if marth_stance_ground_cancel_helper(fighter).get_bool () {
                    return true.into();
                }
            }
        }
        else {
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_CATCH);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_SPECIAL);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
            WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_SPECIAL);
            if fighter.sub_transition_group_check_ground_catch().get_bool()
            || fighter.sub_transition_group_check_ground_special().get_bool()
            || fighter.sub_transition_group_check_ground_attack().get_bool()
            || fighter.sub_transition_group_check_air_special().get_bool() 
            || fighter.sub_transition_group_check_air_attack().get_bool() {
                return true.into();
            }
        }
    }
    false.into()
}

unsafe extern "C" fn marth_ground_air_cancel_helper(fighter: &mut L2CFighterCommon, require_cancel: bool) -> L2CValue {
    if !require_cancel || CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            if marth_stance_ground_cancel_helper(fighter).get_bool()
            || marth_stance_dash_cancel_helper(fighter, require_cancel).get_bool() {
                return true.into();
            }
        }
        else {
            if marth_stance_air_cancel_helper(fighter).get_bool() {
                return true.into();
            }
        }
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_ground_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    // let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3);
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
        return true.into();
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
        return true.into();
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
        return true.into();
    }
    let curr_status = fighter.global_table[STATUS_KIND].get_i32();
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3);
    if curr_status < status
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 != 0 {
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK);
    if curr_status < status
    && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    && only_jabs(fighter) {
        fighter.change_status(status.into(), true.into());
        return true.into();
    }
    false.into()
}

pub unsafe extern "C" fn marth_stance_air_cancel_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    // let curr_status = fighter.global_table[STATUS_KIND].get_i32();
    // let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK);
    // if curr_status < status
    // && cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0
    // && only_jabs(fighter) {
    //     fighter.change_status(status.into(), true.into());
    //     return true.into();
    // }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_HI != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_HI.into(), true.into());
        return true.into();
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_S != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), true.into());
        return true.into();
    }
    if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N != 0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
        return true.into();
    }
    false.into()
}

unsafe extern "C" fn marth_stance_dash_cancel_helper(fighter: &mut L2CFighterCommon, require_cancel: bool) -> L2CValue {
    let curr_status = fighter.global_table[STATUS_KIND].get_i32();
    let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK);
    let is_jab = curr_status == status && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT | *COLLISION_KIND_MASK_SHIELD);
    let cancel = CancelModule::is_enable_cancel(fighter.module_accessor) || is_jab;
    let cancel = cancel && !fighter.global_table[IN_HITLAG].get_bool();
    if cancel || !require_cancel {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
            let cat1 = fighter.global_table[CMD_CAT1].get_i32();
            let cat2 = fighter.global_table[CMD_CAT2].get_i32();
            let lr = PostureModule::lr(fighter.module_accessor);
            let dash_f = if lr < 0.0 {
                cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
            }
            else {
                cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0
            };
            let dash_b = if lr < 0.0 {
                cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R != 0
            }
            else {
                cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L != 0
            };
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_TURN_DASH != 0
            || dash_b {
                let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B);
                fighter.change_status(status.into(), true.into());
                return true.into();
            }
            if cat1 & *FIGHTER_PAD_CMD_CAT1_FLAG_DASH != 0
            || dash_f {
                let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F);
                fighter.change_status(status.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

unsafe extern "C" fn marth_stance_mot_end_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE) {
            let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT);
            fighter.change_status(status.into(), true.into());
            return true.into();
        }
        else {
            let status = CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT);
            fighter.change_status(status.into(), false.into());
        }
    }
    false.into()
}

unsafe extern "C" fn marth_speciallw_common_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status < CustomStatusModule::get_agent_status_kind(fighter.battle_object, FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER)
    && status != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_IS_STANCE);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        marth_specials_main,
        marth_specials2_main,
        marth_specials3_main,
        marth_specials3_exec_stop,
        marth_speciallw_hit_main,
        marth_speciallw_hit_end
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ENTER,
        StatusInfo::new()
            .with_pre(marth_speciallw_enter_pre)
            .with_main(marth_speciallw_enter_main)
            .with_end(marth_speciallw_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_WAIT,
        StatusInfo::new()
            .with_pre(marth_speciallw_wait_pre)
            .with_main(marth_speciallw_wait_main)
            .with_end(marth_speciallw_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_F,
        StatusInfo::new()
            .with_pre(marth_speciallw_dash_pre)
            .with_main(marth_speciallw_dash_f_main)
            .with_end(marth_speciallw_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_DASH_B,
        StatusInfo::new()
            .with_pre(marth_speciallw_dash_pre)
            .with_main(marth_speciallw_dash_b_main)
            .with_end(marth_speciallw_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_EXIT,
        StatusInfo::new()
            .with_pre(marth_speciallw_exit_pre)
            .with_main(marth_speciallw_exit_main)
            .with_end(marth_speciallw_common_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK,
        StatusInfo::new()
            .with_pre(marth_speciallw_attack_pre)
            .with_main(marth_speciallw_attack_main)
            .with_end(marth_speciallw_attack_end)
    );
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_marth"),
        FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_ATTACK_LW3,
        StatusInfo::new()
            .with_pre(marth_speciallw_attack_pre)
            .with_main(marth_speciallw_attack_lw3_main)
            .with_end(marth_speciallw_attack_end)
    );
}