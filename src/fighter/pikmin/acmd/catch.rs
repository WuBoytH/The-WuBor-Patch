use crate::imports::acmd_imports::*;

unsafe extern "C" fn pikmin_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.8, 361, 100, 0, 30, 7.0, 0.0, 8.0, 9.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn pikmin_catchattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 3, 1, -24, 0, 0, 0.6, false);
    }
}

unsafe extern "C" fn pikmin_catchattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_s"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_catchattack", pikmin_catchattack);
    agent.acmd("effect_catchattack", pikmin_catchattack_eff);
    agent.acmd("sound_catchattack", pikmin_catchattack_snd);
}