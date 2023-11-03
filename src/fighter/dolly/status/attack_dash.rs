use crate::imports::status_imports::*;
use super::super::{vl, helper::*};

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn dolly_attackdash_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_command = VarModule::is_flag(fighter.module_accessor, dolly::status::flag::ATTACK_DASH_COMMAND);
    let is_cancel = VarModule::is_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
    fighter.status_pre_AttackDash();
    VarModule::set_flag(fighter.module_accessor, dolly::status::flag::ATTACK_DASH_COMMAND, is_command);
    VarModule::set_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn dolly_attackdash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, dolly::status::flag::ATTACK_DASH_COMMAND) {
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
    if VarModule::is_flag(fighter.module_accessor, dolly::status::flag::ATTACK_DASH_COMMAND) {
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
        let log = fighter.status_attack()["log_infos"]["attack_dash"].get_int();
        WorkModule::set_int64(fighter.module_accessor, log as i64, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_LOG_ATTACK_KIND);
        if !StopModule::is_stop(fighter.module_accessor) {
            fighter.sub_attack_dash_uniq(false.into());
        }
        fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_attack_dash_uniq as *const () as _));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(dolly_attackdash_main_loop as *const () as _))
}

unsafe extern "C" fn dolly_attackdash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, dolly::instance::flag::RISING_FORCE) {
        if dolly_hit_cancel(fighter).get_i32() == 1 {
            return 1.into();
        }
    }
    else if !VarModule::is_flag(fighter.module_accessor, dolly::status::flag::ATTACK_DASH_COMMAND) {
        if dolly_hit_cancel(fighter).get_i32() == 1
        || dolly_attack_start_cancel(fighter).get_i32() == 1 {
            return 1.into();
        }
    }
    fighter.status_AttackDash_Main();
    0.into()
}

#[status_script(agent = "dolly", status = FIGHTER_STATUS_KIND_ATTACK_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn dolly_attackdash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, dolly::status::flag::ATTACK_DASH_COMMAND) {
        VarModule::off_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
        VarModule::off_flag(fighter.module_accessor, dolly::status::flag::ATTACK_DASH_COMMAND);
    }
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if ![
        *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_HI_COMMAND
    ].contains(&status) {
        if VarModule::is_flag(fighter.module_accessor, dolly::instance::flag::RISING_FORCE) {
            EffectModule::clear_screen(fighter.module_accessor, 1, 0x14);
        }
        VarModule::off_flag(fighter.module_accessor, dolly::instance::flag::RISING_FORCE);
    }
    fighter.status_end_AttackDash();
    0.into()
}

pub fn install() {
    install_status_scripts!(
        dolly_attackdash_pre,
        dolly_attackdash_main,
        dolly_attackdash_end
    );
}