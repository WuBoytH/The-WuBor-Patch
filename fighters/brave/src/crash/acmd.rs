use super::*;

unsafe extern "C" fn game_crash2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::disable_tip(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 1, Hash40::new("top"), 0.1, 366, 50, 0, 10, 55.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 3, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 999.0, 60, 100, 10, 30, 30.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 499.5, 60, 100, 10, 30, 55.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_MAGIC);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_crash2", game_crash2, Priority::Low);
}