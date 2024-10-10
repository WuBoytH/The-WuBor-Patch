use super::*;
use super::super::helper::*;

unsafe extern "C" fn dolly_attack_dash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_AttackDash()
}

unsafe extern "C" fn dolly_attack_dash_command_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::ATTACK_DASH_COMMAND);
    let special_command_lr = ControlModule::get_special_command_lr(fighter.module_accessor, 1);
    if special_command_lr != 0.0 && PostureModule::lr(fighter.module_accessor) != special_command_lr {
        PostureModule::set_lr(fighter.module_accessor, special_command_lr);
        PostureModule::update_rot_y_lr(fighter.module_accessor);
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            ENERGY_MOTION_RESET_TYPE_GROUND_TRANS,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
    }
    dolly_attack_dash_main(fighter)
}

unsafe extern "C" fn dolly_attack_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL) {
        VarModule::on_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL);
    }
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
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_ATTACK_DASH {
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
        let log = fighter.status_attack()["log_infos"]["attack_dash"].get_int();
        WorkModule::set_int64(fighter.module_accessor, log as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_attack_dash_uniq(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attack_dash_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attack_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::RISING_FORCE) {
        if dolly_hit_cancel(fighter).get_i32() != 0 {
            return 1.into();
        }
    }
    else {
        if dolly_hit_cancel(fighter).get_i32() != 0{
            return 1.into();
        }
        if !VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::ATTACK_DASH_COMMAND) {
            if dolly_attack_start_cancel(fighter).get_i32() != 0 {
                return 1.into();
            }
        }
        
    }
    fighter.status_AttackDash_Main();
    0.into()
}

unsafe extern "C" fn dolly_attack_dash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, vars::dolly::status::flag::ATTACK_DASH_COMMAND) {
        VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::IS_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.module_accessor, vars::dolly::status::flag::ATTACK_DASH_COMMAND);
    }
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if ![
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND
    ].contains(&status) {
        if VarModule::is_flag(fighter.module_accessor, vars::dolly::instance::flag::RISING_FORCE) {
            EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
        }
        VarModule::off_flag(fighter.module_accessor, vars::dolly::instance::flag::RISING_FORCE);
    }
    if [
        *FIGHTER_STATUS_KIND_SPECIAL_N,
        *FIGHTER_STATUS_KIND_SPECIAL_S,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND,
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND,
        vars::dolly::status::SPECIAL_N_COMMAND
    ].contains(&status) {
        sv_kinetic_energy!(
            mul_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_MOTION,
            0.4
        );
    }
    fighter.status_end_AttackDash();
    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *FIGHTER_STATUS_KIND_ATTACK_DASH, dolly_attack_dash_main);
    agent.status(End, *FIGHTER_STATUS_KIND_ATTACK_DASH, dolly_attack_dash_end);

    agent.status(Pre, vars::dolly::status::ATTACK_DASH_COMMAND, dolly_attack_dash_pre);
    agent.status(Main, vars::dolly::status::ATTACK_DASH_COMMAND, dolly_attack_dash_command_main);
    agent.status(End, vars::dolly::status::ATTACK_DASH_COMMAND, dolly_attack_dash_end);
}