use crate::imports::*;

unsafe extern "C" fn lucas_speciallwend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 85, 98, 40, 60, 3.5, 0.0, 6.3, 7.0, Some(0.0), Some(6.3), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PSI);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_speciallwend", lucas_speciallwend);

    agent.acmd("game_specialairlwend", lucas_speciallwend);
}