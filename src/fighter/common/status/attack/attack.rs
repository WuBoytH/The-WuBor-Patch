use crate::imports::*;

#[skyline::hook(replace = L2CFighterCommon_status_pre_Attack)]
unsafe extern "C" fn status_pre_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_ATTACK_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_1 as u32,
        0
    );
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_sub_status_AttackCommon)]
unsafe extern "C" fn sub_status_attackcommon(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[PREV_STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_ATTACK {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_ATTACK_WORK_INT_100_HIT_NEAR_COUNT);
    }
    // let combo = ComboModule::count(fighter.module_accessor) as i32;
    // let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
    // if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_EFLAME_INSTANCE_WORK_ID_FLAG_HAS_ESWORD)
    // || attack_combo_max <= combo {
        ComboModule::reset(fighter.module_accessor);
    // }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100);
    WorkModule::set_int64(fighter.module_accessor, hash40("attack_11") as i64, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
}

// The following five are reimplemented to make sure only Neutral Attack inputs
// can trigger Jab followups.
#[skyline::hook(replace = L2CFighterCommon_attack_combo_none_uniq_chk_button)]
unsafe extern "C" fn attack_combo_none_uniq_chk_button(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    if !param_1.get_bool() {
        if ControlModule::check_button_on(fighter.module_accessor, param_2.get_i32())
        && only_jabs(fighter) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
            }
        }
        fighter.attack_uniq_chk_command(param_3);
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            if !WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) {
                return;
            }
        }
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
}

#[skyline::hook(replace = L2CFighterCommon_attack_combo_uniq_chk_button)]
unsafe extern "C" fn attack_combo_uniq_chk_button(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue, param_3: L2CValue) {
    if !param_1.get_bool() {
        fighter.attack_uniq_chk_command(param_3.clone());
        if fighter.global_table[CMD_CAT1].get_i32() & param_3.get_i32() != 0 {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
            }
            VarModule::off_flag(fighter.module_accessor, attack::flag::INVALID_HOLD_INPUT);
        }
        else if !only_jabs(fighter) {
            VarModule::on_flag(fighter.module_accessor, attack::flag::INVALID_HOLD_INPUT);
        }
        let button = param_2.get_i32();
        if !ControlModule::check_button_on(fighter.module_accessor, button) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON);
        }
        else if !VarModule::is_flag(fighter.module_accessor, attack::flag::INVALID_HOLD_INPUT) {
            if !AttackModule::is_infliction_status(fighter.module_accessor, 0x7f) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART) {
                    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RELEASE_BUTTON) {
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
                        ComboModule::reset(fighter.module_accessor);
                    }
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO)
                && only_jabs(fighter) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
                    if ControlModule::check_button_on_trriger(fighter.module_accessor, button) {
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER);
                    }
                }
            }
            else if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            && only_jabs(fighter) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO);
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_COMBO)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_CONNECT_COMBO) {
            let combo_count = ComboModule::count(fighter.module_accessor) as i32;
            let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
            if combo_count != attack_combo_max {
                return;
            }
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART);
            ComboModule::reset(fighter.module_accessor);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO) {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO_TRIGGER) {
                    return;
                }
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART_ATTACK);
            }
        }
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            if !WorkModule::count_down_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME, 0) {
                return;
            }
        }
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
    }
}

#[skyline::hook(replace = L2CFighterCommon_attack_uniq_chk_command)]
unsafe extern "C" fn attack_uniq_chk_command(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let cat1 = fighter.global_table[CMD_CAT1].get_i32();
    if cat1 & param_1.get_i32() != 0
    && only_jabs(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE);
    }
}

#[skyline::hook(replace = L2CFighterCommon_check_100_count_button)]
unsafe extern "C" fn check_100_count_button(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    let button = param_1.get_i32();
    if only_jabs(fighter) {
        if fighter.global_table[IS_STOP].get_bool() {
            if !ControlModule::check_button_on_trriger(fighter.module_accessor, button)
            && !ControlModule::check_button_on_release(fighter.module_accessor, button) {
                return;
            }
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
        }
        else {
            if !ControlModule::check_button_trigger(fighter.module_accessor, button)
            && !ControlModule::check_button_release(fighter.module_accessor, button) {
                return;
            }
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
        }
    }
}

#[skyline::hook(replace = L2CFighterCommon_status_Attack_Main_button)]
unsafe extern "C" fn status_attack_main_button(fighter: &mut L2CFighterCommon, param_1: L2CValue, param_2: L2CValue) -> L2CValue {
    fighter.check_100_count_button(param_1.clone());
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
        let normal_cancels = [
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3,
            *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3
        ].to_vec();
        if normal_cancel_common(fighter, normal_cancels).get_bool() {
            return 1.into();
        }
    }
    let attack100_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack100_type"), 0);
    if attack100_type != *FIGHTER_ATTACK100_TYPE_NONE {
        if AttackModule::is_infliction_status(fighter.module_accessor, 0x7f)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100)
        && ControlModule::check_button_on(fighter.module_accessor, param_1.get_i32())
        && only_jabs(fighter) {
            let combo = ComboModule::count(fighter.module_accessor) as i32;
            let attack_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_max"), 0);
            if attack_combo_max <= combo
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO)
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), true.into());
                return 1.into();
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100) {
            let attack_100_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_INT_100_COUNT);
            let attack_100_enable_cnt = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_100_enable_cnt"), 0);
            if attack_100_enable_cnt <= attack_100_count
            && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_100.into(), true.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }
    if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME)
    && !StopModule::is_stop(fighter.module_accessor)
    && fighter.sub_check_button_jump().get_bool() {
        let mot = MotionModule::motion_kind(fighter.module_accessor);
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EXPRESSION, Hash40::new_raw(mot), -1);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        let callable: extern "C" fn(&mut L2CFighterCommon, L2CValue) -> L2CValue = std::mem::transmute(param_2.get_ptr());
        callable(fighter, true.into());
        return 1.into();
    }
    if 1 == WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME)
    && !fighter.global_table[IS_STOP].get_bool() {
        let kind =  WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        if 0 < kind {
            FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, kind);
            WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        }
    }
    let attack_combo_type = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_combo_type"), 0);
    if attack_combo_type != *FIGHTER_COMBO_TYPE_NONE {
        if attack_combo_type == *FIGHTER_COMBO_TYPE_HIT
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            return 1.into();
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
            fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 1.into();
    }
    0.into()
}

// The following three are reimplemented for resetting ground normals.
#[skyline::hook(replace = L2CFighterCommon_status_end_Attack)]
unsafe extern "C" fn status_end_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_Attack100)]
unsafe extern "C" fn status_end_attack100(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_SUB_KIND_NONE, *FIGHTER_INSTANCE_WORK_ID_INT_TRICK_SUB);
    0.into()
}

#[skyline::hook(replace = L2CFighterCommon_status_end_AttackDash)]
unsafe extern "C" fn status_end_attackdash(fighter: &mut L2CFighterCommon) -> L2CValue {
    FGCModule::reset_used_ground_normals(fighter, false);
    let attack_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < attack_kind {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, attack_kind);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
    0.into()
}

/// Checks if your attack input is strictly a Neutral Attack input.
#[inline(always)]
pub unsafe extern "C" fn only_jabs(fighter: &mut L2CFighterCommon) -> bool {
    !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON)
    && !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
    && fighter.global_table[CMD_CAT1].get_i32() & (
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 | *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4 |
        *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH
    ) == 0
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_pre_attack,
            sub_status_attackcommon,
            attack_combo_none_uniq_chk_button,
            attack_combo_uniq_chk_button,
            attack_uniq_chk_command,
            check_100_count_button,
            status_attack_main_button,
            status_end_attack,
            status_end_attack100,
            status_end_attackdash
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}