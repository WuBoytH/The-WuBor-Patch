use crate::imports::acmd_imports::*;
use super::super::vl;

unsafe extern "C" fn samusd_specialnstart(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 23.0 / 14.0);
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let cshot_id = VarModule::get_int(agent.module_accessor, samusd::instance::int::CSHOT_ID);
        if sv_battle_object::is_active(cshot_id as u32) {
            let boma = sv_battle_object::module_accessor(cshot_id as u32);
            if utility::get_category(&mut *boma) == *BATTLE_OBJECT_CATEGORY_WEAPON
            && utility::get_kind(&mut *boma) == *WEAPON_KIND_SAMUSD_CSHOT {
                WorkModule::set_int(boma, 1, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
            }
        }
    }
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_CSHOT, false, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_N_FLAG_BULLET_DISP);
    }
}

unsafe extern "C" fn samusd_special(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.667);
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
}

unsafe extern "C" fn samusd_specialair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.667);
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn samusd_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.667);
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
}

unsafe extern "C" fn samusd_specialairs(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.667);
    frame(agent.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn samusd_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 0.0, -5.0, Some(0.0), Some(0.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 92, 100, 200, 0, 3.2, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 3.0, 105, 100, 200, 0, 3.2, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 92, 100, 180, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 98, 100, 180, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 92, 100, 80, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 98, 100, 80, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 84, 100, 100, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 90, 100, 100, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 84, 100, 40, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 82, 100, 40, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 90, 100, 40, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 82, 100, 20, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 90, 100, 20, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 70, 190, 0, 56, 10.0, 0.0, 6.5, 0.0, None, None, None, 2.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUSD_SCREW_FINISH, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn samusd_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 93, 100, 115, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 96, 100, 115, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 93, 100, 60, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 96, 100, 60, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 96, 100, 70, 0, 3.2, 0.0, 2.0, 5.0, Some(0.0), Some(2.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.0, 98, 100, 70, 0, 3.2, 0.0, 2.0, -5.0, Some(0.0), Some(2.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 1.0, 96, 100, 40, 0, 3.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 1.0, 98, 100, 40, 0, 3.0, 0.0, 9.0, -5.0, Some(0.0), Some(9.0), Some(-5.0), 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 3, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.0, 70, 240, 0, 50, 10.0, 0.0, 6.5, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SAMUSD_SCREW, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn samusd_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samusd_special_l01"));
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(
            agent,
            0,
            vl::param_special_lw::start_jump_speed_y,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
        );
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 33.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 45, 30, 0, 70, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(
            agent,
            0,
            -vl::param_special_lw::fall_speed_start_y,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
        );
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
}

unsafe extern "C" fn samusd_specialairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_samusd_special_l01"));
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(
            agent,
            0,
            vl::param_special_lw::start_jump_speed_y,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
        );
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    frame(agent.lua_state_agent, 33.0);
    macros::FT_MOTION_RATE(agent, 2.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 45, 30, 0, 70, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(
            agent,
            0,
            -vl::param_special_lw::fall_speed_start_y,
            *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
        );
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        VisibilityModule::set_int64(agent.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
    macros::FT_MOTION_RATE(agent, 0.6);
}

unsafe extern "C" fn samusd_speciallw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_bomb_jump"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.48, true);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("samusd_dash_attack"), Hash40::new("top"), 0, 0, 0, 90, 0, 0, 1, true);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_specialnstart", samusd_specialnstart);

    agent.game_acmd("game_specialairnstart", samusd_specialnstart);

    agent.game_acmd("game_special", samusd_special);

    agent.game_acmd("game_specialair", samusd_specialair);

    agent.game_acmd("game_specials", samusd_specials);

    agent.game_acmd("game_specialairs", samusd_specialairs);

    agent.game_acmd("game_specialhi", samusd_specialhi);

    agent.game_acmd("game_specialairhi", samusd_specialairhi);

    agent.game_acmd("game_speciallw", samusd_speciallw);
    agent.effect_acmd("effect_speciallw", samusd_speciallw_eff);

    agent.game_acmd("game_specialairlw", samusd_specialairlw);
    agent.effect_acmd("effect_specialairlw", samusd_speciallw_eff);
}