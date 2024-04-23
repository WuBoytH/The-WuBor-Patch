use crate::imports::*;

// Crouch Jab

unsafe extern "C" fn game_attacksquat2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 77, 12, 0, 40, 2.0, 0.0, 9.0, 13.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 77, 12, 0, 40, 3.4, 0.0, 9.0, 12.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 77, 12, 0, 40, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 270, 12, 60, 0, 3.0, 0.0, 3.0, 5.0, Some(0.0), Some(3.0), Some(12.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::set_down_only(agent.module_accessor, 0, true);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 16.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 16.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 16.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 22.0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO_INPUT);
    }
}

// Tombstone Crusher

unsafe extern "C" fn game_attacksquat1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::HIT_NODE(agent, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 0, 0, 50, 2.5, 0.0, 10.5, 5.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 0, 0, 50, 4.0, 0.0, 10.5, 5.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 361, 0, 0, 50, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(10.5), Some(5.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 48, 64, 0, 70, 2.5, 0.0, 10.5, 5.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 9.0, 48, 64, 0, 70, 4.0, 0.0, 10.5, 5.5, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 9.0, 48, 64, 0, 70, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(10.5), Some(5.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 11.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 1.1);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 361, 0, 0, 50, 2.5, 0.0, 7.0, 10.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 0, 0, 50, 4.0, 0.0, 7.0, 10.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 9.0, 361, 0, 0, 50, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(7.0), Some(10.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 48, 64, 0, 70, 2.5, 0.0, 7.0, 10.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 9.0, 48, 64, 0, 70, 4.0, 0.0, 7.0, 10.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 9.0, 48, 64, 0, 70, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(7.0), Some(10.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 11.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 3, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 4, 1.1);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 5, 1.1);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::HIT_NODE(agent, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(agent, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attacksquat2", game_attacksquat2, Priority::Low);

    agent.acmd("game_attacksquat1", game_attacksquat1, Priority::Low);
}