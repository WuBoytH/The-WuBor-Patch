use super::*;

unsafe extern "C" fn game_appealhir(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
    JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 14.0);
    for _ in 0..5 {
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.3);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.16, 367, 80, 45, 15, 5.5, 0.0, 5.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
}
}

unsafe extern "C" fn game_appealhil(agent: &mut L2CAgentBase) {

    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
    JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 14.0);
    for _ in 0..5 {
    if macros::is_excute(agent) {
        macros::FT_ADD_DAMAGE(agent, 0.3);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.16, 367, 80, 45, 15, 5.5, 0.0, 5.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    wait(agent.lua_state_agent, 1.0);
}
}

pub fn install(agent: &mut smashline::Agent) {

    agent.acmd("game_appealhir", game_appealhir, Priority::Low);
    agent.acmd("game_appealhil", game_appealhil, Priority::Low);
}