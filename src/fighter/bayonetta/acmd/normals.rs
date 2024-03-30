use crate::imports::*;

unsafe extern "C" fn bayonetta_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, false, 10, 3, 10, 5, true);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handl"), 1.5, 55, 25, 0, 45, 4.0, 0.0, 0.0, 1.5, Some(-6.0), Some(0.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("handl"), 1.5, 55, 25, 0, 30, 4.0, 0.0, 0.0, 1.5, Some(-6.0), Some(0.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("handl"), 1.5, 85, 25, 0, 42, 4.0, 6.0, 0.0, 1.5, Some(-6.0), Some(0.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn bayonetta_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, false, 10, 3, 10, 5, true);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("handr"), 1.5, 55, 25, 0, 45, 4.5, -4.0, -0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("handr"), 1.5, 74, 25, 0, 40, 4.0, 2.5, -0.5, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.5, 76, 25, 0, 40, 2.2, 0.0, 4.5, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    // wait(agent.lua_state_agent, 2.0);
    // macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn bayonetta_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 10, 5, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, false, 10);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.2, 68, 20, 0, 47, 2.0, 0.0, 6.0, 7.0, Some(0.0), Some(6.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.2, 85, 20, 0, 47, 2.0, 0.0, 6.0, 13.0, Some(0.0), Some(6.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.2, 68, 20, 0, 28, 4.5, 0.0, 14.0, 5.0, Some(0.0), Some(14.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.2, 90, 20, 0, 28, 4.5, 0.0, 14.0, 11.0, Some(0.0), Some(14.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 4.0, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.25);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}

unsafe extern "C" fn bayonetta_attack100(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, true, 10, 0, 3, 0, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), *FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, true, true, 10);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    }
    loop {
        frame(agent.lua_state_agent, 2.0);
        bayonetta_attack100_inner(agent);
        frame(agent.lua_state_agent, 6.0);
        bayonetta_attack100_inner(agent);
        frame(agent.lua_state_agent, 10.0);
        bayonetta_attack100_inner(agent);
        frame(agent.lua_state_agent, 14.0);
        bayonetta_attack100_inner(agent);
        macros::wait_loop_clear(agent);
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CHECK_COMBO_NUM);
        }
    }
}

#[inline(always)]
unsafe extern "C" fn bayonetta_attack100_inner(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.2, 361, 20, 0, 8, 6.7, 0.0, 9.0, 18.0, Some(0.0), Some(9.0), Some(18.0), 0.6, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.2, 368, 20, 0, 10, 6.7, 0.0, 9.0, 9.0, Some(0.0), Some(9.0), Some(9.0), 0.6, 0.1, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_vec_target_pos(agent.module_accessor, 1, Hash40::new("top"), &Vector2f{x: 14.0, y: 1.0}, 4, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 2.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(agent, 0, 1, 15.0);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
    }
}

unsafe extern "C" fn bayonetta_attack100end(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2bfb02b69a), true);
        macros::CORRECT(agent, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, false, false, 10, 3, 15, 5, true);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 90, 100, 30, 0, 3.0, 0.0, 7.0, 28.0, Some(0.0), Some(7.0), Some(10.0), 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 90, 100, 20, 0, 4.5, 0.0, 11.0, 27.0, Some(0.0), Some(11.0), Some(11.0), 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 43, 121, 0, 67, 6.5, 0.0, 8.5, 8.0, None, None, None, 1.8, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 43, 121, 0, 67, 6.5, 0.0, 8.5, 13.0, None, None, None, 1.8, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 43, 121, 0, 67, 6.5, 0.0, 8.5, 18.0, None, None, None, 1.8, 0.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, fighter::status::flag::DASH_CANCEL);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    // frame(agent.lua_state_agent, 28.0);
    // macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 0.85 as u64);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, fighter::status::flag::DASH_CANCEL);
    }
}

unsafe extern "C" fn bayonetta_attacks32(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, fighter::status::flag::JUMP_CANCEL);
        AttackModule::clear_inflict_kind_status(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, true, false, false, 10, 3, 10, 5, true);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, fighter::status::flag::JUMP_CANCEL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 48, 25, 0, 42, 6.5, 0.0, 13.5, 5.0, Some(0.0), Some(13.5), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 62, 25, 0, 46, 6.5, 0.0, 13.5, 13.0, Some(0.0), Some(13.5), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 0.85);
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 5.0);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, fighter::status::flag::JUMP_CANCEL);
    }
}

unsafe extern "C" fn bayonetta_attacks33(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, fighter::status::flag::JUMP_CANCEL);
        AttackModule::clear_inflict_kind_status(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10, 3, 10, 5, true);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, fighter::status::flag::JUMP_CANCEL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 81, 60, 0, 60, 8.0, 0.0, 16.0, 6.3, Some(0.0), Some(8.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 7.0, 81, 60, 0, 60, 8.0, 0.0, 26.0, 7.0, Some(0.0), Some(8.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    // macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 0.85 as u64);
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, fighter::status::flag::JUMP_CANCEL);
    }
}

unsafe extern "C" fn bayonetta_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), FIGHTER_BAYONETTA_SHOOTING_SLOT_L_LEG, true, false, false, 10, 3, 15, 5, true);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), FIGHTER_BAYONETTA_SHOOTING_SLOT_R_LEG, true, false, false, 10);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2b7cb92b79), FIGHTER_BAYONETTA_SHOOTING_SLOT_L_ARM, true, false, false, 10);
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 5.0, 6.5);
    }
    macros::FT_MOTION_RATE(agent, 5.0 / 6.0);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 50, 30, 0, 70, 2.8, 0.0, 4.5, 1.0, Some(0.0), Some(3.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    // frame(agent.lua_state_agent, 24.0);
    // frame(agent.lua_state_agent, 28.0);
    // macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 0.85 as u64);
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    }
}

unsafe extern "C" fn bayonetta_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    // if macros::is_excute(agent) {
    //     notify_event_msc_cmd!(agent, Hash40::new_raw(0x2d51fcdb09), *FIGHTER_BAYONETTA_SHOOTING_SLOT_R_ARM, true, true, false, 10, 3, 15, 5, true);
    // }
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.4);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 367, 80, 10, 0, 4.0, 0.0, 11.0, 9.0, Some(0.0), Some(5.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 367, 80, 10, 0, 4.0, 0.0, 11.0, 11.0, Some(0.0), Some(5.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 27.0);
    // frame(agent.lua_state_agent, 28.0);
    // macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 0.85 as u64);
    // frame(agent.lua_state_agent, 28.0);
    // if macros::is_excute(agent) {
    //     WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_ACTION);
    //     WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_MOTION_STOP);
    //     WorkModule::on_flag(agent.module_accessor, *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_CHECK_END);
    // }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attack11", bayonetta_attack11);

    agent.acmd("game_attack12", bayonetta_attack12);

    agent.acmd("game_attack13", bayonetta_attack13);

    agent.acmd("game_attack100", bayonetta_attack100);

    agent.acmd("game_attack100end", bayonetta_attack100end);

    agent.acmd("game_attacks32", bayonetta_attacks32);

    agent.acmd("game_attacks33", bayonetta_attacks33);

    agent.acmd("game_attacklw3", bayonetta_attacklw3);

    agent.acmd("game_attackdash", bayonetta_attackdash);
}