use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*,
    super::super::vl
};

#[acmd_script( agent = "lucario", scripts = ["game_specialnshoot", "game_specialairnshoot"], category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialnshoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

#[acmd_script( agent = "lucario_auraball", scripts = [ "game_charge", "game_chargemax" ], category = ACMD_GAME, low_priority )]
unsafe fn lucario_auraball_charge(_weapon: &mut L2CAgentBase) {
}

#[acmd_script( agent = "lucario_auraball", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn lucario_auraball_shoot(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 12.0, 361, 42, 0, 14, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 12.0, 361, 49, 0, 35, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2.3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        attack!(weapon, MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        AttackModule::enable_safe_pos(weapon.module_accessor);
    }
}

#[acmd_script( agent = "lucario", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specials(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 2.0, 5.0);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 6.3, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 6.0, 8.4, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairs(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 9.0, 5.0);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 6.3, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.0, 0.0, 6.0, 8.4, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialsthrow", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialsthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 361, 50, 40, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialsthrow2", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialsthrow2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 0, 50, 40, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, -1);
    }
}

#[acmd_script( agent = "lucario", script = "effect_specialsthrow2", category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_specialsthrow2_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_hakkei_near"), Hash40::new("top"), -2, 6, 10, 0, 0, 0, 1, true);
        let scale = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLOAT_HAKKEI_NEAR_SCALE);
        macros::LAST_EFFECT_SET_SCALE_W(fighter, scale, scale, scale);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucario", script = "sound_specialsthrow2", category = ACMD_SOUND, low_priority )]
unsafe fn lucario_specialsthrow2_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucario_005"));
    }
    let scale = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    if scale <= WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER) {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_s"));
        }
    }
    else if scale <= WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER) {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_m"));
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_l"));
        }
    }
}

#[acmd_script( agent = "lucario", script = "expression_specialsthrow2", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucario_specialsthrow2_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_grapple"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_impact"), 0);
    }
    frame(fighter.lua_state_agent, 54.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairsthrow", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairsthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 330, 50, 40, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

#[acmd_script( agent = "lucario", script = "effect_specialairsthrow", category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_specialairsthrow_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_hakkei_near"), Hash40::new("top"), -2, 6, 10, 0, 0, 0, 1, true);
        let scale = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLOAT_HAKKEI_NEAR_SCALE);
        macros::LAST_EFFECT_SET_SCALE_W(fighter, scale, scale, scale);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucario", script = "sound_specialairsthrow", category = ACMD_SOUND, low_priority )]
unsafe fn lucario_specialairsthrow_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucario_005"));
    }
    let scale = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    if scale <= WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER) {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_s"));
        }
    }
    else if scale <= WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER) {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_m"));
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_l"));
        }
    }
}

#[acmd_script( agent = "lucario", script = "expression_specialairsthrow", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucario_specialairsthrow_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_grapple"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_impact"), 0);
    }
    frame(fighter.lua_state_agent, 54.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairsthrow2", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairsthrow2(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 80, 50, 40, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        let target = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

#[acmd_script( agent = "lucario", script = "effect_specialairsthrow2", category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_specialairsthrow2_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_hakkei_near"), Hash40::new("top"), -2, 6, 10, 0, 0, 0, 1, true);
        let scale = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLOAT_HAKKEI_NEAR_SCALE);
        macros::LAST_EFFECT_SET_SCALE_W(fighter, scale, scale, scale);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucario", script = "sound_specialairsthrow2", category = ACMD_SOUND, low_priority )]
unsafe fn lucario_specialairsthrow2_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucario_005"));
    }
    let scale = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    if scale <= WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER) {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_s"));
        }
    }
    else if scale <= WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER) {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_m"));
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_s02_l"));
        }
    }
}

#[acmd_script( agent = "lucario", script = "expression_specialairsthrow2", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucario_specialairsthrow2_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_grapple"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_impact"), 0);
    }
    frame(fighter.lua_state_agent, 54.0);
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    }
}

#[acmd_script( agent = "lucario_qigong", script = "game_shoot", category = ACMD_GAME, low_priority )]
unsafe fn lucario_qigong_shoot(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 361, 58, 0, 48, 3.0, 0.0, 0.0, -4.5, Some(0.0), Some(0.0), Some(22.0), 0.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        macros::ATTACK(weapon, 1, 1, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.0, 0.0, 0.0, -4.5, Some(0.0), Some(0.0), Some(22.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_attack_level(weapon.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
    }
    wait(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
    }
    wait(weapon.lua_state_agent, 8.0);
    if macros::is_excute(weapon) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
}

#[acmd_script( agent = "lucario", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialhi(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::IS_SUPER_DASH_CANCEL);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairhi(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        VarModule::on_flag(fighter.battle_object, commons::instance::flag::DISABLE_SPECIAL_HI);
        VarModule::off_flag(fighter.battle_object, lucario::instance::flag::IS_SUPER_DASH_CANCEL);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialhimove", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialhimove(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 55, 100, 30, 60, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairhimove", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairhimove(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 5.0, 55, 100, 30, 60, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_specialhiend", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialhiend(fighter: &mut L2CAgentBase) {
    wait(fighter.lua_state_agent, 1.0);
    if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 38, 100, 0, 70, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    wait(fighter.lua_state_agent, 10.0);
    if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairhiend", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairhiend(fighter: &mut L2CAgentBase) {
    wait(fighter.lua_state_agent, 1.0);
    if !VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 6.0, 38, 100, 0, 70, 12.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if VarModule::is_flag(fighter.battle_object, commons::instance::flag::IS_FGC) {
            if VarModule::is_flag(fighter.battle_object, lucario::instance::flag::IS_SUPER_DASH_CANCEL) {
                macros::SET_SPEED_EX(fighter, 1.1, 1.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_AERIAL, true);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
    }
}

#[acmd_script( agent = "lucario", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn lucario_speciallw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 8.0);
    frame(fighter.lua_state_agent, 6.125);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 83, 93, 0, 10, 3.0, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(-5.0), 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 83, 93, 0, 10, 4.0, 0.0, 5.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
    }
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        if VarModule::get_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL) < vl::private::AURA_CHARGE_MAX {
            VarModule::inc_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL);
            FighterUtil::flash_eye_info(fighter.module_accessor);
            VarModule::off_flag(fighter.battle_object, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
        }
    }
}

#[acmd_script( agent = "lucario", script = "effect_speciallw", category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_speciallw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_kagebunshin"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(fighter, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.70);
    }
}

#[acmd_script( agent = "lucario", script = "sound_speciallw", category = ACMD_SOUND, low_priority )]
unsafe fn lucario_speciallw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.125);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_l01"));
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairlw(fighter: &mut L2CAgentBase) {
    if !VarModule::is_flag(fighter.battle_object, lucario::instance::flag::USED_AURA_CHARGE_AIR) {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: 0.4, z: 0.0});
            VarModule::on_flag(fighter.battle_object, lucario::instance::flag::USED_AURA_CHARGE_AIR);
        }
    }
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 8.0);
    frame(fighter.lua_state_agent, 6.125);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 83, 93, 0, 10, 3.0, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(-5.0), 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 83, 93, 0, 10, 4.0, 0.0, 5.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
    }
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        if VarModule::get_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL) < vl::private::AURA_CHARGE_MAX {
            VarModule::inc_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL);
            FighterUtil::flash_eye_info(fighter.module_accessor);
            VarModule::off_flag(fighter.battle_object, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
        }
    }
}

#[acmd_script( agent = "lucario", script = "effect_specialairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_specialairlw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_kagebunshin"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT(fighter, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("lucario_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.70);
    }
}

#[acmd_script( agent = "lucario", script = "sound_specialairlw", category = ACMD_SOUND, low_priority )]
unsafe fn lucario_specialairlw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.125);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucario_special_l01"));
    }
}

#[acmd_script( agent = "lucario", script = "game_speciallwcancel", category = ACMD_GAME, low_priority )]
unsafe fn lucario_speciallwcancel(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "lucario", script = "effect_speciallwcancel", category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_speciallwcancel_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("lucario_aura"), false, true);
    }
}

#[acmd_script( agent = "lucario", script = "game_specialairlwcancel", category = ACMD_GAME, low_priority )]
unsafe fn lucario_specialairlwcancel(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "lucario", script = "effect_specialairlwcancel", category = ACMD_EFFECT, low_priority )]
unsafe fn lucario_specialairlwcancel_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("lucario_aura"), false, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucario_specialnshoot,
        lucario_auraball_charge,
        lucario_auraball_shoot,
        lucario_specials,
        lucario_specialairs,
        lucario_specialsthrow,
        lucario_specialsthrow2, lucario_specialsthrow2_eff, lucario_specialsthrow2_snd, lucario_specialsthrow2_exp,
        lucario_specialairsthrow, lucario_specialairsthrow_eff, lucario_specialairsthrow_snd, lucario_specialairsthrow_exp,
        lucario_specialairsthrow2, lucario_specialairsthrow2_eff, lucario_specialairsthrow2_snd, lucario_specialairsthrow2_exp,
        lucario_qigong_shoot,
        lucario_specialhi,
        lucario_specialairhi,
        lucario_specialhimove,
        lucario_specialairhimove,
        lucario_specialhiend,
        lucario_specialairhiend,
        lucario_speciallw, lucario_speciallw_eff, lucario_speciallw_snd,
        lucario_specialairlw, lucario_specialairlw_eff, lucario_specialairlw_snd,
        lucario_speciallwcancel, lucario_speciallwcancel_eff,
        lucario_specialairlwcancel, lucario_specialairlwcancel_eff
    );
}