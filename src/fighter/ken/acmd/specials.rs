use crate::imports::acmd_imports::*;

#[acmd_script( agent = "ken", scripts = [ "game_specialn", "game_specialairn" ], category = ACMD_GAME, low_priority )]
unsafe fn ken_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ken::status::flag::SPECIAL_DECIDE_STRENGTH);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        macros::FT_MOTION_RATE(agent, 8.0 / 5.0);
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        macros::FT_MOTION_RATE(agent, 6.0 / 5.0);
    }
    else {
        macros::FT_MOTION_RATE(agent, 4.0 / 5.0);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SHOOT);
    }
    let strength = WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    if strength == *FIGHTER_RYU_STRENGTH_W {
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -12.0);
    }
    else if strength == *FIGHTER_RYU_STRENGTH_M {
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -10.0);
    }
    else {
        MiscModule::calc_motion_rate_from_cancel_frame(agent, 13.0, -8.0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_N_FLAG_SPECIAL_FALL);
    }
}

#[acmd_script( agent = "ken_hadoken", scripts = [ "game_movew", "game_movem", "game_moves" ], category = ACMD_GAME, low_priority )]
unsafe fn ken_hadoken_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 10, 0, 65, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::enable_safe_pos(agent.module_accessor);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.2);
    }
}

#[acmd_script( agent = "ken", scripts = [ "game_speciallwstart", "game_specialairlwstart" ], category = ACMD_GAME, low_priority )]
unsafe fn ken_speciallwstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ken::status::flag::SPECIAL_LW_ENABLE_ACTION);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ken::status::flag::SPECIAL_LW_UNABLE_ACTION);
        VarModule::on_flag(agent.battle_object, ken::status::flag::SPECIAL_LW_RESET_GRAVITY);
    }
}

#[acmd_script( agent = "ken", scripts = [ "effect_speciallwstart", "effect_specialairlwstart" ], category = ACMD_EFFECT, low_priority )]
unsafe fn ken_speciallwstart_eff(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "ken", scripts = [ "sound_speciallwstart", "sound_specialairlwstart" ], category = ACMD_SOUND, low_priority )]
unsafe fn ken_speciallwstart_snd(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "ken", scripts = [ "expression_speciallwstart", "expression_specialairlwstart" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn ken_speciallwstart_exp(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "ken", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn ken_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ken::status::flag::SPECIAL_LW_RESET_GRAVITY);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 45, 10, 0, 70, 3.5, 0.0, 9.5, 10.0, Some(0.0), Some(9.5), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 15, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "ken", scripts = [ "effect_speciallw", "effect_specialairlw" ], category = ACMD_EFFECT, low_priority )]
unsafe fn ken_speciallw_eff(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "ken", scripts = [ "sound_speciallw", "sound_specialairlw" ], category = ACMD_SOUND, low_priority )]
unsafe fn ken_speciallw_snd(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "ken", scripts = [ "expression_speciallw", "expression_specialairlw" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn ken_speciallw_exp(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "ken", scripts = [ "game_speciallwstepf", "game_specialairlwstepf" ], category = ACMD_GAME, low_priority )]
unsafe fn ken_speciallwstepf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, ken::status::flag::SPECIAL_LW_RESET_GRAVITY);
    }
}

#[acmd_script( agent = "ken", scripts = [ "effect_speciallwstepf", "effect_specialairlwstepf" ], category = ACMD_EFFECT, low_priority )]
unsafe fn ken_speciallwstepf_eff(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "ken", scripts = [ "sound_speciallwstepf", "sound_specialairlwstepf" ], category = ACMD_SOUND, low_priority )]
unsafe fn ken_speciallwstepf_snd(agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "ken", scripts = [ "expression_speciallwstepf", "expression_specialairlwstepf" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn ken_speciallwstepf_exp(agent: &mut L2CAgentBase) {
}

pub fn install() {
    install_acmd_scripts!(
        ken_specialn,
        ken_hadoken_move,

        ken_speciallwstart,
        ken_speciallwstart_eff,
        ken_speciallwstart_snd,
        ken_speciallwstart_exp,

        ken_speciallw,
        ken_speciallw_eff,
        ken_speciallw_snd,
        ken_speciallw_exp,

        ken_speciallwstepf,
        ken_speciallwstepf_eff,
        ken_speciallwstepf_snd,
        ken_speciallwstepf_exp
    );
}