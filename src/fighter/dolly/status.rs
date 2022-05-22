use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    wubor_utils::{
        wua_bind::*,
        vars::*,
        table_const::*
    },
    super::{helper::*, vl, vars::*},
    super::super::common::common_status::dash::*
};

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_dashback_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_pre(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_GUARD_OFF, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_guardoff_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOff()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Escape()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        dolly_attack_substatus3(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(dolly_attack_substatus3 as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attack_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attack_substatus3(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
        if combo >= attack_combo_max {
            return 0.into()
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
            return 0.into();
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    else {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    fighter.attack_mtrans();
    0.into()
}

unsafe extern "C" fn dolly_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_hit_cancel(fighter).get_i32() == 0 {
        let combo = ComboModule::count(fighter.module_accessor);
        if combo == 1
        && dolly_attack_start_cancel(fighter).get_i32() == 1 {
            return 1.into();
        }
        fighter.status_Attack_Main();
        return 0.into();
    }
    else {
        return 1.into();
    }
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_dash"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DASH_ATTACK_COMMAND) {
        sv_kinetic_energy!(
            set_speed_mul,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            vl::param_attack_dash::distance_mul_l
        );
    }
    else {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
        let catch_dash_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("catch_dash_frame"));
        WorkModule::set_int(fighter.module_accessor, catch_dash_frame, *FIGHTER_STATUS_ATTACK_DASH_WORK_INT_CATCH_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_DASH_FLAG_ATTACK_HI4_DISABLE);
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let jump_mini_attack_enable_frame = WorkModule::get_param_int(
                fighter.module_accessor,
                hash40("common"),
                hash40("jump_mini_attack_enable_frame")
            );
            WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
        let log = fighter.get_mini_jump_attack_data_log_info(hash40("attack_dash").into()).get_i64();
        WorkModule::set_int64(fighter.module_accessor, log, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_attack_dash_uniq(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attackdash_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attackdash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_hit_cancel(fighter).get_i32() == 0
    && dolly_attack_start_cancel(fighter).get_i32() == 0 {
        fighter.status_AttackDash_Main();
        return 0.into();
    }
    1.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_attackdash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
        *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND,
        *FIGHTER_STATUS_KIND_APPEAL
    ].contains(&fighter.global_table[STATUS_KIND].get_i32()) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        let mut mul = 0.3;
        if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_APPEAL {
            mul = 0.1;
        }
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
            speed_x * mul,
            0.0,
            0.0,
            0.0,
            0.0
        );
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DASH_ATTACK_COMMAND) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_DASH_ATTACK_COMMAND);
    }
    fighter.status_end_AttackDash();
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_attacks3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    fighter.status_AttackS3Common();
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attacks3_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attacks3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_hit_cancel(fighter).get_i32() == 0
    && dolly_attack_start_cancel(fighter).get_i32() == 0 {
        fighter.status_AttackS3_Main();
        return 0.into();
    }
    1.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_HI3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_attackhi3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    fighter.clear_lua_stack();
    let mut mot = sv_fighter_util::get_attack_hi3_motion(fighter.lua_state_agent);
    if fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ESCAPE
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ESCAPE_ATTACK) {
        mot = Hash40::new("escape_attack");
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x3a40337e2c), *FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_16 - 1);
    }
    fighter.status_AttackHi3_Common(mot.into(), mot.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attackhi3_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attackhi3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_hit_cancel(fighter).get_i32() == 0 {
        if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ESCAPE {
            if dolly_attack_start_cancel(fighter).get_i32() == 1 {
                return 1.into();
            }
        }
        else {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ESCAPE_ATTACK) {
                if dolly_attack_start_cancel(fighter).get_i32() == 1 {
                    return 1.into();
                }
            }
        }
        fighter.status_AttackHi3_Main();
        return 0.into();
    }
    else {
        return 1.into();
    }
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    fighter.status_AttackLw3_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        WarkModule::reset_i32(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT);
    }
    if dolly_hit_cancel(fighter).get_i32() == 0
    && dolly_attack_start_cancel(fighter).get_i32() == 0 {
        if FGCModule::chain_cancels(fighter,
            *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3,
            true,
            FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT,
            1
        ).get_bool() == false {
            fighter.status_AttackLw3_Main();
            return 0.into();
        }
        else {
            let count = WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT);
            let mot;
            match count {
                1 => mot = Hash40::new("attack_lw3_2"),
                _ => mot = Hash40::new("attack_lw3")
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
        }
    }
    1.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_attacklw3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        WarkModule::reset_i32(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT);
    }
    fighter.status_end_AttackLw3()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let aerial = ControlModule::get_attack_air_kind(fighter.module_accessor);
    let mot;
    match aerial {
        2 => mot = Hash40::new("attack_air_f"),
        3 => mot = Hash40::new("attack_air_b"),
        4 => mot = Hash40::new("attack_air_hi"),
        5 => mot = Hash40::new("attack_air_lw"),
        _ => mot = Hash40::new("attack_air_n")
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
    WorkModule::set_int64(fighter.module_accessor, mot.hash as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    fighter.sub_attack_air_common(false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attackair_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attackair_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if dolly_hit_cancel(fighter).get_i32() == 1 {
            return 1.into();
        }
    }
    if dolly_attack_start_cancel(fighter).get_i32() == 1 {
        return 1.into();
    }
    let frame = fighter.global_table[MOTION_FRAME].get_f32();
    let attack_cancel_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame")) as f32;
    if frame == attack_cancel_frame {
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        if mot == hash40("attack_air_f") {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F
            );
        }
        else if mot == hash40("attack_air_b") {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B
            );
        }
        else if mot == hash40("attack_air_hi") {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI
            );
        }
        else if mot == hash40("attack_air_lw") {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW
            );
        }
        else {
            notify_event_msc_cmd!(
                fighter,
                Hash40::new_raw(0x2b94de0d96),
                FIGHTER_LOG_ACTION_CATEGORY_ATTACK,
                FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N
            );
        }
    }
    fighter.status_AttackAir_Main();
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_n").into());
    fighter.sub_change_motion_by_situation(Hash40::new("special_n").into(), Hash40::new("special_air_n").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE) {
            ArticleModule::generate_article_enable(
                fighter.module_accessor,
                *FIGHTER_DOLLY_GENERATE_ARTICLE_WAVE,
                false,
                -1
            );
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE) {
                    let hop_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("hop_speed_y"));
                    sv_kinetic_energy!(
                        set_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        hop_speed_y
                    );
                    let gravity_accel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_accel"));
                    sv_kinetic_energy!(
                        set_accel,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        -gravity_accel
                    );
                    let gravity_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_max"));
                    sv_kinetic_energy!(
                        set_stable_speed,
                        fighter,
                        FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                        gravity_max
                    );
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_SPECIAL_N_HOP_DONE);
                }
            }
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE_DONE) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.clear_lua_stack();
                lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                let gravity = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                sv_kinetic_energy!(
                    reset_energy,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    ENERGY_STOP_RESET_TYPE_AIR,
                    0.0,
                    gravity,
                    0.0,
                    0.0,
                    0.0
                );
                let gravity_accel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_accel"));
                sv_kinetic_energy!(
                    set_accel,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    -gravity_accel
                );
                let gravity_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("gravity_max"));
                sv_kinetic_energy!(
                    set_stable_speed,
                    fighter,
                    FIGHTER_KINETIC_ENERGY_ID_GRAVITY,
                    gravity_max
                );
                KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
            else {
                KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
        else {
            fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_n").into());
        }
        fighter.sub_change_motion_by_situation(Hash40::new("special_n").into(), Hash40::new("special_air_n").into(), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
    }
    else {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specials_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specials_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialf_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END {
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialb_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialb_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specials_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialb_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

unsafe extern "C" fn dolly_specials_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialhi_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specialhi_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialhi_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_specialhi_end_main(fighter)
}

unsafe extern "C" fn dolly_specialhi_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
        MotionAnimcmdModule::flush_current_motion(fighter.module_accessor);
        ItemModule::set_change_status_event(fighter.module_accessor, true);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_specialhi_jump_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_JUMP
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_FALL
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_LANDING {
        MotionAnimcmdModule::flush_current_motion(fighter.module_accessor);
    }
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    ItemModule::set_change_status_event(fighter.module_accessor, true);
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_end_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_command_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    dolly_speciallw_end_main(fighter)
}

unsafe extern "C" fn dolly_speciallw_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_speciallw_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let param;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_HIT) {
        param = hash40("landing_frame_hit");
    }
    else {
        param = hash40("landing_frame_fail");
    }
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_lw"), param);
    WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_superspecial_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    dolly_superspecial_main_helper(fighter, hash40("param_super_special").into());
    let eff_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_INT_SCREEN_EFFECT_COUNT);
    if 0 < eff_count {
        MotionAnimcmdModule::call_script_single(
            fighter.module_accessor,
            *FIGHTER_ANIMCMD_EFFECT,
            Hash40::new("effect_superspecialcancelfillscreen"),
            -1
        );
    }
    0.into()
}

unsafe extern "C" fn dolly_superspecial_main_helper(fighter: &mut L2CFighterCommon, hash: L2CValue) {
    let param = hash.get_u64();
    let map_coll_joint = WorkModule::get_param_int64(fighter.module_accessor, param, hash40("map_coll_joint"));
    let offx = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_X);
    let offy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_Y);
    let offz = WorkModule::get_float(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SUPER_SPECIAL_WORK_FLOAT_MAP_COLL_OFFSET_Z);
    GroundModule::set_shape_data_rhombus_modify_node_offset(fighter.module_accessor, Hash40::new_raw(map_coll_joint), &Vector3f{x: offx, y: offy, z: offz});
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_superspecial2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
        ArticleModule::remove_exist(
            fighter.module_accessor,
            *FIGHTER_DOLLY_GENERATE_ARTICLE_FIRE,
            ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL)
        );
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("dolly_buster_punch"), true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("dolly_buster_dash"), true, true);
    }
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_superspecial2_blow_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_STATUS_KIND_WAIT
    && status != *FIGHTER_STATUS_KIND_FALL
    && status != *FIGHTER_STATUS_KIND_CATCH_CUT {
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_APPEAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_appeal_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL);
    fighter.status_end_Appeal();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_dashback_pre, dolly_dashback_main,
        dolly_guardoff_main,
        dolly_escape_main,
        dolly_attack_main,
        dolly_attackdash_main, dolly_attackdash_end,
        dolly_attacks3_main,
        dolly_attackhi3_main,
        dolly_attacklw3_main, dolly_attacklw3_end,
        dolly_attackair_main,
        dolly_specialn_main,
        dolly_specialn_end,
        dolly_specials_end,
        dolly_specials_command_end,
        dolly_specialf_attack_end,
        dolly_specialb_end,
        dolly_specialb_command_end,
        dolly_specialb_attack_end,
        dolly_specialhi_end,
        dolly_specialhi_command_end,
        dolly_specialhi_jump_end,
        dolly_speciallw_end,
        dolly_speciallw_command_end,
        dolly_speciallw_attack_end,
        dolly_superspecial_end,
        dolly_superspecial2_end,
        dolly_superspecial2_blow_end,
        dolly_appeal_end
    );
}