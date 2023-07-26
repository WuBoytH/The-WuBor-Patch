use crate::imports::acmd_imports::*;

#[acmd_script( agent = "dolly", script = "game_catchattack", category = ACMD_GAME, low_priority )]
unsafe fn dolly_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.2, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "dolly", script = "effect_catchattack", category = ACMD_EFFECT, low_priority )]
unsafe fn dolly_catchattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("dolly_attack_arc2"), Hash40::new("dolly_attack_arc2"), Hash40::new("top"), -2, 11.5, 0.3, -69, -97, 106, 0.63, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.6);
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "dolly", script = "sound_catchattack", category = ACMD_SOUND, low_priority )]
unsafe fn dolly_catchattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_swing_s"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        dolly_catchattack, dolly_catchattack_eff, dolly_catchattack_snd
    );
}