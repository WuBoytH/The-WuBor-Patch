use crate::imports::acmd_imports::*;

// Spinning Demon Finisher

#[acmd_script( agent = "demon", script = "game_attackstand24", category = ACMD_GAME, low_priority )]
unsafe fn demon_attackstand24(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.5, 53, 50, 0, 83, 2.0, 0.0, 13.0, 12.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.5, 53, 50, 0, 83, 4.0, 0.0, 11.0, 4.0, Some(0.0), Some(12.0), Some(12.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.5, 53, 50, 0, 83, 4.0, 0.0, 12.5, 4.0, Some(0.0), Some(13.5), Some(12.0), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.5, 53, 50, 0, 83, 3.0, 0.0, 7.0, 4.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Spinning Demon 1 Finisher

#[acmd_script( agent = "demon", script = "game_attackstand2f", category = ACMD_GAME, low_priority )]
unsafe fn demon_attackstand2f(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 4.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.5, 44, 65, 0, 75, 3.0, 0.0, 15.0, 10.5, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.5, 44, 65, 0, 75, 5.2, 0.0, 10.5, 5.5, Some(0.0), Some(13.72), Some(10.5), 0.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "demon", script = "effect_attackstand2f", category = ACMD_EFFECT, low_priority )]
unsafe fn demon_attackstand2f_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("demon_devil_sign_flash"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false)
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("demon_attack_arc_d"), Hash40::new("top"), 0, 12.2, 4, 17, 10, 177, 0.85, 0, 0, 1, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "demon", script = "sound_attackstand2f", category = ACMD_SOUND, low_priority )]
unsafe fn demon_attackstand2f_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_demon_swing_long02"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_demon_rnd_attack_s_02"));
    }
}

#[acmd_script( agent = "demon", script = "expression_attackstand2f", category = ACMD_EXPRESSION, low_priority )]
unsafe fn demon_attackstand2f_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_80_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_L, 3);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_80_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_80_nohitl"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 4);
    }
}

pub fn install() {
    install_acmd_scripts!(
        demon_attackstand24,
        demon_attackstand2f, demon_attackstand2f_eff, demon_attackstand2f_snd, demon_attackstand2f_exp
    );
}