use crate::imports::*;

unsafe extern "C" fn game_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 70, 60, 0, 45, 5.0, 0.0, 12.0, 7.0, Some(0.0), Some(15.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 70, 60, 0, 45, 3.5, 0.0, 12.0, 11.0, Some(0.0), Some(15.0), Some(11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 70, 60, 0, 45, 2.0, 0.0, 12.0, 14.0, Some(0.0), Some(15.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 0, 2.75);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 1, 2.75);
        macros::ATK_SET_SHIELD_SETOFF_MUL(agent, 2, 2.75);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 18, 4, 0, 25, 40, 1.0, true);
    }
}

unsafe extern "C" fn sound_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_s"));
    }
}

unsafe extern "C" fn expression_attack11(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attackdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 11.0, 70, 85, 0, 50, 7.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 80, 60, 0, 45, 4.0, 0.0, 9.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("hip"), 12.0, 22, 82, 0, 31, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("kneel"), 12.0, 22, 82, 0, 31, 5.0, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 2, 0, Hash40::new("kneel"), 12.0, 22, 82, 0, 31, 5.5, 5.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, false, 0);
    }
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 13.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 4.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 14.0, 78, 75, 0, 50, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 4.5, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("haver"), 8.0, 78, 75, 0, 50, 4.5, 0.0, 12.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -10, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -10, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("ganon_sword_flare"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -8, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -8, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -6, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -6, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -4, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -4, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        macros::FOOT_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -2, 0, 3.0, 0, 0, 120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        else {
            macros::EFFECT(agent, Hash40::new("sys_damage_spark_s"), Hash40::new("top"), -2, 0, 3.0, 0, 0, -120, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("haver"), 0, 0, 0, -80, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("ganon_sword_flare"), false, false);
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ganon_appeal_l01"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_ganon_attack06"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_m"));
    }
}

unsafe extern "C" fn expression_attackhi3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_TOP);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackl"), 0);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

unsafe extern "C" fn game_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 65, 40, 10, 40, 3.0, 0.0, 3.0, 2.0, Some(0.0), Some(3.0), Some(14.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn effect_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 8.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 8, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

unsafe extern "C" fn expression_attacklw3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

unsafe extern "C" fn game_attacklw32(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 85, 60, 0, 75, 4.0, 0.0, 4.0, 3.0, Some(0.0), Some(4.0), Some(15.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attacklw32(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 9.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.8, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 9, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_attacklw32(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_ganon_rnd_attack"));
        macros::PLAY_SE(agent, Hash40::new("se_ganon_swing_l"));
    }
}

unsafe extern "C" fn expression_attacklw32(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attack11", game_attack11, Priority::Low);
    agent.acmd("effect_attack11", effect_attack11, Priority::Low);
    agent.acmd("sound_attack11", sound_attack11, Priority::Low);
    agent.acmd("expression_attack11", expression_attack11, Priority::Low);

    agent.acmd("game_attackdash", game_attackdash, Priority::Low);

    agent.acmd("game_attacks3", game_attacks3, Priority::Low);

    agent.acmd("game_attackhi3", game_attackhi3, Priority::Low);
    agent.acmd("effect_attackhi3", effect_attackhi3, Priority::Low);
    agent.acmd("sound_attackhi3", sound_attackhi3, Priority::Low);
    agent.acmd("expression_attackhi3", expression_attackhi3, Priority::Low);

    agent.acmd("game_attacklw3", game_attacklw3, Priority::Low);
    agent.acmd("effect_attacklw3", effect_attacklw3, Priority::Low);
    agent.acmd("sound_attacklw3", sound_attacklw3, Priority::Low);
    agent.acmd("expression_attacklw3", expression_attacklw3, Priority::Low);

    agent.acmd("game_attacklw32", game_attacklw32, Priority::Low);
    agent.acmd("effect_attacklw32", effect_attacklw32, Priority::Low);
    agent.acmd("sound_attacklw32", sound_attacklw32, Priority::Low);
    agent.acmd("expression_attacklw32", expression_attacklw32, Priority::Low);
}