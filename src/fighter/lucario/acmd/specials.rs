use crate::imports::acmd_imports::*;
use super::super::helper::*;

// SPECIAL N

unsafe extern "C" fn lucario_specialnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.0, 2.2);
    }
}

unsafe extern "C" fn lucario_specialnshoot(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_N_ENABLE_SUPERDASH);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 12.0, 6.0);
}

unsafe extern "C" fn lucario_specialnshoot_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_beamm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn lucario_specialairnshoot_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_beamm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

unsafe extern "C" fn lucario_specialnhold2_eff(agent: &mut L2CAgentBase) {
    FGCModule::ex_flash(agent);
    loop {
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_whirlwind_r"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false)
        }
        wait(agent.lua_state_agent, 8.0);
    }
}

unsafe extern "C" fn lucario_specialnhold2_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_waza_ex"));
        macros::PLAY_SE(agent, Hash40::new("vc_lucario_005"));
    }
}

unsafe extern "C" fn lucario_specialnshoot2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_AURABALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_N_SPIRIT_BOMB_ENABLE_FALL);
    }
}

unsafe extern "C" fn lucario_specialnshoot2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false)
    }
}

unsafe extern "C" fn lucario_specialnshoot2_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucario_attack07"));
    }
}

unsafe extern "C" fn lucario_specialnshoot2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_beaml"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

// SPECIAL S

unsafe extern "C" fn lucario_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 2.0, 5.0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.6);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_S_CHECK_ENHANCE);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 5.0, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 7.0, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 9.0, Some(0.0), Some(6.0), Some(4.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 11.5, Some(0.0), Some(6.0), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 12.5, Some(0.0), Some(6.0), Some(6.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

unsafe extern "C" fn lucario_specials_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("havel"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("haver"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn lucario_specials_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s03"));
    }
}

unsafe extern "C" fn lucario_specials_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn lucario_specials2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 2.0, 5.0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 5.0, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 7.0, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 9.0, Some(0.0), Some(6.0), Some(4.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 11.5, Some(0.0), Some(6.0), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 12.5, Some(0.0), Some(6.0), Some(6.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn lucario_specials2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("havel"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("haver"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4.5, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 8.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.0, -3, 180, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}

unsafe extern "C" fn lucario_specials2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s03"));
    }
}

unsafe extern "C" fn lucario_specials2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn lucario_specialairs(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 9.0, 5.0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.6);
    frame(agent.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_S_CHECK_ENHANCE);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 5.0, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 7.0, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 9.0, Some(0.0), Some(6.0), Some(4.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 11.5, Some(0.0), Some(6.0), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 12.5, Some(0.0), Some(6.0), Some(6.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
}

unsafe extern "C" fn lucario_specialairs_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("havel"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("haver"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
}

unsafe extern "C" fn lucario_specialairs_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s03"));
    }
}

unsafe extern "C" fn lucario_specialairs_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn lucario_specialairs2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 9.0, 5.0);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 5.0, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 7.0, Some(0.0), Some(6.0), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 9.0, Some(0.0), Some(6.0), Some(4.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 11.5, Some(0.0), Some(6.0), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.0, 12.5, Some(0.0), Some(6.0), Some(6.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn lucario_specialairs2_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("havel"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_aura"), Hash40::new("haver"), 0, 0, 0.5, 0, 0, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 8.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.0, -3, 180, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        }
        wait(agent.lua_state_agent, 2.0);
    }
}

unsafe extern "C" fn lucario_specialairs2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s03"));
    }
}

unsafe extern "C" fn lucario_specialairs2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn lucario_specialsthrow(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 20, 50, 40, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 27.0, -5.0);
}

unsafe extern "C" fn lucario_specialsthrow_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_near"), Hash40::new("top"), 0, 6.25, 12, 0, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn lucario_specialsthrow_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_005"));
    }
    let scale = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    if scale <= WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_s"));
        }
    }
    else if scale <= WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
        }
    }
}

unsafe extern "C" fn lucario_specialsthrow_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_grapple"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_impact"), 0);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn lucario_specialsthrow2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 361, 50, 40, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        lucario_drain_aura(agent, false);
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 50.0);
    macros::FT_MOTION_RATE(agent, 0.25);
    frame(agent.lua_state_agent, 67.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCARIO_GENERATE_ARTICLE_QIGONG, false, -1);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    macros::FT_MOTION_RATE(agent, 2.25);
}

unsafe extern "C" fn lucario_specialsthrow2_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_hakkei_near"), Hash40::new("top"), 0, 6.25, 12, 0, 0, 0, 0.5, true);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 35.0);
    FGCModule::ex_flash(agent);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("lucario_hakkei_start"), Hash40::new("haver"), -0.9, 0, 0, 0, 0, 0, 1.5, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.25);
    }
    frame(agent.lua_state_agent, 67.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn lucario_specialsthrow2_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_005"));
    }
    let scale = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    if scale <= WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_s"));
        }
    }
    else if scale <= WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
        }
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_waza_ex"));
    }
    frame(agent.lua_state_agent, 55.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucario_004"));
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_m"));
    }
}

unsafe extern "C" fn lucario_specialsthrow2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_grapple"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_impact"), 0);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_impact"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 84.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn lucario_specialairsthrow(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 15, 50, 40, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn lucario_specialairsthrow_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn lucario_specialairsthrow_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_005"));
    }
    let scale = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    if scale <= WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_s"));
        }
    }
    else if scale <= WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
        }
    }
}

unsafe extern "C" fn lucario_specialairsthrow_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_grapple"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_impact"), 0);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn lucario_specialairsthrow2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        lucario_drain_aura(agent, false);
        macros::WHOLE_HIT(agent, *HIT_STATUS_INVINCIBLE);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 80, 50, 40, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 6.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_POWER_PUNCH_STATUS_WORK_ID_FLAG_REQUEST_THROW);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_S_ENABLE_GRAVITY);
    }
}

unsafe extern "C" fn lucario_specialairsthrow2_eff(agent: &mut L2CAgentBase) {
    FGCModule::ex_flash(agent);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_l"), Hash40::new("handl"), 0.5, 0, 0, 0, 0, 180, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_bflash_r"), Hash40::new("handr"), 0.5, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn lucario_specialairsthrow2_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_waza_ex"));
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_005"));
    }
    let scale = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    if scale <= WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_s"));
        }
    }
    else if scale <= WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER) {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_m"));
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s01"));
            macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
        }
    }
}

unsafe extern "C" fn lucario_specialairsthrow2_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_grapple"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 23.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_impact"), 0);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

// SPECIAL HI

unsafe extern "C" fn lucario_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }
}

unsafe extern "C" fn lucario_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.75);
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_GRAVITY_ONOFF);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_RUSH_DIR);
    }
}

unsafe extern "C" fn lucario_specialhimove(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        if VarModule::get_int(agent.module_accessor, lucario::status::int::AURA_ENHANCED_BY) > 0 {
            macros::ATTACK(agent, 0, 0, Hash40::new("rot"), 2.0, 38, 70, 0, 50, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_NONE);
        }
    }
}

unsafe extern "C" fn lucario_specialhimove_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_sinsoku_hadou2"), Hash40::new("havel"), 0, 0, 0, 0, -30, 180, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_sinsoku_hadou2"), Hash40::new("haver"), 0, 0, 0, 0, -30, 0, 1, true);
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    if VarModule::get_int(agent.module_accessor, lucario::status::int::AURA_ENHANCED_BY) > 0 {
        FGCModule::ex_flash(agent);
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("rot"), 0, 0, 0, 180, 0, 0, 1.0, true);
        }
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("lucario_shinsoku1"), Hash40::new("lucario_shinsoku2"), 11, Hash40::new("waist"), -2.0, 0.0, 3.0, Hash40::new("waist"), -2.0, 0.0, -3.0, true, Hash40::new("null"), Hash40::new("waist"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.5, -1.0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

unsafe extern "C" fn lucario_specialhimove_snd(agent: &mut L2CAgentBase) {
    if VarModule::get_int(agent.module_accessor, lucario::status::int::AURA_ENHANCED_BY) > 0 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_waza_ex"));
        }
    }
    let curr_aura = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER);
    let mid_aura = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_MIDDLE_AURAPOWER);
    let high_aura = WorkModule::get_float(agent.module_accessor, *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_SE_HIGH_AURAPOWER);
    if curr_aura < mid_aura {
        macros::PLAY_STATUS(agent, Hash40::new("se_lucario_special_h02"));
    }
    else if curr_aura < high_aura {
        macros::PLAY_STATUS(agent, Hash40::new("se_lucario_special_h03"));
        macros::PLAY_SE(agent, Hash40::new("se_lucario_smash_add03"));
    }
    else {
        macros::PLAY_STATUS(agent, Hash40::new("se_lucario_special_h04"));
        macros::PLAY_SE(agent, Hash40::new("se_lucario_smash_add02"));
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_REMAIN(agent, Hash40::new("vc_lucario_006"));
    }
}

unsafe extern "C" fn lucario_specialhiend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 12.0);
    if VarModule::is_flag(agent.module_accessor, lucario::status::flag::SPECIAL_HI_SUPER_DASH_CANCEL) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

unsafe extern "C" fn lucario_specialairhiend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_LUCARIO_MACH_STATUS_WORK_ID_FLAG_AIR_END_CONTROL_X);
    }
    frame(agent.lua_state_agent, 12.0);
    if VarModule::is_flag(agent.module_accessor, lucario::status::flag::SPECIAL_HI_SUPER_DASH_CANCEL) {
        CancelModule::enable_cancel(agent.module_accessor);
    }
}

// SPECIAL LW

unsafe extern "C" fn lucario_speciallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 83, 93, 0, 10, 3.0, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(-5.0), 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 83, 93, 0, 10, 4.0, 0.0, 5.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn lucario_speciallw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_kagebunshin"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

unsafe extern "C" fn lucario_speciallw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_l01"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucario_appeal01"));
    }
}

unsafe extern "C" fn lucario_specialairlw(agent: &mut L2CAgentBase) {
    if !VarModule::is_flag(agent.module_accessor, lucario::instance::flag::USED_AURA_CHARGE_AIR) {
        if macros::is_excute(agent) {
            KineticModule::add_speed(agent.module_accessor, &Vector3f{x: 0.0, y: 0.4, z: 0.0});
            VarModule::on_flag(agent.module_accessor, lucario::instance::flag::USED_AURA_CHARGE_AIR);
        }
    }
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, lucario::status::flag::SPECIAL_LW_ENABLE_CANCEL);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 0.5, 83, 93, 0, 10, 3.0, 0.0, 4.0, 5.0, Some(0.0), Some(4.0), Some(-5.0), 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 0.5, 83, 93, 0, 10, 4.0, 0.0, 5.0, 0.0, Some(0.0), Some(15.0), Some(0.0), 0.78, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 5, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn lucario_specialairlw_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_kagebunshin"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("lucario_aura"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 0.5);
    }
}

unsafe extern "C" fn lucario_specialairlw_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_l01"));
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucario_appeal01"));
    }
}

unsafe extern "C" fn lucario_speciallwend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn lucario_speciallwend_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucario_aura"), false, true);
    }
}

unsafe extern "C" fn lucario_speciallwcancel(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
}

unsafe extern "C" fn lucario_speciallwcancel_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucario_aura"), false, true);
    }
}

unsafe extern "C" fn lucario_speciallwcancel_snd(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("vc_lucario_appeal02"));
    }
}

unsafe extern "C" fn lucario_speciallwattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 279, 30, 0, 60, 3.0, 0.0, 3.0, 20.0, Some(0.0), Some(3.0), Some(4.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_AURA);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 81, 30, 0, 60, 6.0, 0.0, 6.0, 12.0, Some(0.0), Some(14.0), Some(12.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_AURA);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 12.0, 81, 30, 0, 60, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(20.0), Some(12.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_AURA);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 44.0, 5.0);
}

unsafe extern "C" fn lucario_speciallwattack_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucario_aura"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucario_kagebunshin"), false, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_c"), Hash40::new("top"), -1, 6, 6, 0, 0, -80, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.25);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_c"), false, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 12, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("lucario_hakkei_near"), Hash40::new("top"), 0.0, 0.0, 12.0, -90, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn lucario_speciallwattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucario_attack07"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_swing_l"));
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_special_s02_l"));
    }
}

unsafe extern "C" fn lucario_speciallwattack_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_impact"), 0);
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_stg_shockwave_l"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

unsafe extern "C" fn lucario_specialairlwattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        JostleModule::set_status(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 40.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 12.0, 279, 30, 0, 60, 6.0, 0.0, -4.0, 7.0, Some(0.0), Some(7.0), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_AURA);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 12.0, 279, 20, 0, 45, 6.0, 0.0, -4.0, 7.0, Some(0.0), Some(7.0), Some(7.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 8, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_LUCARIO, *ATTACK_REGION_AURA);
        AttackModule::set_add_reaction_frame_revised(agent.module_accessor, 1, 8.0, false);
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 44.0, 5.0);
}

unsafe extern "C" fn lucario_specialairlwattack_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucario_aura"), false, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("lucario_kagebunshin"), false, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("lucario_kagebunshin_flash"), Hash40::new("top"), 0, 15, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_arc_c"), Hash40::new("top"), -1, 6, 3, 0, 0, -80, 1.0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
}

unsafe extern "C" fn lucario_specialairlwattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucario_attack07"));
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_lucario_swing_l"));
    }
}

unsafe extern "C" fn lucario_specialairlwattack_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(
            agent.module_accessor,
            Hash40::new("rbkind_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_impact"), 0);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, true, 0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialnstart", lucario_specialnstart);

    agent.acmd("game_specialairnstart", lucario_specialnstart);

    agent.acmd("game_specialnshoot", lucario_specialnshoot);
    agent.acmd("expression_specialnshoot", lucario_specialnshoot_exp);

    agent.acmd("game_specialairnshoot", lucario_specialnshoot);
    agent.acmd("expression_specialairnshoot", lucario_specialairnshoot_exp);

    agent.acmd("effect_specialnhold2", lucario_specialnhold2_eff);
    agent.acmd("sound_specialnhold2", lucario_specialnhold2_snd);

    agent.acmd("effect_specialairnhold2", lucario_specialnhold2_eff);
    agent.acmd("sound_specialairnhold2", lucario_specialnhold2_snd);

    agent.acmd("game_specialnshoot2", lucario_specialnshoot2);
    agent.acmd("effect_specialnshoot2", lucario_specialnshoot2_eff);
    agent.acmd("sound_specialnshoot2", lucario_specialnshoot2_snd);
    agent.acmd("expression_specialnshoot2", lucario_specialnshoot2_exp);

    agent.acmd("game_specialairnshoot2", lucario_specialnshoot2);
    agent.acmd("sound_specialairnshoot2", lucario_specialnshoot2_snd);
    agent.acmd("expression_specialairnshoot2", lucario_specialnshoot2_exp);

    agent.acmd("game_specials", lucario_specials);
    agent.acmd("effect_specials", lucario_specials_eff);
    agent.acmd("sound_specials", lucario_specials_snd);
    agent.acmd("expression_specials", lucario_specials_exp);

    agent.acmd("game_specials2", lucario_specials2);
    agent.acmd("effect_specials2", lucario_specials2_eff);
    agent.acmd("sound_specials2", lucario_specials2_snd);
    agent.acmd("expression_specials2", lucario_specials2_exp);

    agent.acmd("game_specialairs", lucario_specialairs);
    agent.acmd("effect_specialairs", lucario_specialairs_eff);
    agent.acmd("sound_specialairs", lucario_specialairs_snd);
    agent.acmd("expression_specialairs", lucario_specialairs_exp);

    agent.acmd("game_specialairs2", lucario_specialairs2);
    agent.acmd("effect_specialairs2", lucario_specialairs2_eff);
    agent.acmd("sound_specialairs2", lucario_specialairs2_snd);
    agent.acmd("expression_specialairs2", lucario_specialairs2_exp);

    agent.acmd("game_specialsthrow", lucario_specialsthrow);
    agent.acmd("effect_specialsthrow", lucario_specialsthrow_eff);
    agent.acmd("sound_specialsthrow", lucario_specialsthrow_snd);
    agent.acmd("expression_specialsthrow", lucario_specialsthrow_exp);

    agent.acmd("game_specialsthrow2", lucario_specialsthrow2);
    agent.acmd("effect_specialsthrow2", lucario_specialsthrow2_eff);
    agent.acmd("sound_specialsthrow2", lucario_specialsthrow2_snd);
    agent.acmd("expression_specialsthrow2", lucario_specialsthrow2_exp);

    agent.acmd("game_specialairsthrow", lucario_specialairsthrow);
    agent.acmd("effect_specialairsthrow", lucario_specialairsthrow_eff);
    agent.acmd("sound_specialairsthrow", lucario_specialairsthrow_snd);
    agent.acmd("expression_specialairsthrow", lucario_specialairsthrow_exp);

    agent.acmd("game_specialairsthrow2", lucario_specialairsthrow2);
    agent.acmd("effect_specialairsthrow2", lucario_specialairsthrow2_eff);
    agent.acmd("sound_specialairsthrow2", lucario_specialairsthrow2_snd);
    agent.acmd("expression_specialairsthrow2", lucario_specialairsthrow2_exp);

    agent.acmd("game_specialhi", lucario_specialhi);

    agent.acmd("game_specialairhi", lucario_specialairhi);

    agent.acmd("game_specialhimove", lucario_specialhimove);
    agent.acmd("effect_specialhimove", lucario_specialhimove_eff);
    agent.acmd("sound_specialhimove", lucario_specialhimove_snd);

    agent.acmd("game_specialhiend", lucario_specialhiend);

    agent.acmd("game_specialairhiend", lucario_specialairhiend);

    agent.acmd("game_speciallw", lucario_speciallw);
    agent.acmd("effect_speciallw", lucario_speciallw_eff);
    agent.acmd("sound_speciallw", lucario_speciallw_snd);

    agent.acmd("game_specialairlw", lucario_specialairlw);
    agent.acmd("effect_specialairlw", lucario_specialairlw_eff);
    agent.acmd("sound_specialairlw", lucario_specialairlw_snd);

    agent.acmd("game_specialairlwend", lucario_speciallwend);
    agent.acmd("effect_specialairlwend", lucario_speciallwend_eff);

    agent.acmd("game_specialairlwend", lucario_speciallwend);
    agent.acmd("effect_specialairlwend", lucario_speciallwend_eff);

    agent.acmd("game_speciallwcancel", lucario_speciallwcancel);
    agent.acmd("effect_speciallwcancel", lucario_speciallwcancel_eff);
    agent.acmd("sound_speciallwcancel", lucario_speciallwcancel_snd);

    agent.acmd("game_specialairlwcancel", lucario_speciallwcancel);
    agent.acmd("effect_specialairlwcancel", lucario_speciallwcancel_eff);
    agent.acmd("sound_specialairlwcancel", lucario_speciallwcancel_snd);

    agent.acmd("game_speciallwattack", lucario_speciallwattack);
    agent.acmd("effect_speciallwattack", lucario_speciallwattack_eff);
    agent.acmd("sound_speciallwattack", lucario_speciallwattack_snd);
    agent.acmd("expression_speciallwattack", lucario_speciallwattack_exp);

    agent.acmd("game_specialairlwattack", lucario_specialairlwattack);
    agent.acmd("effect_specialairlwattack", lucario_specialairlwattack_eff);
    agent.acmd("sound_specialairlwattack", lucario_specialairlwattack_snd);
    agent.acmd("expression_specialairlwattack", lucario_specialairlwattack_exp);
}