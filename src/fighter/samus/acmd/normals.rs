use crate::imports::acmd_imports::*;

#[acmd_script( agent = "samus", script = "game_attackhi3" , category = ACMD_GAME, low_priority )]
unsafe extern "C" fn samus_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 0, 70, 100, 6.0, 0.0, 20.0, 9.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 366, 0, 70, 60, 6.0, 0.0, 21.0, 8.0, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 55, 92, 0, 50, 6.0, 0.0, 22.0, 6.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 34.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

#[acmd_script( agent = "samus", script = "effect_attackhi3" , category = ACMD_EFFECT, low_priority )]
unsafe extern "C" fn samus_attackhi3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 21.0, 10.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -40, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 22.0, 9.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -44, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("samus_atk_bomb"), Hash40::new("top"), 0.0, 23.0, 7.0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, -3, -48, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "samus", script = "sound_attackhi3" , category = ACMD_SOUND, low_priority )]
unsafe extern "C" fn samus_attackhi3_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_smash_h01"));
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samus_smash_h02"));
    }
}

#[acmd_script( agent = "samus", script = "expression_attackhi3" , category = ACMD_EXPRESSION, low_priority )]
unsafe extern "C" fn samus_attackhi3_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_explosion"), 8);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohit_explosion"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_attackhi3", samus_attackhi3);
    agent.effect_acmd("effect_attackhi3", samus_attackhi3_eff);
    agent.sound_acmd("sound_attackhi3", samus_attackhi3_snd);
    agent.expression_acmd("expression_attackhi3", samus_attackhi3_exp);
}