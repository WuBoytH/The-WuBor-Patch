use crate::imports::*;

unsafe extern "C" fn game_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 100, 30, 0, 5.5, 0.0, 6.5, 12.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 5, -2, -7, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.104, 0.024, 0.754);
    }
}

unsafe extern "C" fn sound_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_attack100_01"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catchattack", game_catchattack, Priority::Low);
    agent.acmd("effect_catchattack", effect_catchattack, Priority::Low);
    agent.acmd("sound_catchattack", sound_catchattack, Priority::Low);
}