use super::*;

unsafe extern "C" fn game_pillar(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 68, 30, 0, 10, 6.5, 0.0, 3.1, 2.0, None, None, None, 0.33, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 68, 30, 0, 10, 4.5, 0.0, 9.6, 2.0, None, None, None, 0.33, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

unsafe extern "C" fn game_pillarair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 58, 45, 0, 16, 6.5, 0.0, 3.1, 2.0, None, None, None, 0.33, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 58, 45, 0, 16, 4.5, 0.0, 9.6, 2.0, None, None, None, 0.33, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PSI);
        macros::AREA_WIND_2ND_RAD_arg9(agent, 0, 1, 0.05, 200, 0.6, 0, 10, 20, 60);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_pillar", game_pillar, Priority::Low);

    agent.acmd("game_pillarair", game_pillarair, Priority::Low);
}