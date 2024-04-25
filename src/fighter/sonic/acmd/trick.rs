use crate::imports::*;

unsafe extern "C" fn game_trickhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_MOVEMENT);
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_CONTROL);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 14.0, 15.0);
}

unsafe extern "C" fn game_trickf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_MOVEMENT);
        macros::ATTACK(agent, 0, 0, Hash40::new("legr"), 10.0, 25, 100, 0, 60, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneer"), 10.0, 25, 100, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("footr"), 10.0, 25, 100, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_CONTROL);
    }
}

unsafe extern "C" fn game_trickb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_MOVEMENT);
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_CONTROL);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 6.0, 5.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_trickhi", game_trickhi, Priority::Low);

    agent.acmd("game_trickf", game_trickf, Priority::Low);

    agent.acmd("game_trickb", game_trickb, Priority::Low);
}