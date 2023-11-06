use crate::imports::status_imports::*;
use super::super::helper::*;

unsafe extern "C" fn dolly_attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    fighter.status_AttackS3Common();
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attack_s3_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attack_s3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if dolly_hit_cancel(fighter).get_i32() == 0
    && dolly_attack_start_cancel(fighter).get_i32() == 0 {
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_s3_lw")
        && !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let stick_dir = ControlModule::get_stick_dir(fighter.module_accessor);
            let attack_s3_stick_dir_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_dir_hi"));
            if (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4 == 0
            && FGCModule::cancel_exceptions(
                fighter,
                *FIGHTER_STATUS_KIND_ATTACK_HI3,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3,
                true
            ).get_bool())
            || (fighter.global_table[CMD_CAT1].get_i32() & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4 == 0
            && attack_s3_stick_dir_hi < stick_dir
            && FGCModule::cancel_exceptions(
                fighter,
                *FIGHTER_STATUS_KIND_ATTACK_S3,
                *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3,
                true
            ).get_bool()) {
                VarModule::on_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
                return 1.into();
            }
        }
        dolly_attack_s3_main_loop_inner(fighter, FIGHTER_COMBO_KIND_S3.into());
        return 0.into();
    }
    1.into()
}

unsafe extern "C" fn dolly_attack_s3_main_loop_inner(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor)
    && fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 0.into();
    }
    if CustomCancelManager::execute_cancel(fighter) {
        return 1.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        let combo = ComboModule::count(fighter.module_accessor) as i32;
        let s3_combo_max = WorkModule::get_param_int(fighter.module_accessor, hash40("s3_combo_max"), 0);
        if combo < s3_combo_max
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE)
        && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO) {
            dolly_attack_s3_mtrans_param(fighter, param_1);
        }
    }
    else {
        dolly_attack_s3_mtrans_param(fighter, param_1);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        if 0 < jump_attack_frame {
            if !StopModule::is_stop(fighter.module_accessor)
            && fighter.sub_check_button_jump().get_bool() {
                let log = fighter.status_attack();
                let info = log[0x10f40d7b92u64].get_i64();
                let mot = MotionModule::motion_kind(fighter.module_accessor);
                MotionAnimcmdModule::call_script_single(
                    fighter.module_accessor,
                    *FIGHTER_ANIMCMD_EXPRESSION,
                    Hash40::new_raw(mot),
                    -1
                );
                WorkModule::set_int64(fighter.module_accessor, info, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                fighter.change_status_jump_mini_attack(true.into());
                return 1.into();
            }
        }
        if 1 == jump_attack_frame {
            if !fighter.global_table[IS_STOP].get_bool()
            && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND) > 0 {
                let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
                FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
                WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn dolly_attack_s3_mtrans_param(fighter: &mut L2CFighterCommon, param_1: L2CValue) {
    ControlModule::reset_trigger(fighter.module_accessor);
    ControlModule::clear_command(fighter.module_accessor, false);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_PRECEDE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT);
    ComboModule::set(fighter.module_accessor, param_1.get_i32());
    let mut cont = false;
    if StatusModule::is_changing(fighter.module_accessor) {
        let status_interrupt = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
        let prev_status = fighter.global_table[PREV_STATUS_KIND].get_i32();
        if status_interrupt != prev_status {
            cont = true;
        }
        else {
            if FighterMotionModuleImpl::is_valid_cancel_frame(fighter.module_accessor, -1, true) {
                cont = true;
            }
        }
    }
    let mot;
    let stick_dir = ControlModule::get_stick_dir(fighter.module_accessor);
    let force_hi = VarModule::is_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
    VarModule::off_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
    let attack_s3_stick_dir_hi = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_dir_hi"));
    let attack_s3_stick_dir_lw = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_dir_lw"));
    if attack_s3_stick_dir_hi < stick_dir
    || force_hi {
        mot = Hash40::new("attack_s3_hi");
    }
    else if stick_dir < attack_s3_stick_dir_lw {
        mot = Hash40::new("attack_s3_lw");
    }
    else {
        fighter.clear_lua_stack();
        mot = sv_fighter_util::get_attack_s3_s_motion(fighter.lua_state_agent);
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
    let attack_struct = fighter.status_attack();
    let log_infos = attack_struct["log_infos"].clone();
    let attack_s3_s = log_infos["attack_s3_s"].get_u64();
    WorkModule::set_int64(fighter.module_accessor, attack_s3_s as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if !StatusModule::is_changing(fighter.module_accessor) {
        let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
        if 0 < jump_attack_frame {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    else {
        if cont
        && !force_hi
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            let jump_mini_attack_enable_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("jump_mini_attack_enable_frame"));
            WorkModule::set_int(fighter.module_accessor, jump_mini_attack_enable_frame + 1, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
        }
    }
    let jump_attack_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME);
    if jump_attack_frame != 0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK) {
            return;
        }
    }
    let log = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    if 0 < log {
        FighterStatusModuleImpl::reset_log_action_info(fighter.module_accessor, log);
        WorkModule::set_int64(fighter.module_accessor, 0, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_S3, dolly_attack_s3_main);
}