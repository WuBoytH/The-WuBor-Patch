use super::*;
use crate::{agent_init::*, helper::*};

extern "C" {
    #[link_name = "common_fighter_frame"]
    pub fn common_fighter_frame(fighter: &mut L2CFighterCommon);
}

// unsafe extern "C" fn dolly_reset_vars(fighter: &mut L2CFighterCommon) {
//     let status = fighter.global_table[STATUS_KIND].get_i32();
//     if [
//         *FIGHTER_STATUS_KIND_DEAD,
//         *FIGHTER_STATUS_KIND_REBIRTH
//     ].contains(&status)  {
//         VarModule::set_float(fighter.module_accessor, vars::dolly::instance::float::GO_METER, 0.0);
//     }
// }

unsafe extern "C" fn dolly_super_special_aura(fighter: &mut L2CFighterCommon) {
    if VarModule::get_float(fighter.module_accessor, vars::dolly::instance::float::GO_METER) >= 200.0 {
        let eff = VarModule::get_int(fighter.module_accessor, vars::dolly::instance::int::SUPER_SPECIAL_AURA) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("sys_aura_light"),
                Hash40::new("hip"),
                &vars::ZERO_VECTOR,
                &vars::ZERO_VECTOR,
                3.0,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            EffectModule::set_rgb(fighter.module_accessor, eff, 1.0, 0.6, 0.2);
            VarModule::set_int(fighter.module_accessor, vars::dolly::instance::int::SUPER_SPECIAL_AURA, eff as i32);
        }
        let eff = VarModule::get_int(fighter.module_accessor, vars::dolly::instance::int::SUPER_SPECIAL_AURA2) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("sys_aura_light"),
                Hash40::new("hip"),
                &vars::ZERO_VECTOR,
                &vars::ZERO_VECTOR,
                3.0,
                false,
                0,
                0,
                0,
                0,
                0,
                false,
                false
            );
            let eff = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            EffectModule::set_rgb(fighter.module_accessor, eff, 1.0, 0.6, 0.2);
            VarModule::set_int(fighter.module_accessor, vars::dolly::instance::int::SUPER_SPECIAL_AURA2, eff as i32);
        }
    }
    else {
        let eff = VarModule::get_int(fighter.module_accessor, vars::dolly::instance::int::SUPER_SPECIAL_AURA) as u32;
        let eff2 = VarModule::get_int(fighter.module_accessor, vars::dolly::instance::int::SUPER_SPECIAL_AURA2) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::kill(fighter.module_accessor, eff, true, false);
            EffectModule::kill(fighter.module_accessor, eff2, true, false);
        }
    }
}

unsafe extern "C" fn dolly_super_super_cancels(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW
    && !fighter.global_table[IS_STOP].get_bool()
    && fighter.global_table[STATUS_FRAME].get_f32() < 8.0 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
        }
        if dolly_check_special_command(fighter).get_bool() {
            VarModule::off_flag(fighter.module_accessor, vars::dolly::instance::flag::SPECIAL_CANCEL);
        }
    }
}

unsafe extern "C" fn dolly_scuffed_special_super_cancels(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND].get_i32() == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK {
        let situation = fighter.global_table[SITUATION_KIND].get_i32();
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL)
        && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT)
        && dolly_final_cancel(fighter, situation.into()).get_bool() {
            return;
        }
    }
}

unsafe extern "C" fn dolly_training_meter(fighter: &mut L2CFighterCommon) {
    if smashball::is_training_mode() {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            add_go(fighter.module_accessor, 100.0);
        }
    }
}

unsafe extern "C" fn autoturn_on_cancel_frame(fighter: &mut L2CFighterCommon) {
    let new_lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    let lr = PostureModule::lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND
    && CancelModule::is_enable_cancel(fighter.module_accessor)
    && new_lr != lr {
        let status = if [
            *FIGHTER_STATUS_KIND_ATTACK_LW3,
            *FIGHTER_STATUS_KIND_ATTACK_LW4
        ].contains(&StatusModule::status_kind(fighter.module_accessor)) {
            FIGHTER_DOLLY_STATUS_KIND_SQUAT_TURN_AUTO
        }
        else {
            FIGHTER_DOLLY_STATUS_KIND_TURN_AUTO
        };
        fighter.change_status(status.into(), false.into());
    }
}

unsafe extern "C" fn on_main(fighter: &mut L2CFighterCommon) {
    // dolly_reset_vars(fighter);
    common_fighter_frame(fighter);
    dolly_super_special_aura(fighter);
    dolly_super_super_cancels(fighter);
    dolly_scuffed_special_super_cancels(fighter);
    dolly_training_meter(fighter);
    autoturn_on_cancel_frame(fighter);
}

pub fn install(agent: &mut Agent) {
    agent.on_line(Main, on_main);
}