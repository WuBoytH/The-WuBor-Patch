use super::*;

unsafe extern "C" fn demon_attack_step_2s_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_STEP_2S_HOLD);
    MotionModule::change_motion(
        fighter.module_accessor,
        Hash40::new("attack_step_2s"),
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_ATTACK, FIGHTER_LOG_ATTACK_KIND_ADDITIONS_ATTACK_14);
    ControlModule::reset_special_command(fighter.module_accessor, true);
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, false);
    let rage_system = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM);
    WorkModule::set_flag(fighter.module_accessor, rage_system, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_RAGE_SYSTEM);
    if !StopModule::is_stop(fighter.module_accessor) {
        demon_attack_step_2s_substatus(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(demon_attack_step_2s_substatus as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(demon_attack_step_2s_main_loop as *const () as _))
}

unsafe extern "C" fn demon_attack_step_2s_substatus(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue {
    if !param_1.get_bool() {
        if VarModule::is_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_STEP_2S_HOLD)
        && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::off_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_STEP_2S_HOLD);
        }
    }
    0.into()
}

unsafe extern "C" fn demon_attack_step_2s_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }

    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_ATTACK_STEP_FLAG_CANCEL);
            return 0.into();
        }
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        return 0.into();
    }

    if VarModule::is_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_STEP_2S_TO_HEAVENS_GATE) {
        VarModule::off_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_STEP_2S_TO_HEAVENS_GATE);
        let step = VarModule::get_int(fighter.module_accessor, vars::demon::status::int::ATTACK_STEP_2S_TO_HEAVENS_GATE_STEP);
        if step == 0 {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW)
            && fighter.global_table[CMD_CAT4].get_i32() & *FIGHTER_PAD_CMD_CAT4_FLAG_COMMAND_623A != 0
            && fighter.sub_transition_term_id_cont_disguise(fighter.global_table[CHECK_SPECIAL_LW_UNIQ].clone()).get_bool() {
                MotionModule::change_motion(
                    fighter.module_accessor,
                    Hash40::new("attack_step_2s_s"),
                    0.0,
                    1.0,
                    false,
                    0.0,
                    false,
                    false
                );
                VarModule::inc_int(fighter.module_accessor, vars::demon::status::int::ATTACK_STEP_2S_TO_HEAVENS_GATE_STEP);
            }
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), true.into());
            return 0.into();
        }
    }

    if VarModule::is_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_STEP_2S_CHECK_HOLD) {
        VarModule::off_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_STEP_2S_CHECK_HOLD);
        if VarModule::is_flag(fighter.module_accessor, vars::demon::status::flag::ATTACK_STEP_2S_HOLD) {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("attack_step_2s_h"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, demon_attack_step_2s_main);
}