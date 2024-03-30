use crate::imports::*;

unsafe extern "C" fn elight_attackairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.0, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 20, 0, 25, 3.0, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 15, 0, 25, 3.0, 0.0, 8.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 2.0, 75, 20, 0, 55, 3.0, 0.0, 8.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 4, 0, Hash40::new("top"), 2.0, 75, 20, 0, 55, 3.0, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 5, 0, Hash40::new("top"), 2.0, 75, 15, 0, 55, 3.0, 0.0, 8.0, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 2, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 3, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 4, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 5, 4.0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, fighter::status::flag::NORMAL_CANCEL);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, fighter::status::flag::NORMAL_CANCEL);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn elight_attackairn_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("elight_attack100_hand"), Hash40::new("handl"), 1, 0.7, -0.2, 0, 0, 0, 0.37, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("elight_attack_speedline"), Hash40::new("top"), 0, 10, 3, 8, 6, 0, 0.45, true, 0.6);
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("elight_attack100"), Hash40::new("top"), 0, 8.0, 11.5, 0, 0, 0, 0.2, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("elight_attack100_hand"), false, true);
    }
}

unsafe extern "C" fn elight_attackairn_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_elight_swing_s01"));
    }
}

unsafe extern "C" fn elight_attackairn_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn elight_landingairn(_agent: &mut L2CAgentBase) {
    
}

unsafe extern "C" fn elight_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 5.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 45, 65, 0, 62, 3.5, 2.0, 5.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 45, 65, 0, 62, 3.0, 2.0, 9.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 6.0, 45, 65, 0, 62, 2.5, 2.0, 11.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 7.0, 45, 65, 0, 62, 3.0, 2.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("haver"), 7.0, 45, 65, 0, 62, 2.5, 2.0, -4.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("haver"), 7.0, 45, 65, 0, 62, 2.5, 2.0, 6.0, -4.0, Some(2.0), Some(11.0), Some(-4.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 5, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 7.0, 45, 65, 0, 62, 3.5, -2.0, 5.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 6.0, 45, 65, 0, 62, 3.0, -2.0, 9.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 6.0, 45, 65, 0, 62, 2.5, -2.0, 11.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 7.0, 45, 65, 0, 62, 3.0, -2.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("haver"), 7.0, 45, 65, 0, 62, 2.5, -2.0, -4.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 4, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 13.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn elight_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 3.0, 0.0, 6.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 6.5, 72, 66, 0, 67, 2.5, 0.0, 9.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 6.5, 72, 66, 0, 67, 2.5, 0.0, 12.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 5.25, 0.0, 7.0, -5.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 4, false);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 3.0, 0.0, 6.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 6.5, 72, 66, 0, 67, 2.5, 0.0, 9.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 6.5, 72, 66, 0, 67, 2.5, 0.0, 12.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 7.5, 72, 66, 0, 67, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 18.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 79.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn elight_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 9.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 81, 60, 0, 90, 4.0, 0.0, 5.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 81, 60, 0, 90, 3.5, 0.0, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 5.0, 81, 60, 0, 90, 2.5, 0.0, 11.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 6.0, 81, 60, 0, 90, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 6.0, 81, 60, 0, 90, 4.0, -1.0, 5.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 5.0, 81, 60, 0, 90, 3.5, -2.0, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 5.0, 81, 60, 0, 90, 2.5, -3.0, 11.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("haver"), 6.0, 81, 60, 0, 90, 3.0, -1.0, 0.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 17.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 19.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 43.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn elight_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(agent, 11.0 / 8.0);
    frame(agent.lua_state_agent, 11.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 11.0 / 17.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 366, 20, 10, 60, 3.5, 0.0, 4.0, 8.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 366, 20, 10, 60, 2.5, 0.0, 0.0, 12.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 0, -2.0, false);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, -2.0, false);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 70, 20, 0, 75, 5.0, 0.0, 4.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.1);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK);
    }
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(agent.lua_state_agent, 45.0);
    if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 72.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn elight_attackairlw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("elight_sword_open"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_open"), true, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        macros::EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light_few"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("elight_attack100_finish"), Hash40::new("top"), 0, 4, 8, 0, 0, 0, 0.4, true);
        macros::LAST_EFFECT_SET_RATE(agent, 2.0);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light_few"), false, true);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_m"), true, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
        macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
    }
}

unsafe extern "C" fn elight_attackairlw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_elight_attackair_l01"));
        macros::PLAY_SE(agent, Hash40::new("vc_elight_attack01"));
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_elight_attack100_end"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_elight_rnd_attack04"));
    }
}

unsafe extern "C" fn elight_attackairlw_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_79_lightairslash"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohits"),
            3,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_79_slashlarge"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn elight_landingairlw(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
            if macros::is_excute(agent) {
                ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 10.0, 10.0, false, false, 0.0, false, true, false);
            }
        }
        if MotionModule::is_changing(agent.module_accessor) {
            if macros::is_excute(agent) {
                WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
            }
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 361, 40, 0, 70, 4.0, 0.0, 4.5, 9.0, Some(0.0), Some(4.5), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_SWORD);
            macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 0.1);
        }
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 15.0);
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if ArticleModule::is_exist(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
            if macros::is_excute(agent) {
                ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
            }
        }
        if MotionModule::is_changing(agent.module_accessor) {
            if macros::is_excute(agent) {
                WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
            }
        }
    }
}

unsafe extern "C" fn elight_landingairlw_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("elight_sword_beam_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
            macros::EFFECT_FOLLOW(agent, Hash40::new("elight_sword_light_few"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("elight_attack100_finish"), Hash40::new("top"), 0, 4.5, 8, 0, 0, 0, 0.35, true);
            macros::LAST_EFFECT_SET_RATE(agent, 2.0);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_light_few"), false, true);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if macros::is_excute(agent) {
            macros::EFFECT_OFF_KIND(agent, Hash40::new("elight_sword_beam_m"), true, true);
            macros::EFFECT_FOLLOW(agent, Hash40::new("elight_sword_close_m"), Hash40::new("sword1"), 0, 0, 0, 0, 90, 0, 1, true);
            macros::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(agent, -0.3);
        }
    }
}

unsafe extern "C" fn elight_landingairlw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_elight_landing02"));
    }
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_elight_attack100_end"));
            macros::PLAY_SEQUENCE(agent, Hash40::new("seq_elight_rnd_attack04"));
        }
    }
}

unsafe extern "C" fn elight_landingairlw_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_79_lands_light"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if VarModule::is_flag(agent.module_accessor, attack_air::flag::ENABLE_LANDING_ATTACK) {
        if macros::is_excute(agent) {
            macros::RUMBLE_HIT(agent, Hash40::new("rbkind_79_slashlarge"), 0);
        }
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 9);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attackairn", elight_attackairn);
    agent.acmd("effect_attackairn", elight_attackairn_eff);
    agent.acmd("sound_attackairn", elight_attackairn_snd);
    agent.acmd("expression_attackairn", elight_attackairn_exp);

    agent.acmd("game_landingairn", elight_landingairn);

    agent.acmd("game_attackairf", elight_attackairf);

    agent.acmd("game_attackairb", elight_attackairb);

    agent.acmd("game_attackairhi", elight_attackairhi);

    agent.acmd("game_attackairlw", elight_attackairlw);
    agent.acmd("effect_attackairlw", elight_attackairlw_eff);
    agent.acmd("sound_attackairlw", elight_attackairlw_snd);
    agent.acmd("expression_attackairlw", elight_attackairlw_exp);

    agent.acmd("game_landingairlw", elight_landingairlw);
    agent.acmd("effect_landingairlw", elight_landingairlw_eff);
    agent.acmd("sound_landingairlw", elight_landingairlw_snd);
    agent.acmd("expression_landingairlw", elight_landingairlw_exp);
}