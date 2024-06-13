use super::*;

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 11.0);
    for _ in 0..9 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("footl"), 0.3, 367, 20, 0, 20, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.2, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(agent, 1, 1, Hash40::new("footr"), 0.4, 367, 20, 0, 20, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        }
        wait(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 57.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 2, Hash40::new("footl"), 6.5, 44, 60, 0, 60, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.2, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 2, Hash40::new("footr"), 6.5, 44, 60, 0, 60, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 2, Hash40::new("footl"), 6.5, 270, 60, 0, 80, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.2, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 2, Hash40::new("footr"), 6.5, 270, 60, 0, 80, 2.8, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_PART, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
}