use super::*;

unsafe extern "C" fn game_rise(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 1, 0, 5, 12, 60);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 85, 100, 15, 0, 3.4, 0.0, 7.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.6, 0.0, 8, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 85, 100, 15, 0, 3.6, 0.0, 7.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.6, 0.0, 8, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}

unsafe extern "C" fn game_burn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 0.8, 0, 9, 15, 60);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 85, 100, 15, 0, 3.8, 0.0, 8.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.6, 0.0, 8, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 85, 100, 15, 0, 4.0, 0.0, 9.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.6, 0.0, 8, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.3, 110, 100, 15, 0, 4.0, 0.0, 9.0, 0.0, Some(0.0), Some(1.0), Some(0.0), 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -0.6, 0.0, 8, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
}

unsafe extern "C" fn game_vanish(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 2, 0.05, 200, 1, 0, 9, 22, 60);
        macros::ATTACK(agent, 1, 1, Hash40::new("top"), 4.0, 70, 110, 0, 80, 6.5, 0.0, 11.0, 0.0, Some(0.0), Some(3.0), Some(0.0), 2.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_rise", game_rise, Priority::Low);

    agent.acmd("game_burn", game_burn, Priority::Low);

    agent.acmd("game_vanish", game_vanish, Priority::Low);
}