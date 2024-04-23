use crate::imports::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("arml"), 2.0, 361, 100, 30, 0, 3.0, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("arml"), 2.0, 361, 100, 30, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("shoulderl"), 2.0, 361, 100, 30, 0, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    // macros::FT_MOTION_RATE(agent, 0.5);
    // frame(agent.lua_state_agent, 3.0);
    // macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 100, 30, 0, 3.0, 0.0, 8.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 2.0, 361, 100, 30, 0, 3.0, 0.0, 8.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 2.0, 361, 100, 30, 0, 4.0, 0.0, 8.0, 13.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_close"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}

unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
        if macros::is_excute(agent) {
            ArticleModule::add_motion_partial(agent.module_accessor, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD, *WEAPON_ELIGHT_ESWORD_MOTION_PART_SET_KIND_OPEM_CLOSE, Hash40::new("to_open"), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(agent.module_accessor) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_ELIGHT_INSTANCE_WORK_ID_FLAG_ADD_PARTIAL_MTION_SWORD_WHEN_CHANGEING);
        }
    }
    // macros::FT_MOTION_RATE(agent, 0.5);
    // frame(agent.lua_state_agent, 3.0);
    // macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 3.0, -2.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 2.5, -1.5, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 60, 98, 0, 66, 3.0, 0.0, 8.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 3.0, 0.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 2.5, 0.0, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 60, 98, 0, 66, 3.0, 0.0, 15.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 3.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 3.0, 2.0, 8.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 4.0, 60, 98, 0, 66, 2.5, 1.5, 13.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 60, 98, 0, 66, 3.0, 0.0, 16.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::IS_EXIST_ARTICLE(agent, *FIGHTER_ELIGHT_GENERATE_ARTICLE_ESWORD) {
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
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_attack100end(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, fighter::status::flag::SPECIAL_CANCEL);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 40, 90, 0, 80, 8.0, 0.0, 10.0, 12.0, Some(0.0), Some(10.0), Some(18.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, fighter::status::flag::SPECIAL_CANCEL);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 10.0);
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
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 361, 54, 0, 72, 5.0, 0.0, 7.0, 13.0, Some(0.0), Some(7.0), Some(17.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.5, 361, 55, 0, 72, 5.0, 0.0, 7.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 29.0);
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
}

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
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
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 5.5, 62, 65, 0, 52, 3.0, -1.5, 4.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 4.5, 62, 65, 0, 52, 3.0, -1.5, 8.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 4.5, 62, 65, 0, 52, 2.5, -1.5, 12.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.5, 62, 65, 0, 52, 3.0, 0.0, 10.0, 6.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 5.5, 62, 65, 0, 52, 3.5, 2.0, 6.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 4.5, 62, 65, 0, 52, 3.5, 2.0, 10.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 4.5, 62, 65, 0, 52, 3.0, 2.0, 13.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 5.5, 62, 65, 0, 52, 3.0, 0.0, 5.0, 6.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 22.0);
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
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
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
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.6, 0.0, 17.0, -7.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.4, 0.0, 15.0, -11.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.0, 0.0, 13.0, -14.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.2, 0.0, 18.0, -3.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.6, 0.0, 21.0, -4.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.4, 0.0, 23.0, -8.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.0, 0.0, 25.0, -11.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.2, 0.0, 19.0, 0.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.6, 0.0, 22.0, 3.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.4, 0.0, 25.0, 2.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.0, 0.0, 28.0, 1.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.2, 0.0, 17.0, 4.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.6, 0.0, 17.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.4, 0.0, 20.0, 8.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.0, 0.0, 24.0, 9.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.2, 0.0, 12.0, 5.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.6, 0.0, 11.0, 10.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.4, 0.0, 14.0, 12.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.0, 0.0, 16.0, 14.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.2, 0.0, 9.0, 5.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.6, 0.0, 5.0, 7.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.4, 0.0, 5.0, 11.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 96, 73, 0, 60, 2.0, 0.0, 5.0, 14.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 96, 73, 0, 60, 2.2, 0.0, 5.0, 5.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    macros::FT_MOTION_RATE(agent, 0.667);
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
    frame(agent.lua_state_agent, 38.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
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
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 93, 60, 0, 67, 3.0, 0.0, 2.0, 8.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 87, 55, 0, 60, 2.5, 0.0, 2.0, 13.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 87, 55, 0, 60, 2.0, 0.0, 2.0, 16.5, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 4.0, 93, 60, 0, 67, 3.0, 0.0, 2.0, 4.0, None, None, None, 0.75, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 10.0);
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
    frame(agent.lua_state_agent, 14.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attack11", game_attack11, Priority::Low);

    agent.acmd("game_attack12", game_attack12, Priority::Low);

    agent.acmd("game_attack13", game_attack13, Priority::Low);

    agent.acmd("game_attack100end", game_attack100end, Priority::Low);

    agent.acmd("game_attackdash", game_attackdash, Priority::Low);

    agent.acmd("game_attacks3", game_attacks3, Priority::Low);

    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);

    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
}