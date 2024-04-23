use crate::imports::*;

unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 3.7, 0.0, Some(0.0), Some(-3.7), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 0.0, 3.7, Some(0.0), Some(0.0), Some(-3.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn game_turn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 3.7, 0.0, Some(0.0), Some(-3.7), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("rot"), 6.0, 90, 20, 0, 75, 1.2, 0.0, 0.0, 3.7, Some(0.0), Some(0.0), Some(-3.7), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -6, -1.0, 24, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_RICHTER_CROSS, *ATTACK_REGION_OBJECT);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_fly", game_fly, Priority::Low);

    agent.acmd("game_turn", game_turn, Priority::Low);
}