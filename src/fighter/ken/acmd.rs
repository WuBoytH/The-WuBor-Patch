use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40, Vector2f},
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "ken", script = "game_run", category = ACMD_GAME, low_priority )]
unsafe fn ken_run(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if VarModule::get_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
            macros::FT_MOTION_RATE(fighter, 0.7);
        }
    }
    frame(fighter.lua_state_agent, 22.0);
    if VarModule::get_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.battle_object, ken::status::flag::SPECIAL_LW_STEP_KICK);
        }
    }
    frame(fighter.lua_state_agent, 31.0);
    if VarModule::get_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE) == ken::QUICK_STEP_STATE_RUN {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

// Make Quick Step (non-prox light f tilt) have step kick properties

#[acmd_script( agent = "ken", script = "game_attacks3w", category = ACMD_GAME, low_priority )]
unsafe fn ken_attacks3w(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        VarModule::set_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE, ken::QUICK_STEP_STATE_DISABLE);
    }
    frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    frame(fighter.lua_state_agent, 8.0);
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 72, 46, 33, 50, 3.8, 0.0, 11.0, 6.5, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 72, 46, 33, 50, 3.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 55, 46, 0, 66, 3.8, 0.0, 11.0, 6.5, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.8, 72, 46, 0, 66, 3.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        HitModule::set_status_all(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 26.0);
    if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
        VarModule::set_int(fighter.battle_object, ken::instance::int::QUICK_STEP_STATE, ken::QUICK_STEP_STATE_ENABLE);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

// Nerfed damage on Inazuma Kick, but increased combo potential

#[acmd_script( agent = "ken", script = "game_attackcommand3", category = ACMD_GAME, low_priority )]
unsafe fn ken_attackcommand3(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("kneer"), 10.0, 30, 50, 0, 35, 3.0, 6.3, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.25);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

// V Shift start-up.

// #[acmd_script( agent = "ken", script = "game_speciallwstepb", category = ACMD_GAME, low_priority )]
// unsafe fn ken_speciallwstepb(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
//         DamageModule::set_damage_lock(fighter.module_accessor, true);
//     }
//     macros::FT_MOTION_RATE(fighter, 1.6);
//     frame(fighter.lua_state_agent, 8.75);
//     if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT) {
//         if macros::is_excute(fighter) {
//             HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
//         }
//     }
//     frame(fighter.lua_state_agent, 20.0);
//     if macros::is_excute(fighter) {
//         damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
//         DamageModule::set_damage_lock(fighter.module_accessor, false);
//     }
// }

// V-Shift Break - Shadow-Thunder Kick

// #[acmd_script( agent = "ken", script = "game_speciallw", category = ACMD_GAME, low_priority )]
// unsafe fn ken_speciallw(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
//         if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT) {
//             macros::SLOW_OPPONENT(fighter, 100.0, 18.0);
//         }
//     }
//     macros::FT_MOTION_RATE(fighter, 1.0);
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 6.0, 30, 98, 100, 0, 3.2, -1.5, -1.0, -1.0, None, None, None, 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
//         macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 30, 98, 100, 0, 3.2, -6.2, -1.0, -1.0, None, None, None, 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
//         macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 6.0, 30, 98, 100, 0, 3.9, 4.3, -1.7, -1.0, None, None, None, 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
//         AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
//         AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
//         AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 5.0, false);
//     }
//     wait(fighter.lua_state_agent, 3.0);
//     if macros::is_excute(fighter) {
//         AttackModule::clear_all(fighter.module_accessor);
//         HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
//         if WorkModule::is_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT) {
//             macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
//             WorkModule::off_flag(fighter.module_accessor, FIGHTER_KEN_STATUS_GUARD_FLAG_V_SHIFT);
//         }
//     }
// }

// #[acmd_script( agent = "ken", script = "effect_speciallw", category = ACMD_EFFECT, low_priority )]
// unsafe fn ken_speciallw_eff(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
//         macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("footl"), 0.5, 0, 0, 0, 0, 0, 1.5, true);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
//         macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.5, 1);
//         macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ryu_attack_line"), Hash40::new("ryu_attack_line"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, true, *EF_FLIP_YZ);
//         macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 14, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
//         macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//     }
//     frame(fighter.lua_state_agent, 18.0);
//     if macros::is_excute(fighter) {
//         macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder"), false, true);
//     }
// }

// #[acmd_script( agent = "ken", script = "sound_speciallw", category = ACMD_SOUND, low_priority )]
// unsafe fn ken_speciallw_snd(fighter: &mut L2CAgentBase) {
//     frame(fighter.lua_state_agent, 9.0);
//     if macros::is_excute(fighter) {
//         macros::PLAY_SE(fighter, Hash40::new("se_ken_smash_s01"));
//         macros::PLAY_SE(fighter, Hash40::new("vc_ken_attack09"));
//     }
//     frame(fighter.lua_state_agent, 38.0);
//     if macros::is_excute(fighter) {
//         macros::PLAY_SE(fighter, Hash40::new("se_ken_step_left_m"));
//     }
// }

// #[acmd_script( agent = "ken", script = "expression_speciallw", category = ACMD_EXPRESSION, low_priority )]
// unsafe fn ken_speciallw_exp(fighter: &mut L2CAgentBase) {
//     if macros::is_excute(fighter) {
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//     }
//     frame(fighter.lua_state_agent, 7.0);
//     if macros::is_excute(fighter) {
//         slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
//         ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
//     }
//     frame(fighter.lua_state_agent, 13.0);
//     if macros::is_excute(fighter) {
//         ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit1"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
//         macros::AREA_WIND_2ND_arg10(fighter, 0, 0.8, 180, 8, 0.8, -10, 7, 20, 14, 80);
//     }
//     frame(fighter.lua_state_agent, 15.0);
//     if macros::is_excute(fighter) {
//         macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attack1"), 0);
//     }
//     frame(fighter.lua_state_agent, 28.0);
//     if macros::is_excute(fighter) {
//         AreaModule::erase_wind(fighter.module_accessor, 0);
//     }
// }

#[acmd_script( agent = "ken", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn ken_specialsstart(fighter: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let ratio;
    if VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
        ratio = 1.2;
    }
    else {
        ratio = 0.9;
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * ratio, 0, 50, 100, 0, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 3.0, 3.5, 8.5, 4.5);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
    }
}

#[acmd_script( agent = "ken", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn ken_specials(fighter: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let ratio;
    if VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
        ratio = 1.2;
    }
    else {
        ratio = 0.9;
    }
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 5.5, 3.0, 9.0, 3.0);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * ratio, 30, 20, 0, 60, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT) == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0 * ratio, 30, 30, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0 * ratio, 30, 30, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * ratio, 55, 20, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0 * ratio, 31, 25, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT) == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * ratio, 40, 20, 0, 120, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0 * ratio, 40, 20, 0, 120, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * ratio, 55, 20, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0 * ratio, 31, 25, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(fighter.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
        AttackModule::set_size(fighter.module_accessor, 1, 0.1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * ratio, 45, 120, 0, 30, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * ratio, 55, 20, 0, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * ratio, 55, 20, 0, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(fighter.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
        AttackModule::set_size(fighter.module_accessor, 1, 0.1);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ken", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn ken_specialairsstart(fighter: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let ratio;
    if VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
        ratio = 1.2;
    }
    else {
        ratio = 0.9;
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            let speedx = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, speedx, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * ratio, 80, 50, 0, 60, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * ratio, 0, 50, 100, 0, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 3.0, 3.5, 8.5, 4.5);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
    }
}

#[acmd_script( agent = "ken", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn ken_specialairs(fighter: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let ratio;
    if VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
        ratio = 1.2;
    }
    else {
        ratio = 0.9;
    }
    wait(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * ratio, 80, 50, 0, 60, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0 * ratio, 30, 30, 0, 60, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT) == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * ratio, 30, 20, 0, 70, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * ratio, 55, 20, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_S_INT_LOOP_COUNT) == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0 * ratio, 30, 20, 0, 120, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * ratio, 55, 20, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_size(fighter.module_accessor, 0, 0.1);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * ratio, 80, 50, 0, 60, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * ratio, 45, 120, 0, 30, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * ratio, 55, 20, 0, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * ratio, 55, 20, 0, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_size(fighter.module_accessor, 0, 0.1);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ken", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn ken_specialhi(fighter: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let ratio;
    if VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
        ratio = 1.0;
    }
    else {
        ratio = 0.8;
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_W
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2 * ratio, 100, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0 * ratio, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * ratio, 80, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * ratio, 95, 10, 0, 95, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0 * ratio, 70, 105, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0 * ratio, 70, 121, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5 * ratio, 70, 116, 0, 80, 5.5, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "ken", script = "game_specialhicommand", category = ACMD_GAME, low_priority )]
unsafe fn ken_specialhicommand(fighter: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let ratio;
    if VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
        ratio = 1.0;
    }
    else {
        ratio = 0.8;
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if VarModule::get_int(fighter.battle_object, ken::instance::int::SHORYUREPPA) == 1 {
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_RYU_STRENGTH_S, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_W
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_M {
        if VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER) {
            VarModule::inc_int(fighter.battle_object, ken::instance::int::SHORYUREPPA);
        }
        if VarModule::get_int(fighter.battle_object, ken::instance::int::SHORYUREPPA) == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 270, 10, 0, 25, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 270, 10, 0, 25, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 5.0, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2 * ratio, 100, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0 * ratio, 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * ratio, 80, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        if VarModule::get_int(fighter.battle_object, ken::instance::int::SHORYUREPPA) == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 270, 10, 0, 25, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 270, 10, 0, 25, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 28.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 28.0, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * ratio, 95, 10, 0, 95, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0 * ratio, 70, 105, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0 * ratio, 70, 121, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        if VarModule::get_int(fighter.battle_object, ken::instance::int::SHORYUREPPA) == 1 {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 270, 10, 30, 30, 5.5, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 270, 10, 30, 30, 5.5, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 28.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 28.0, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5 * ratio, 70, 116, 0, 80, 5.5, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "ken", script = "sound_specialhicommand", category = ACMD_SOUND, low_priority )]
unsafe fn ken_specialhicommand_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ken_command_success"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        if VarModule::get_int(fighter.battle_object, ken::instance::int::SHORYUREPPA) == 0 {
            macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_h01_command"));
        }
        else {
            macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_l02"));
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h03"));
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h01"));
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h02"));
        }
    }
}

#[acmd_script( agent = "ken", scripts = ["game_specialairhi", "game_specialairhicommand"], category = ACMD_GAME, low_priority )]
unsafe fn ken_specialhiair(fighter: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    if VarModule::is_flag(fighter.battle_object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
    }
    else {
        AttackModule::set_power_mul_status(fighter.module_accessor, 0.8);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_W
    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 90, 100, 90, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 80, 55, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 80, 54, 0, 70, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 80, 100, 100, 0, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 112, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0, 70, 107, 0, 76, 6.0, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "ken", script = "game_speciallwstepf", category = ACMD_GAME, low_priority )]
unsafe fn ken_speciallwstepf(fighter: &mut L2CAgentBase) {
    let special_lw_type = VarModule::get_int(fighter.battle_object, ken::instance::int::SPECIAL_LW_TYPE);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            SlowModule::clear_whole(fighter.module_accessor);
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
        }
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
            MotionModule::set_frame(fighter.module_accessor, 19.0, true);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "ken", script = "game_specialairlwstepf", category = ACMD_GAME, low_priority )]
unsafe fn ken_specialairlwstepf(fighter: &mut L2CAgentBase) {
    let special_lw_type = VarModule::get_int(fighter.battle_object, ken::instance::int::SPECIAL_LW_TYPE);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            SlowModule::clear_whole(fighter.module_accessor);
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_QUICK_STEP {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_UNIQ);
        }
        else if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
        }
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            let diff_x = VarModule::get_float(fighter.battle_object, ken::instance::float::DIFF_X);
            if diff_x != 0.0 {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: (diff_x / 5.0) * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            else {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: 10.0 * PostureModule::lr(fighter.module_accessor),
                    y: 0.0
                });
            }
            HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
            MotionModule::set_frame(fighter.module_accessor, 19.0, true);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        if special_lw_type == ken::SPECIAL_LW_TYPE_HEAT_RUSH {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "ken_hadoken", script = "game_movew", category = ACMD_GAME, low_priority )]
unsafe fn ken_hadoken_movew(weapon: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let object = sv_system::battle_object(otarget_id as u64);
    if VarModule::is_flag(object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
    }
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.5, 0, 10, 0, 45, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 4.5, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(weapon.module_accessor, 0, 5.0, false);
    }
    wait(weapon.lua_state_agent, 7.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 4.5, 0, 10, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 4.5, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script( agent = "ken_hadoken", script = "game_movem", category = ACMD_GAME, low_priority )]
unsafe fn ken_hadoken_movem(weapon: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let object = sv_system::battle_object(otarget_id as u64);
    if VarModule::is_flag(object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
    }
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 50, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(weapon.module_accessor, 0, 5.0, false);
    }
    wait(weapon.lua_state_agent, 6.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 50, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script( agent = "ken_hadoken", script = "game_moves", category = ACMD_GAME, low_priority )]
unsafe fn ken_hadoken_moves(weapon: &mut L2CAgentBase) {
    let mut property = "collision_attr_normal";
    let otarget_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let object = sv_system::battle_object(otarget_id as u64);
    if VarModule::is_flag(object, ken::instance::flag::V_TRIGGER) {
        property = "collision_attr_fire";
    }
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.5, 0, 10, 0, 55, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 5.5, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(weapon.module_accessor, 0, 5.0, false);
    }
    wait(weapon.lua_state_agent, 5.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 5.5, 0, 10, 0, 55, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 5.5, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ken_run,
        ken_attacks3w,
        ken_attackcommand3,
        // ken_speciallwstepb,
        // ken_speciallw, ken_speciallw_eff, ken_speciallw_snd, ken_speciallw_exp,
        ken_specialsstart,
        ken_specials,
        ken_specialairsstart,
        ken_specialairs,
        ken_specialhi,
        ken_specialhicommand, ken_specialhicommand_snd,
        ken_specialhiair,
        ken_speciallwstepf,
        ken_specialairlwstepf,
        ken_hadoken_movew,
        ken_hadoken_movem,
        ken_hadoken_moves
    );
}