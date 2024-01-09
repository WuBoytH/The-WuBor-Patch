use crate::imports::acmd_imports::*;

unsafe extern "C" fn pzenigame_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 100, 30, 0, 5.0, 0.0, 8.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn pzenigame_catchattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 3, 10.5, -7, 30, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.4, 1, 0.5);
    }
}

unsafe extern "C" fn pzenigame_catchattack_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_catchattack", pzenigame_catchattack);
    agent.effect_acmd("effect_catchattack", pzenigame_catchattack_eff);
    agent.expression_acmd("expression_catchattack", pzenigame_catchattack_exp);
}