use {
    smash::{
        lua2cpp::{L2CFighterCommon, *},
        hash40,
        phx::Hash40,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*,
        table_const::*
    },
    super::super::common::common_status::dash::fgc_dashback_main
};

#[status_script(agent = "dolly", status = FIGHTER_DOLLY_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ESCAPE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_escape_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Escape_common();
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ESCAPE_ATTACK);
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_escape_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_escape_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 0.into();
        }
    }
    else {
        let normal_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_HIT_NORMAL_FRAME);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
        && normal_frame == 1 {
            let cat = fighter.global_table[CMD_CAT1].get_i32();
            if cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_F != 0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB);
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_F.into(), true.into());
                return 0.into();
            }
            if cat & *FIGHTER_PAD_CMD_CAT1_FLAG_ESCAPE_B != 0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ESCAPE_XLU_START_1F);
                WorkModule::on_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_CANCEL_ESCAPE_TO_ESCAPE_FB);
                fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_B.into(), true.into());
                return 0.into();
            }
        }
        let enable_attack = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_WORK_INT_ESCAPE_ATTACK);
        if enable_attack == *FIGHTER_ESCAPE_ATTACK_MODE_ENABLE {
            let is_catch = fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH == 0;
            if is_catch as i32 & 1 != 0 {
                if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                    return 0.into();
                }
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 0.into();
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ESCAPE_WORK_FLAG_ATTACK) {
        if fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N != 0 {
            if fighter.global_table[CMD_CAT4].get_i32() & (
                *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_R_COMMAND | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL_COMMAND |
                *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_COMMAND | *FIGHTER_PAD_CMD_CAT4_FLAG_SUPER_SPECIAL2_R_COMMAND
            ) == 0 {
                if FighterControlModuleImpl::special_command_236236_step(fighter.module_accessor) & 0xff < 6 {
                    if FighterControlModuleImpl::special_command_214214_step(fighter.module_accessor) & 0xff < 6 {
                        if FighterControlModuleImpl::special_command_21416_step(fighter.module_accessor) & 0xff < 5 {
                            if FighterControlModuleImpl::special_command_23634_step(fighter.module_accessor) & 0xff < 5 {
                                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ESCAPE_ATTACK);
                                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_HI3.into(), true.into());
                                return 0.into();
                            }
                        }
                    }
                }
            }
        }
    }
    fighter.sub_escape_check_rumble();
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_attacklw3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    dolly_attacklw3_common(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attacklw3_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attacklw3_common(fighter: &mut L2CFighterCommon) {
    let cont = dolly_attacklw3_common_helper(fighter);
    let mot;
    if WorkModule::get_int(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT) == 0 {
        mot = Hash40::new("attack_lw3");
    }
    else {
        mot = Hash40::new("attack_lw3_2");
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
    WorkModule::set_int64(fighter.module_accessor, hash40("attack_lw3") as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if cont {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
            WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) != 0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            if 0 < attack_kind {
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
                WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            }
        }
    }
    else {
        let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        if 0 < attack_kind {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        }
    }
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
    WorkModule::unable_transition_term_group_ex(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN);
    let fb_kind = ControlModule::get_attack_lw3_fb_kind(fighter.module_accessor);
    if fb_kind == *FIGHTER_COMMAND_ATTACK3_KIND_B {
        PostureModule::reverse_lr(fighter.module_accessor);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, lr);
        sv_kinetic_energy::set_chara_dir(fighter.lua_state_agent);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.sub_attack3_uniq_check(false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack3_uniq_check as *const () as _));
}

unsafe extern "C" fn dolly_attacklw3_common_helper(fighter: &mut L2CFighterCommon) -> bool {
    let status_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let status_prev = fighter.global_table[PREV_STATUS_KIND].get_i32();
    if status_interrupt != status_prev {
        if status_prev == *FIGHTER_STATUS_KIND_ESCAPE {
            if status_interrupt != status_prev {
                if FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
                    return true;
                }
            }
        }
        else {
            return true;
        }
    }
    else {
        if status_interrupt != status_prev {
            if FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
                return true;
            }
        }
    }
    false
}

unsafe extern "C" fn dolly_attacklw3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        reset_i32(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT);
    }
    if dolly_hit_cancel(fighter).get_bool() == false {
        if dolly_attack_start_cancel(fighter).get_bool() == false {
            if chain_cancels(fighter,
                *FIGHTER_STATUS_KIND_ATTACK_LW3,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3,
                true,
                FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT,
                1
            ).get_bool() == false {
                fighter.status_AttackLw3_Main();
                return 0.into();
            }
        }
    }
    1.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_LW3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_attacklw3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK_LW3 {
        reset_i32(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_INT_D_TILT_CHAIN_COUNT);
    }
    fighter.status_end_AttackLw3()
}

unsafe extern "C" fn dolly_hit_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if dolly_final_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                return 1.into();
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
            if dolly_special_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                return 1.into();
            }
        }
    }
    0.into()
}

unsafe extern "C" fn dolly_special_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let ret;
    let special_n = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    let special_s = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    let special_hi = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    let special_lw = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    let special_s_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    let special_hi_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    let special_lw_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    let super_special = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    let super_special2 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        ret = fighter.sub_transition_group_check_air_special();
    }
    else {
        ret = fighter.sub_transition_group_check_ground_special();
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
    if !special_s_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    }
    if !special_hi_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    }
    if !special_lw_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    }
    if !super_special {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    }
    if !super_special2 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    }
    ret
}

unsafe extern "C" fn dolly_final_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue {
    let ret;
    let final_cancel = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    if situation.get_i32() != *SITUATION_KIND_GROUND {
        ret = fighter.sub_transition_group_check_air_special();
    }
    else {
        ret = fighter.sub_transition_group_check_ground_special();
    }
    if !final_cancel {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL);
    }
    ret
}

unsafe extern "C" fn dolly_attack_start_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.global_table[MOTION_FRAME].get_f32() <= WorkModule::get_param_int(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame")) as f32
    && dolly_kara_cancel(fighter).get_bool() {
        return 1.into();
    }
    0.into()
}

unsafe extern "C" fn dolly_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue {
    let special_s_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    let special_hi_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    let special_lw_command = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    let super_special = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    let super_special2 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    let val = fighter.sub_transition_group_check_air_special();
    if !special_s_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND);
    }
    if !special_hi_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND);
    }
    if !special_lw_command {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND);
    }
    if !super_special {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
    }
    if !super_special2 {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL2);
    }
    val
}

pub fn install() {
    install_status_scripts!(
        dolly_dashback_main,
        dolly_escape_main,
        dolly_attacklw3_main,
        dolly_attacklw3_end
    );
}