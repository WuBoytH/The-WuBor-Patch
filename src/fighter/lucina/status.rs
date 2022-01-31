use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::{Hash40, Vector2f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    super::helper::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    }
};

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
        lucina_jab_cancels_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_jab_cancels_substatus2 as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_Attack_Main as *const () as _))
}

unsafe extern "C" fn lucina_jab_cancels_substatus2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IN_HITLAG].get_bool() {
        let special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2
        ].to_vec();
        let normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        ].to_vec();
        FGCModule::cancel_system(fighter, normal_cancels, special_cancels, false, 0);
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_attacks3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS3Common();
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_tilt_cancels_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_tilt_cancels_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackS3_Main as *const () as _))
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.clear_lua_stack();
    let mot = sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    fighter.status_AttackHi3_Common(mot.into(), Hash40::new("attack_hi3").into());
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
        lucina_attackhi3_cancel_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_attackhi3_cancel_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackHi3_Main as *const () as _))
}

unsafe extern "C" fn lucina_attackhi3_cancel_substatus2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IN_HITLAG].get_bool() {
        let special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2
        ].to_vec();
        let jump_cancel = 1;
        FGCModule::cancel_system(fighter, [].to_vec(), special_cancels, false, jump_cancel);
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackLw3_common();
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_tilt_cancels_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_tilt_cancels_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackLw3_Main as *const () as _))
}

unsafe extern "C" fn lucina_tilt_cancels_substatus2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IN_HITLAG].get_bool() {
        let special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2
        ].to_vec();
        FGCModule::cancel_system(fighter, [].to_vec(), special_cancels, false, 0);
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(true.into());
    lucina_attackair_set_cancels(fighter);
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_attackair_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_attackair_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_AttackAir_Main as *const () as _))
}

unsafe extern "C" fn lucina_attackair_set_cancels(fighter: &mut L2CFighterCommon) {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    let flags;
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLAG_NORMAL_CANCEL);
    if mot == hash40("attack_air_n") {
        flags = ATTACK_AIR_F_MASK + ATTACK_AIR_B_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
        FGCModule::disable_aerial(fighter, ATTACK_AIR_N_MASK);
    }
    else if mot == hash40("attack_air_f") {
        flags = ATTACK_AIR_N_MASK + ATTACK_AIR_B_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
        FGCModule::disable_aerial(fighter, ATTACK_AIR_F_MASK);
    }
    else if mot == hash40("attack_air_b") {
        flags = ATTACK_AIR_N_MASK + ATTACK_AIR_F_MASK + ATTACK_AIR_HI_MASK + ATTACK_AIR_LW_MASK;
        FGCModule::disable_aerial(fighter, ATTACK_AIR_B_MASK);
    }
    else if mot == hash40("attack_air_hi") {
        flags = ATTACK_AIR_B_MASK + ATTACK_AIR_LW_MASK;
        FGCModule::disable_aerial(fighter, ATTACK_AIR_HI_MASK);
    }
    else {
        flags = 0b00000;
        FGCModule::disable_aerial(fighter, ATTACK_AIR_LW_MASK);
    }
    WorkModule::set_int(fighter.module_accessor, flags, FIGHTER_STATUS_WORK_ID_INT_ENABLED_AERIALS);
}

unsafe extern "C" fn lucina_attackair_substatus2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IN_HITLAG].get_bool() {
        let mut special_cancels : Vec<i32> = [].to_vec();
        let mut jump_cancel = 1;
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        if mot != hash40("attack_air_lw") {
            if mot == hash40("attack_air_f") {
                if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_WORK_ID_FLAG_JUMP_CANCEL) {
                    jump_cancel = 0;
                }
            }
            special_cancels = [
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI,
                *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2
            ].to_vec();
        }
        FGCModule::cancel_system(fighter, [].to_vec(), special_cancels, true, jump_cancel);
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CHARGE_MAX);
    lucina_specialn_mot_helper(fighter);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND)
    && spent_meter(fighter.module_accessor, false) {
        let spent = WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_SPENT_SP);
        add_sp(fighter.module_accessor, -spent);
        WorkModule::set_int(fighter.module_accessor, 40, FIGHTER_YU_INSTANCE_WORK_ID_INT_SP_FLASH_TIMER);
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_IS_EX);
        sp_diff_checker(fighter.module_accessor);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_specials_cancel_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_specials_cancel_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specialn_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_air_n_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_air_n_start"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_n_start"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new("special_n_start"),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
}

unsafe extern "C" fn lucina_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor)
    && StatusModule::is_situation_changed(fighter.module_accessor) {
        lucina_specialn_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialn_loop_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_n_loop"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialn_loop_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specialn_loop_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
    && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_IS_EX) {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_IS_EX) {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
                }
            }
            else {
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_HEROIC_GRAB);
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), false.into());
            }
        }
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialn_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_STATUS_FLAG_IS_EX) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_end_max") as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_end_max") as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_AIR_MOTION);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_end") as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_end") as i64, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_AIR_MOTION);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_specials_cancel_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_specials_cancel_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialn_end_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specialn_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            lucina_specialn_end_mot_helper(fighter);
        }
    }
    else {
        lucina_specialn_end_mot_helper(fighter);
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

unsafe extern "C" fn lucina_specialn_end_mot_helper(fighter: &mut L2CFighterCommon) {
    let ground_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_MOTION);
    let air_mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_WORK_INT_END_AIR_MOTION);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let air_brake_x = sv_fighter_util::get_default_fighter_param_air_brake_x(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, air_brake_x, 0.0);
        sv_kinetic_energy::set_brake(fighter.lua_state_agent);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(air_mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(air_mot),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        let ground_brake = sv_fighter_util::get_default_fighter_param_ground_brake(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_brake, 0.0);
        sv_kinetic_energy::set_brake(fighter.lua_state_agent);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(ground_mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_N_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(
                fighter.module_accessor,
                Hash40::new_raw(ground_mot),
                -1.0,
                1.0,
                0.0,
                false,
                false
            );
        }
    }
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucina_specials_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_turn;
    if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND) {
        is_turn = *FIGHTER_STATUS_ATTR_START_TURN as u32;
    }
    else {
        is_turn = 0;
    }
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        is_turn,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    WorkModule::on_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N_S);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s1"),
        1.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_raginglion_loop as *const () as _))
}

unsafe extern "C" fn lucina_raginglion_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2.into(), false.into());
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_air_s2_hi"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_specials_cancel_substatus2(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_specials_cancel_substatus2 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials2_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials2_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("special_s2_hi"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
            }
        }
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucina_specials4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_UNIQ,
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
        (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK |
        *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32,
        0
    );
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("special_s1"),
        1.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_lightningflash_loop as *const () as _))
}

unsafe extern "C" fn lucina_lightningflash_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.sub_wait_ground_check_common(0.into());
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn lucina_specialhi_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let turn = if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLAG_COMMAND) {
        *FIGHTER_STATUS_ATTR_START_TURN as u32
    }
    else {
        0
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
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI as u64,
        turn,
        *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32,
        0
    );
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn lucina_specialhi_exec(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_hi"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_hi"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    if !StopModule::is_stop(fighter.module_accessor) {
        lucina_specials_cancel_substatus2(fighter);
        lucina_specialhi_substatus(fighter);
    }
    fighter.global_table[SUB_STATUS2].assign(&L2CValue::Ptr(lucina_specials_cancel_substatus2 as *const () as _));
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(lucina_specialhi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_specials_cancel_substatus2(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.global_table[IN_HITLAG].get_bool() {
        let special_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2
        ].to_vec();
        FGCModule::cancel_system(fighter, [].to_vec(), special_cancels, false, 0);
    }
    0.into()
}

unsafe extern "C" fn lucina_specialhi_substatus(fighter: &mut L2CFighterCommon) -> L2CValue {
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
    0.into()
}

unsafe extern "C" fn lucina_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        if MotionModule::frame(fighter.module_accessor) >= 36.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_speciallw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_YU_STATUS_SPECIAL_LW_FLAG_ROMAN_MOVE);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_lw"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    else {
        MotionModule::change_motion(
            fighter.module_accessor,
            Hash40::new("special_air_lw"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_speciallw_main_loop as *const () as _))
}

unsafe extern "C" fn lucina_speciallw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_YU_STATUS_SPECIAL_LW_FLAG_ROMAN_MOVE) {
        let move_x = WorkModule::get_float(fighter.module_accessor, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_ROMAN_MOVE);
        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
            x: move_x,
            y: 0.0
        });
        WorkModule::set_float(fighter.module_accessor, move_x * 0.9, FIGHTER_YU_INSTANCE_WORK_ID_FLOAT_ROMAN_MOVE);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        lucina_attack_main,
        lucina_attacks3_main,
        lucina_attackhi3_main,
        lucina_attacklw3_main,
        lucina_attackair_main,
        lucina_specialn_main, lucina_specialn_loop_main, lucina_specialn_end_main,
        lucina_specials_pre, lucina_specials_main,
        lucina_specials2_main,
        lucina_specials4_pre, lucina_specials4_main,
        lucina_specialhi_pre, lucina_specialhi_exec, lucina_specialhi_main,
        lucina_speciallw_main
    );
}