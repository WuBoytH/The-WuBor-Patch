use crate::imports::acmd_imports::*;

#[acmd_script( agent = "ryu", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn ryu_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 75, 70, 0, 85, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ryu", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn ryu_specialairsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            let speedx = KineticModule::get_sum_speed_x(agent.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(agent.module_accessor);
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(agent, speedx, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 50, 0, 60, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 75, 70, 0, 85, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(agent.module_accessor, 0, 0.1);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ryu", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn ryu_specialairs(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::HIT_NODE(agent, Hash40::new("kneel"), *HIT_STATUS_XLU);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 50, 0, 60, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 55, 60, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 62, 67, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    else {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 62, 67, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_size(agent.module_accessor, 0, 0.1);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 50, 0, 60, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 50, 42, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.5, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    else {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 50, 56, 0, 80, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            AttackModule::set_target_category(agent.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_size(agent.module_accessor, 0, 0.1);
        }
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "game_specialhi", "game_specialhicommand" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 9.0, 70, 58, 40, 60, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 15.0, false);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 75, 64, 0, 70, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 13.0, 80, 69, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 8.0, 70, 58, 40, 60, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 15.0, false);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 10.0, 75, 58, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 12.0, 80, 58, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 7.0, 70, 90, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "ryu", scripts = [ "game_specialairhi", "game_specialairhicommand" ], category = ACMD_GAME, low_priority )]
unsafe fn ryu_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 80, 49, 0, 80, 5.0, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 54, 0, 80, 5.0, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 80, 54, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    wait(agent.lua_state_agent, 1.0);
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 7.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 9.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 11.0, 80, 57, 0, 80, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else if WorkModule::get_int(agent.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 70, 70, 0, 60, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 6.0, 70, 70, 0, 60, 6.0, 4.0, -0.4, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movew", category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadoken_movew(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 2.2);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.0, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 2.2);
    }
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movem", category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadoken_movem(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.5, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.5, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.5, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 2.2);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.5, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.5, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 7.5, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 2.2);
    }
}

#[acmd_script( agent = "ryu_hadoken", script = "game_moves", category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadoken_moves(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 2.2);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 2.2);
    }
}

#[acmd_script( agent = "ryu_hadoken", scripts = ["game_movespw_last", "game_movespm_last", "game_movesps_last"], category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadoken_movesp_last(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.3, 55, 60, 0, 60, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(agent, 0, 1, 2, 2.5);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ryu_specialsstart,
        ryu_specialairsstart,
        ryu_specialairs,
        ryu_specialhi,
        ryu_specialairhi,
        ryu_hadoken_movew,
        ryu_hadoken_movem,
        ryu_hadoken_moves,
        ryu_hadoken_movesp_last
    );
}