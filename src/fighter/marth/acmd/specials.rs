use crate::imports::acmd_imports::*;
use super::super::helper::*;

#[acmd_script( agent = "marth", script = "game_specialnend", category = ACMD_GAME, low_priority )]
unsafe fn marth_specialnend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 7.5, 17.0, Some(0.0), Some(7.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 7.5, 23.5, Some(0.0), Some(7.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_specialnend", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_specialnend_eff(fighter: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 7.5, 15, 0, 0, 0, 0.9, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

#[acmd_script( agent = "marth", script = "game_specialnendhi", category = ACMD_GAME, low_priority )]
unsafe fn marth_specialnendhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_specialnendhi", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_specialnendhi_eff(fighter: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 10.5, 15, -20, 0, 0, 0.9, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, -20, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

#[acmd_script( agent = "marth", script = "game_specialnendlw", category = ACMD_GAME, low_priority )]
unsafe fn marth_specialnendlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 2.0, 17.0, Some(0.0), Some(0.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 0.0, 23.5, Some(0.0), Some(-1.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_specialnendlw", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_specialnendlw_eff(fighter: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 4.5, 15, 12, 0, 0, 0.9, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 12, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

#[acmd_script( agent = "marth", script = "game_specialnendmax", category = ACMD_GAME, low_priority )]
unsafe fn marth_specialnendmax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_FIRE);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 7.5, 17.0, Some(0.0), Some(7.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 7.5, 23.5, Some(0.0), Some(7.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_specialnendmax", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_specialnendmax_eff(fighter: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 7.5, 15, 0, 0, 0, 0.9, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);

    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

#[acmd_script( agent = "marth", script = "game_specialnendmaxhi", category = ACMD_GAME, low_priority )]
unsafe fn marth_specialnendmaxhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_FIRE);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_specialnendmaxhi", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_specialnendmaxhi_eff(fighter: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 10.5, 15, -20, 0, 0, 0.9, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, -20, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);

    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

#[acmd_script( agent = "marth", script = "game_specialnendmaxlw", category = ACMD_GAME, low_priority )]
unsafe fn marth_specialnendmaxlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_FIRE);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 2.0, 17.0, Some(0.0), Some(0.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 0.0, 23.5, Some(0.0), Some(-1.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_specialnendmaxlw", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_specialnendmaxlw_eff(fighter: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 4.5, 15, 12, 0, 0, 0.9, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 12, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);

    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

#[acmd_script( agent = "marth", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn marth_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    let unstance = marth_unstance_game(fighter, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 361, 74, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 11.0, 74, 74, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 7.0, 361, 90, 0, 20, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 7.0, 361, 90, 0, 20, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
        AttackModule::clear(fighter.module_accessor, 3, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_specialhi", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_specialhi_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
        if marth_is_unstance(fighter) {
            macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 0.2, 0.2);
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("marth_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("marth_dolphin_swing"), -1);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, -10, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 5);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

#[acmd_script( agent = "marth", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallw(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
        }
        macros::FT_MOTION_RATE(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

#[acmd_script( agent = "marth", script = "game_speciallwenter", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwenter(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 2.0);
}

#[acmd_script( agent = "marth", script = "game_speciallwexit", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwexit(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 2.0);
}

#[acmd_script( agent = "marth", script = "game_speciallwairenter", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwairenter(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 2.0);
}

#[acmd_script( agent = "marth", script = "game_speciallwairexit", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwairexit(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 2.0);
}

pub fn install() {
    install_acmd_scripts!(
        marth_specialnend, marth_specialnend_eff,
        marth_specialnendhi, marth_specialnendhi_eff,
        marth_specialnendlw, marth_specialnendlw_eff,
        marth_specialnendmax, marth_specialnendmax_eff,
        marth_specialnendmaxhi, marth_specialnendmaxhi_eff,
        marth_specialnendmaxlw, marth_specialnendmaxlw_eff,
        marth_specialhi, marth_specialhi_eff,
        marth_speciallw,
        marth_speciallwenter,
        marth_speciallwexit,
        marth_speciallwairenter,
        marth_speciallwairexit
    );
}