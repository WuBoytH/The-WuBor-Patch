use crate::imports::acmd_imports::*;

#[acmd_script( agent = "pikmin", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn pikmin_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 25, 0, 30, 2.4, 0.0, 5.0, 5.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 180, 25, 0, 30, 2.8, 0.0, 5.0, 8.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 361, 25, 0, 30, 2.8, 0.0, 5.0, 8.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 180, 15, 0, 25, 3.0, 0.0, 5.0, 10.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 361, 15, 0, 25, 3.0, 0.0, 5.0, 10.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    macros::FT_MOTION_RATE(fighter, 4.0 / 9.0);
    frame(fighter.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn pikmin_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 1.5, 365, 25, 50, 0, 4.0, 0.0, 7.0, 3.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 2.0);
    }
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 4.0, 75, 50, 0, 90, 4.5, 0.0, 7.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin", script = "expression_attackdash", category = ACMD_EXPRESSION, low_priority )]
unsafe fn pikmin_attackdash_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_TOP, 3, true);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohitm"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 4);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn pikmin_attacks3(_fighter: &mut L2CAgentBase) {
    // Blank
}

#[acmd_script( agent = "pikmin", script = "sound_attacks3", category = ACMD_SOUND, low_priority )]
unsafe fn pikmin_attacks3_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s01"));
    }
}

#[acmd_script( agent = "pikmin", script = "effect_attacks3loop", category = ACMD_EFFECT, low_priority )]
unsafe fn pikmin_attacks3loop_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pikmin_punch_spin"), Hash40::new("top"), -3, 6, -2, -192, 4, 68, 0.85, true);
    }
}

#[acmd_script( agent = "pikmin", script = "sound_attacks3loop", category = ACMD_SOUND, low_priority )]
unsafe fn pikmin_attacks3loop_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s01"));
    }
}

#[acmd_script( agent = "pikmin", script = "expression_attacks3loop", category = ACMD_EXPRESSION, low_priority )]
unsafe fn pikmin_attacks3loop_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_R);
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            7,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

#[acmd_script( agent = "pikmin", script = "game_attacks3end", category = ACMD_GAME, low_priority )]
unsafe fn pikmin_attacks3end(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        let loops_real = VarModule::get_int(fighter.battle_object, pikmin::instance::int::ATTACK_S3_LOOP_COUNT);
        let loops = loops_real.clamp(0, 6);
        let damage_add = 1.5 * loops as f32;
        let kbg_add = 5 * loops;
        let shield_damage_add = 2 * loops;
        let shieldstun_add = 0.1 * loops as f32;
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 + damage_add, 361, 87 + kbg_add, 0, 35, 4.5, 0.0, 5.0, 13.5, Some(0.0), Some(5.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2 + shield_damage_add, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0 + damage_add, 361, 87 + kbg_add, 0, 35, 2.8, 0.0, 5.0, 11.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -2 + shield_damage_add, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.8 + shieldstun_add);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.8 + shieldstun_add);
        VarModule::set_int(fighter.battle_object, pikmin::instance::int::ATTACK_S3_LOOP_COUNT, 0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin", script = "effect_attacks3end", category = ACMD_EFFECT, low_priority )]
unsafe fn pikmin_attacks3end_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -2, 5.5, -5, 0, 0, 0, 1.1, false, 0.5);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 5.5, -5, 0, 0, 0, 0.65, false, 0.6);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 5.5, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 360, true);
    }
}

#[acmd_script( agent = "pikmin", script = "sound_attacks3end", category = ACMD_SOUND, low_priority )]
unsafe fn pikmin_attacks3end_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_s02"));
    }
}

#[acmd_script( agent = "pikmin", script = "expression_attacks3end", category = ACMD_EXPRESSION, low_priority )]
unsafe fn pikmin_attacks3end_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_R, 5);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 5);
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_nohits"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn pikmin_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, pikmin::status::flag::ATTACK_HI3_DRIFT);
    }
    frame(fighter.lua_state_agent, 6.0);
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 0.6, 120, 100, 50, 0, 3.5, 3.0, 4.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 0.6, 120, 100, 50, 0, 3.5, -3.0, 4.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 0, Hash40::new("head"), 0.6, 92, 100, 20, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 2.0, 90, 120, 0, 50, 7.0, 0.0, 11.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 2.0, 90, 120, 0, 50, 6.0, 0.0, 17.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn pikmin_attackhi3_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pikmin_attack_spin"), Hash40::new("trans"), 0, 11, 0.5, 0, 0, 0, 0.55, true);
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("pikmin_attack_spin"), Hash40::new("trans"), 0, 6, 0.5, 0, 0, 0, 0.7, true, 0.7);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("pikmin_attack_spin"), Hash40::new("trans"), 0, 4, 0.5, 0, 0, 0, 0.85, true, 0.5);
    }
}

#[acmd_script( agent = "pikmin", script = "sound_attackhi3", category = ACMD_SOUND, low_priority )]
unsafe fn pikmin_attackhi3_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h01"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h02"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pikmin_attackhard_h03"));
    }
}

#[acmd_script( agent = "pikmin", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn pikmin_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 6.0, 65, 115, 0, 30, 4.8, 4.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 6.0, 65, 115, 0, 30, 4.0, 2.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(fighter, 13.0, -5.0);
}

pub fn install() {
    install_acmd_scripts!(
        pikmin_attack11,
        pikmin_attackdash, pikmin_attackdash_exp,
        pikmin_attacks3, pikmin_attacks3_snd,
        pikmin_attacks3loop_eff, pikmin_attacks3loop_snd, pikmin_attacks3loop_exp,
        pikmin_attacks3end, pikmin_attacks3end_eff, pikmin_attacks3end_snd, pikmin_attacks3end_exp,
        pikmin_attackhi3, pikmin_attackhi3_eff, pikmin_attackhi3_snd,
        pikmin_attacklw3
    );
}