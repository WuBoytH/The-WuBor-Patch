use crate::imports::*;

unsafe extern "C" fn game_specialsride(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 10, 3.0, 0.0, 8.0, 4.5, Some(0.0), Some(4.0), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    macros::FT_MOTION_RATE(agent, 1.25);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairsride(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    macros::FT_MOTION_RATE(agent, 1.25);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 5.0, 70, 100, 0, 30, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("head"), 3.0, 70, 100, 0, 30, 1.7, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PICKEL_STATUS_SPECIAL_HI_FLAG_ENABLE_CANCEL);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialsride", game_specialsride);

    agent.acmd("game_specialairsride", game_specialairsride);

    agent.acmd("game_specialairhi", game_specialairhi);
}