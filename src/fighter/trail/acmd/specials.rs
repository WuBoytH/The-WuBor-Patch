use crate::imports::*;

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_HI_FLAG_JUMP_START);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.8, 86, 100, 116, 0, 4.2, 0.0, 4.2, 7.6, Some(0.0), Some(4.2), Some(7.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.8, 88, 100, 82, 0, 4.2, 0.0, 12.6, 7.6, Some(0.0), Some(12.6), Some(7.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.8, 110, 100, 120, 0, 4.2, 0.0, 4.2, 16.8, Some(0.0), Some(4.2), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 3.8, 108, 100, 86, 0, 4.2, 0.0, 12.6, 16.8, Some(0.0), Some(12.6), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 3.8, 94, 100, 130, 0, 4.2, 0.0, 4.2, 7.6, Some(0.0), Some(4.2), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 3.8, 92, 100, 76, 0, 4.2, 0.0, 12.6, 7.6, Some(0.0), Some(12.6), Some(16.8), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(agent.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 3, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 4, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 80, 100, 124, 0, 2.4, 0.0, 7.6, -6.6, Some(0.0), Some(7.6), Some(-6.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 82, 100, 70, 0, 2.4, 0.0, 12.2, -6.6, Some(0.0), Some(12.2), Some(-6.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 106, 100, 122, 0, 2.4, 0.0, 7.6, -12.2, Some(0.0), Some(7.6), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 108, 100, 78, 0, 2.4, 0.0, 12.2, -12.2, Some(0.0), Some(12.2), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 94, 100, 114, 0, 2.4, 0.0, 7.6, -6.6, Some(0.0), Some(7.6), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 92, 100, 68, 0, 2.4, 0.0, 12.2, -6.6, Some(0.0), Some(12.2), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 3, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 4, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 76, 100, 92, 0, 2.8, 0.0, 4.6, 6.4, Some(0.0), Some(4.6), Some(6.4), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 78, 100, 62, 0, 2.8, 0.0, 10.0, 6.4, Some(0.0), Some(10.0), Some(6.4), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 108, 100, 100, 0, 2.8, 0.0, 4.6, 14.6, Some(0.0), Some(4.6), Some(14.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 106, 100, 72, 0, 2.8, 0.0, 10.0, 14.6, Some(0.0), Some(10.0), Some(14.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 92, 100, 110, 0, 2.8, 0.0, 4.6, 6.4, Some(0.0), Some(4.6), Some(14.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 90, 100, 58, 0, 2.8, 0.0, 10.0, 6.4, Some(0.0), Some(10.0), Some(14.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 3, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 4, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 88, 100, 92, 0, 2.4, 0.0, 8.8, -6.6, Some(0.0), Some(8.8), Some(-6.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 84, 100, 46, 0, 2.4, 0.0, 13.4, -6.6, Some(0.0), Some(13.4), Some(-6.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 102, 100, 88, 0, 2.4, 0.0, 8.8, -12.2, Some(0.0), Some(8.8), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 106, 100, 48, 0, 2.4, 0.0, 13.4, -12.2, Some(0.0), Some(13.4), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 92, 100, 90, 0, 2.4, 0.0, 8.8, -6.6, Some(0.0), Some(8.8), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 90, 100, 48, 0, 2.4, 0.0, 13.4, -6.6, Some(0.0), Some(13.4), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 3, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 4, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 74, 100, 90, 0, 2.8, 0.0, 7.2, 6.4, Some(0.0), Some(7.2), Some(6.4), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 72, 100, 72, 0, 2.8, 0.0, 12.6, 6.4, Some(0.0), Some(12.6), Some(6.4), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 98, 100, 92, 0, 2.8, 0.0, 7.2, 14.6, Some(0.0), Some(7.2), Some(14.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 96, 100, 72, 0, 2.8, 0.0, 12.6, 14.6, Some(0.0), Some(12.6), Some(14.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 94, 100, 102, 0, 2.8, 0.0, 7.2, 6.4, Some(0.0), Some(7.2), Some(14.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 92, 100, 62, 0, 2.8, 0.0, 12.6, 6.4, Some(0.0), Some(12.6), Some(14.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 3, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 4, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.1, 125, 100, 130, 0, 2.4, 0.0, 8.2, -6.6, Some(0.0), Some(8.2), Some(-6.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.1, 150, 100, 110, 0, 2.4, 0.0, 12.8, -6.6, Some(0.0), Some(12.8), Some(-6.6), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.1, 130, 100, 150, 0, 2.4, 0.0, 8.2, -12.2, Some(0.0), Some(8.2), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.1, 150, 100, 150, 0, 2.4, 0.0, 12.8, -12.2, Some(0.0), Some(12.8), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.1, 130, 100, 130, 0, 2.4, 0.0, 8.2, -6.6, Some(0.0), Some(8.2), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 2.1, 150, 100, 130, 0, 2.4, 0.0, 12.8, -6.6, Some(0.0), Some(12.8), Some(-12.2), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        AttackModule::set_no_finish_camera(agent.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 1, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 2, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 3, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 4, true, false);
        AttackModule::set_no_finish_camera(agent.module_accessor, 5, true, false);
        AttackModule::set_no_damage_fly_smoke_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.6, 62, 176, 0, 44, 3.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 4.6, 62, 176, 0, 44, 3.6, 0.0, 4.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 4.6, 62, 176, 0, 44, 3.6, 0.0, 9.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.6, 62, 176, 0, 44, 5.2, 0.0, 7.4, 8.4, Some(0.0), Some(9.6), Some(8.4), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 4.6, 62, 176, 0, 44, 4.8, 0.0, 12.6, 15.6, Some(0.0), Some(14.8), Some(15.6), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TRAIL_SLASH, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_HI_FLAG_COMMAND_ACCEPT);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_HI_FLAG_COMMAND_ACCEPT);
    }
    frame(agent.lua_state_agent, 75.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 79.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::set_float(agent.module_accessor, 0.0, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLOAT_ATTACK_POWER);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 4.6, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 9.2, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 100, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(9.0), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 2, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 3, 10.0, false);
        AttackModule::set_force_reaction(agent.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(agent.module_accessor, 3, true, false);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLAG_ATTACK_END);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 80, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(13.0), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 80, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(16.5), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhi", game_specialhi);

    agent.acmd("game_specialairhi", game_specialhi);

    agent.acmd("game_speciallw", game_speciallw);

    agent.acmd("game_specialairlw", game_speciallw);
}