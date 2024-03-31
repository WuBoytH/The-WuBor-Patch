use crate::imports::*;

unsafe extern "C" fn game_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.2);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 80, 100, 30, 0, 4.7, 0.0, 8.2, 8.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("pichu_elec_impact"), Hash40::new("top"), 0, 9, 9, 0, 90, 0, 0.7, true);
    }
}

unsafe extern "C" fn expression_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_catchattack", game_catchattack);
    agent.acmd("effect_catchattack", effect_catchattack);
    agent.acmd("expression_catchattack", expression_catchattack);
}