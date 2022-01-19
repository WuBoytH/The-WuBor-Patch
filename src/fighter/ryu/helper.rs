use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    super::super::common::common_status::attack::only_jabs,
    wubor_utils::table_const::*
};

pub unsafe extern "C" fn ryu_attack_main_uniq_chk(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo_count = ComboModule::count(fighter.module_accessor) as i32;
        let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
        if combo_count < attack_combo_max {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                return 0.into();
            }
            ryu_attack_main_uniq_chk2(fighter, hash40("attack_11_s").into(), hash40("attack_11_w").into());
        }
    }
    else {
        ryu_attack_main_uniq_chk2(fighter, hash40("attack_11_s").into(), hash40("attack_11_w").into());
    }
    0.into()
}

pub unsafe extern "C" fn ryu_attack_main_uniq_chk2(fighter: &mut L2CFighterCommon, mot1: L2CValue, mot2: L2CValue) {
    ryu_attack_reset(fighter);
    fighter.attack_mtrans_pre_process();
    if !StatusModule::is_changing(fighter.module_accessor) {
        if ComboModule::count(fighter.module_accessor) == 1 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT) {
                WorkModule::set_int64(fighter.module_accessor, 0x10556e6036, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
            }
            else {
                WorkModule::set_int64(fighter.module_accessor, 0xb4f4e6f8f, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK11, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
            }
        }
    }
    fighter.attack_mtrans_post_process();
    notify_event_msc_cmd!(fighter, 0x265a5c1b6bu64, mot1.get_int(), mot2.get_int());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    if 1 < ComboModule::count(fighter.module_accessor) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);
    }
}

pub unsafe extern "C" fn ryu_attack_reset(fighter: &mut L2CFighterCommon) {
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_BRANCH_FRAME_FIRST);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_CHANGE_LOG);
}

pub unsafe extern "C" fn ryu_attack_main_uniq_chk4(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() == false {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let count_down_int = WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0);
            if count_down_int & 1 != 0 {
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            }
        }
        else {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER) {
                return 0.into();
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
            && only_jabs(fighter) {
                let stick_y = fighter.global_table[STICK_Y].get_f32();
                let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
                let cont;
                if !(stick_y < attack_hi3_stick_y) {
                    cont = false;
                }
                else {
                    let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
                    cont = attack_lw3_stick_y < stick_y;
                }
                if cont {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
                    return 0.into();
                }
            }
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
        }
    }
    else {
        fighter.attack_uniq_chk();
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                if !AttackModule::is_infliction_status(fighter.module_accessor, 0x7f) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                        ComboModule::reset(fighter.module_accessor);
                    }
                }
            }
        }
        else if only_jabs(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                if !AttackModule::is_infliction_status(fighter.module_accessor, 0x7f) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                        ComboModule::reset(fighter.module_accessor);
                    }
                }
                else {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_COMBO) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
                let count = ComboModule::count(fighter.module_accessor) as i32;
                let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
                if count & attack_combo_max == 0 {
                    return 0.into();
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER) {
                    return 0.into();
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
            }
        }
    }
    0.into()
}

pub unsafe extern "C" fn ryu_specials_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_specials_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_specials_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_specials_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ryu_specials_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
                WorkModule::set_float(fighter.module_accessor, 10.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                return 1.into();
            }
            else {
                ryu_specials_mot_helper(fighter);
            }
        }
    }
    else {
        ryu_specials_mot_helper(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn ryu_specials_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if StatusModule::is_changing(fighter.module_accessor) {
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
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_accel_y"));
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -air_accel_y);
        sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        let air_max_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("air_max_speed_y"));
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_max_speed_y);
        sv_kinetic_energy::set_limit_speed(fighter.lua_state_agent);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if StatusModule::is_changing(fighter.module_accessor) {
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
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            return;
        }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy::set_brake(fighter.lua_state_agent);
    }
}

pub unsafe extern "C" fn ryu_specialhi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mot;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            mot = Hash40::new("special_air_hi_command");
        }
        else {
            mot = Hash40::new("special_air_hi");
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            mot = Hash40::new("special_hi_command");
        }
        else {
            mot = Hash40::new("special_hi");
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
    ItemModule::set_change_status_event(fighter.module_accessor, false);
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_specialhi_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_specialhi_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_specialhi_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_specialhi_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if !param_2.get_bool() {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
            let stickx = fighter.global_table[STICK_X].get_f32().abs();
            let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
            if lr_stick_x < stickx {
                PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }
        }
    }
    else {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_BUTTON_ON_TIMER);
    }
    0.into()
}

unsafe extern "C" fn ryu_specialhi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            ryu_specialhi_mot_helper(fighter);
        }
    }
    else {
        ryu_specialhi_mot_helper(fighter);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
        fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn ryu_specialhi_mot_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if StatusModule::is_changing(fighter.module_accessor) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_AIR_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            let start_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_accel_y"));
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -start_accel_y);
            sv_kinetic_energy::set_accel(fighter.lua_state_agent);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        if StatusModule::is_changing(fighter.module_accessor) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.clear_lua_stack();
            lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
            sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        }
    }
}

pub unsafe extern "C" fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let ret;
    let final_cancel = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        ret = fighter.sub_transition_group_check_air_special().get_bool();
    }
    else {
        ret = fighter.sub_transition_group_check_ground_special().get_bool();
    }
    if !final_cancel {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    }
    ret.into()
}

pub unsafe extern "C" fn ryu_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let ret;
    let special_n = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    let special_s = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    let special_hi = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    let special_lw = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    let special_n_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    let special_n2_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    let special_s_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    let special_hi_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    let attack_command1 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        ret = fighter.sub_transition_group_check_air_special().get_bool();
    }
    else {
        ret = fighter.sub_transition_group_check_ground_special().get_bool();
    }
    if !special_n {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    }
    if !special_s {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    }
    if !special_hi {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    }
    if !special_lw {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    }
    if !special_n_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    }
    if !special_n2_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    }
    if !special_s_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    }
    if !special_hi_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    }
    if !attack_command1 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    }
    ret.into()
}

pub unsafe extern "C" fn ryu_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret;
    let special_n_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    let special_n2_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    let special_s_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    let special_hi_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    let attack_command1 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    ret = fighter.sub_transition_group_check_special_command().get_bool();
    if !special_n_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND);
    }
    if !special_n2_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
    }
    if !special_s_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    }
    if !special_hi_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    }
    if !attack_command1 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_COMMAND1);
    }
    ret.into()
}

// pub unsafe extern "C" fn ryu_idkwhatthisis(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
//     let cont = fighter.get_mini_jump_attack_data_cancel_function(param_1.clone()).get_bool();
//     if fighter.get_mini_jump_attack_data_cancel_function(param_1.clone()).get_bool() == false {
//     }
//     else {
//         cont.into()
//     }
// }

pub unsafe extern "C" fn ryu_idkwhatthisis2(fighter: &mut L2CFighterCommon) {
    if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
        && only_jabs(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER);
        }
    }
}
