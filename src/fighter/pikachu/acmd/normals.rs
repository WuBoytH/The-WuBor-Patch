use crate::imports::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.2, 0, 30, 0, 20, 3.5, 0.0, 3.0, 12.1, Some(0.0), Some(3.0), Some(7.7), 1.0, 1.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 3.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(agent.module_accessor, Hash40::new("attack_11"), true);
    frame(agent.lua_state_agent, game_frame);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("tail1"), 5.0, 107, 120, 0, 45, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("tail3"), 5.0, 107, 120, 0, 45, 3.8, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("tail4"), 5.0, 107, 120, 0, 45, 3.5, 0.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 70, 0, 50, 2.5, 0.0, 7.0, 9.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 70, 0, 50, 2.5, 0.0, 7.0, 11.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 70, 0, 50, 3.6, 0.0, 7.0, 14.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 45, 70, 0, 50, 3.6, 0.0, 7.0, 18.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 45, 75, 0, 60, 5.2, 0.0, 7.0, 23.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 45, 60, 0, 40, 4.0, 0.0, 7.0, 25.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attack11", game_attack11);

    agent.acmd("game_attackhi3", game_attackhi3);

    agent.acmd("game_attacks4", game_attacks4);
}