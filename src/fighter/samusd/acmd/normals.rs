use crate::imports::acmd_imports::*;

#[acmd_script( agent = "samusd", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn samusd_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.0, 0.0, 10.0, 6.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.3, 0.0, 10.0, 8.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 10, 0, 25, 2.5, 0.0, 10.0, 12.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 4.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 4.0, false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "samusd", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn samusd_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.2);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 0, 75, 0, 50, 6.5, 0.0, 8.0, 14.0, None, None, None, 0.66, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, -7.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attack12", category = ACMD_EFFECT, low_priority )]
unsafe fn samusd_attack12_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        let color = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) as i32;
        macros::EFFECT_FOLLOW_arg11(fighter, Hash40::new("samusd_gbeam_flash_01"), Hash40::new("armr"), 7, 0, 0, 0, 0, 0, 1.3, true, color);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11.5, 3, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_flash_01"), false, true);
    }
}

#[acmd_script( agent = "samusd", script = "expression_attack12", category = ACMD_EXPRESSION, low_priority )]
unsafe fn samusd_attack12_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_hide_gun") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, Hash40::new_raw(0x03dbd0dc6a), false, 0.0);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GUN, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
    }
}

#[acmd_script( agent = "samusd", script = "sound_attack12", category = ACMD_SOUND, low_priority )]
unsafe fn samusd_attack12_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_s01"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_smash_s02"));
    }
}

#[acmd_script( agent = "samusd", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn samusd_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 367, 0, 50, 38, 6.0, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-2.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 72, 40, 0, 120, 6.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-2.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attackdash", category = ACMD_EFFECT, low_priority )]
unsafe fn samusd_attackdash_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_dash_attack"), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        macros::BURN_COLOR_NORMAL(fighter);
    }
}

#[acmd_script( agent = "samusd", script = "expression_attackdash", category = ACMD_EXPRESSION, low_priority )]
unsafe fn samusd_attackdash_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 2, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

#[acmd_script( agent = "samusd", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn samusd_attacks3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.0, 67, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 67, 100, 0, 30, 3.0, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 10.0, 67, 100, 0, 30, 3.0, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "samusd", script = "game_attacks3hi", category = ACMD_GAME, low_priority )]
unsafe fn samusd_attacks3hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 67, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 67, 100, 0, 30, 3.0, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 11.0, 67, 100, 0, 30, 3.0, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "samusd", script = "game_attacks3lw", category = ACMD_GAME, low_priority )]
unsafe fn samusd_attacks3lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 9.0, 67, 80, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.0, 67, 100, 0, 30, 3.0, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 11.0, 67, 100, 0, 30, 3.0, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "samusd", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn samusd_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 78, 110, 0, 50, 4.0, 0.0, 13.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 78, 110, 0, 50, 5.0, 0.0, 21.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 78, 110, 0, 50, 4.0, 0.0, 23.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "samusd", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn samusd_attackhi3_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 2.0, true);
        macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_cshot_bullet_sub_a"), Hash40::new("handl"), 7, 0, 0, 0, 0, 0, 0.6, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_b"), Hash40::new("top"), 3, 16, -1, 1.7, -0, 135, 1.3, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_cshot_bullet_sub_a"), false, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        macros::BURN_COLOR_NORMAL(fighter);
    }
}

#[acmd_script( agent = "samusd", script = "sound_attackhi3", category = ACMD_SOUND, low_priority )]
unsafe fn samusd_attackhi3_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_attackhard_h01"));
    }
}

#[acmd_script( agent = "samusd", script = "expression_attackhi3", category = ACMD_EXPRESSION, low_priority )]
unsafe fn samusd_attackhi3_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "samusd", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn samusd_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.0, 40, 50, 0, 80, 3.8, 0.0, 0.0, 0.0, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 40, 50, 0, 80, 7.2, 0.0, 1.6, 14.4, None, None, None, 0.65, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        samusd_attack11,
        samusd_attack12, samusd_attack12_eff, samusd_attack12_exp, samusd_attack12_snd,
        samusd_attackdash, samusd_attackdash_eff, samusd_attackdash_exp,
        samusd_attacks3, samusd_attacks3hi, samusd_attacks3lw,
        samusd_attackhi3, samusd_attackhi3_eff, samusd_attackhi3_snd, samusd_attackhi3_exp,
        samusd_attacklw3
    );
}