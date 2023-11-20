use crate::imports::status_imports::*;

unsafe extern "C" fn dolly_special_sb_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let is_cancel = VarModule::is_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    let original = smashline::original_status(smashline::Pre, fighter, status);
    if original(fighter).get_i32() == 1 {
        return 1.into();
    }
    VarModule::set_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL, is_cancel);
    0.into()
}

unsafe extern "C" fn dolly_special_s_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND_INTERRUPT].get_i32();
    if status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK
    && status != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK {
        VarModule::off_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

unsafe extern "C" fn dolly_special_f_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    VarModule::off_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_END {
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

unsafe extern "C" fn dolly_special_b_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() != *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_LANDING {
        VarModule::off_flag(fighter.module_accessor, dolly::status::flag::IS_SPECIAL_CANCEL);
        WorkModule::set_customize_no(fighter.module_accessor, 0, 1);
    }
    0.into()
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, dolly_special_sb_pre);
    agent.status(smashline::Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, dolly_special_sb_pre);
    agent.status(smashline::Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, dolly_special_sb_pre);
    agent.status(smashline::Pre, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, dolly_special_sb_pre);
    agent.status(smashline::End, *FIGHTER_STATUS_KIND_SPECIAL_S, dolly_special_s_end);
    agent.status(smashline::End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, dolly_special_s_end);

    agent.status(smashline::End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_F_ATTACK, dolly_special_f_attack_end);

    agent.status(smashline::End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B, dolly_special_s_end);
    agent.status(smashline::End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_COMMAND, dolly_special_s_end);

    agent.status(smashline::End, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_B_ATTACK, dolly_special_b_attack_end);
}