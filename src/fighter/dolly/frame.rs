use {
    crate::imports::status_imports::*,
    crate::fighter::common::frame::common_fighter_frame,
    super::agent_init::*
};

// unsafe extern "C" fn dolly_reset_vars(fighter: &mut L2CFighterCommon) {
//     let status = fighter.global_table[STATUS_KIND].get_i32();
//     if [
//         *FIGHTER_STATUS_KIND_DEAD,
//         *FIGHTER_STATUS_KIND_REBIRTH
//     ].contains(&status)  {
//         VarModule::set_float(fighter.module_accessor, dolly::instance::float::GO_METER, 0.0);
//     }
// }

unsafe extern "C" fn dolly_super_special_aura(fighter: &mut L2CFighterCommon) {
    if VarModule::get_float(fighter.module_accessor, dolly::instance::float::GO_METER) >= 200.0 {
        let eff = VarModule::get_int(fighter.module_accessor, dolly::instance::int::SUPER_SPECIAL_AURA) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("sys_aura_light"),
                Hash40::new("hip"),
                &ZERO_VECTOR,
                &ZERO_VECTOR,
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
            VarModule::set_int(fighter.module_accessor, dolly::instance::int::SUPER_SPECIAL_AURA, eff as i32);
        }
        let eff = VarModule::get_int(fighter.module_accessor, dolly::instance::int::SUPER_SPECIAL_AURA2) as u32;
        if !EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::req_follow(
                fighter.module_accessor,
                Hash40::new("sys_aura_light"),
                Hash40::new("hip"),
                &ZERO_VECTOR,
                &ZERO_VECTOR,
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
            VarModule::set_int(fighter.module_accessor, dolly::instance::int::SUPER_SPECIAL_AURA2, eff as i32);
        }
    }
    else {
        let eff = VarModule::get_int(fighter.module_accessor, dolly::instance::int::SUPER_SPECIAL_AURA) as u32;
        let eff2 = VarModule::get_int(fighter.module_accessor, dolly::instance::int::SUPER_SPECIAL_AURA2) as u32;
        if EffectModule::is_exist_effect(fighter.module_accessor, eff) {
            EffectModule::kill(fighter.module_accessor, eff, true, false);
            EffectModule::kill(fighter.module_accessor, eff2, true, false);
        }
    }
}

unsafe extern "C" fn dolly_super_super_cancels(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW
    && fighter.global_table[STATUS_FRAME].get_f32() < 8.0 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SUPER_SPECIAL);
        }
        if dolly_check_special_command(fighter).get_bool() {
            VarModule::off_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
        }
    }
}

unsafe extern "C" fn dolly_frame(fighter: &mut L2CFighterCommon) {
    // dolly_reset_vars(fighter);
    common_fighter_frame(fighter);
    dolly_super_special_aura(fighter);
    dolly_super_super_cancels(fighter);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, dolly_frame);
}