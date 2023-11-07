use crate::imports::status_imports::*;

unsafe extern "C" fn pikmin_attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_AttackS3Common();
    fighter.sub_shift_status_main(L2CValue::Ptr(pikmin_attack_s3_main_loop as *const () as _))
}

unsafe extern "C" fn pikmin_attack_s3_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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
            fighter.attack_s3_mtrans();
        }
    }
    else {
        fighter.attack_s3_mtrans();
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
            if pikmin_attack_s3_handle_loop(fighter).get_bool() {
                return 0.into();
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    0.into()
}

unsafe extern "C" fn pikmin_attack_s3_handle_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let step = VarModule::get_int(fighter.module_accessor, pikmin::status::int::ATTACK_S3_STEP);
    if step != pikmin::ATTACK_S3_STEP_END {
        if fighter.sub_transition_group_check_ground_guard().get_bool() {
            return true.into();
        }
        let mot;
        let step;
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            mot = hash40("attack_s3_loop");
            step = pikmin::ATTACK_S3_STEP_LOOP;
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            VarModule::inc_int(fighter.module_accessor, pikmin::instance::int::ATTACK_S3_LOOP_COUNT);
        }
        else {
            mot = hash40("attack_s3_end");
            step = pikmin::ATTACK_S3_STEP_END;
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
        }
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
        VarModule::set_int(fighter.module_accessor, pikmin::status::int::ATTACK_S3_STEP, step);
        false.into()
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        true.into()
    }
}

unsafe extern "C" fn pikmin_attack_s3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_GUARD_ON {
        VarModule::set_int(fighter.module_accessor, pikmin::instance::int::ATTACK_S3_LOOP_COUNT, 0);
    }
    fighter.status_end_AttackS3()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_S3, pikmin_attack_s3_main);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_ATTACK_S3, pikmin_attack_s3_end);
}