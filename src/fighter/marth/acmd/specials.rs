use crate::imports::*;
use super::super::helper::*;

unsafe extern "C" fn game_specialnend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 7.5, 17.0, Some(0.0), Some(7.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 7.5, 23.5, Some(0.0), Some(7.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialnend(agent: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 7.5, 15, 0, 0, 0, 0.9, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

unsafe extern "C" fn game_specialnendhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialnendhi(agent: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 10.5, 15, -20, 0, 0, 0.9, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, -20, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

unsafe extern "C" fn game_specialnendlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 3.0, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 3.0, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 90, 0, 30, 0.9, 0.0, 2.0, 17.0, Some(0.0), Some(0.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 9.0, 361, 100, 0, 60, 0.9, 0.0, 0.0, 23.5, Some(0.0), Some(-1.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 25, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialnendlw(agent: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 4.5, 15, 12, 0, 0, 0.9, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 12, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

unsafe extern "C" fn game_specialnendmax(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_FIRE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(8.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 7.5, 17.0, Some(0.0), Some(7.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 7.5, 23.5, Some(0.0), Some(7.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialnendmax(agent: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 7.5, 15, 0, 0, 0, 0.9, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);

    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

unsafe extern "C" fn game_specialnendmaxhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_FIRE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 8.5, 8.0, Some(0.0), Some(12.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 14.0, 25.0, Some(0.0), Some(14.6), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 9.8, 17.0, Some(0.0), Some(12.3), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 13.0, 23.5, Some(0.0), Some(14.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(10.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 11.5, 25.0, Some(0.0), Some(12.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 9.0, 17.0, Some(0.0), Some(10.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 11.0, 23.5, Some(0.0), Some(12.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialnendmaxhi(agent: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 10.5, 15, -20, 0, 0, 0.9, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, -20, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);

    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

unsafe extern "C" fn game_specialnendmaxlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_FIRE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 6.5, 8.0, Some(0.0), Some(4.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, 3.0, 25.0, Some(0.0), Some(2.7), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 5.6, 17.0, Some(0.0), Some(3.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 3.0, 23.5, Some(0.0), Some(1.7), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 2, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 2.5, 0.0, 4.5, 8.0, Some(0.0), Some(1.5), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 2.5, 0.0, -0.5, 25.0, Some(0.0), Some(-1.0), Some(27.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 0, 0, Hash40::new("top"), 22.0, 361, 90, 0, 30, 0.7, 0.0, 2.0, 17.0, Some(0.0), Some(0.5), Some(21.9), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("top"), 24.0, 361, 100, 0, 60, 0.7, 0.0, 0.0, 23.5, Some(0.0), Some(-1.5), Some(28.6), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 50, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_marth_shield_breaker"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 0, false);
        AttackModule::clear(agent.module_accessor, 1, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_specialnendmaxlw(agent: &mut L2CAgentBase) {
    let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_purple"));
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_breaker_sting"), Hash40::new("top"), -0.0, 4.5, 15, 12, 0, 0, 0.9, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        macros::EFFECT(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 12, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
        else {
            macros::LAST_EFFECT_SET_COLOR(agent, 0.264, 0.47, 1.3);
        }
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);

    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    let unstance = marth_unstance_game(agent, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 11.0, 361, 74, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("top"), 11.0, 74, 74, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 7.0, 361, 90, 0, 20, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 7.0, 361, 90, 0, 20, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 2, false);
        AttackModule::clear(agent.module_accessor, 3, false);
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

unsafe extern "C" fn effect_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_swing"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, -10, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

unsafe extern "C" fn effect_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_dolphin_swing"), Hash40::new("top"), 0, 12, -1, 14, -30, 37, 1, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
    }
    frame(agent.lua_state_agent, 4.0);
    let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_dolphin_jump"), Hash40::new("top"), -0.0, 0, -5, 0, 0, 0, 1, true);
        if marth_is_unstance(agent) {
            macros::LAST_EFFECT_SET_COLOR(agent, 1.0, 0.2, 0.2);
        }
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_jump"), -1);
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("marth_dolphin_swing"), -1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("marth_dolphin_shadow"), Hash40::new("top"), -0.0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -2, -10, 15, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 5);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
        }
        macros::FT_MOTION_RATE(agent, 0.2);
    }
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

unsafe extern "C" fn game_speciallwenter(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 2.0);
}

unsafe extern "C" fn game_speciallwairenter(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 2.0);
}

unsafe extern "C" fn game_speciallwexit(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 2.0);
}

unsafe extern "C" fn game_speciallwairexit(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 2.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialnend", game_specialnend, Priority::Low);
    agent.acmd("effect_specialnend", effect_specialnend, Priority::Low);

    agent.acmd("game_specialnendhi", game_specialnendhi, Priority::Low);
    agent.acmd("effect_specialnendhi", effect_specialnendhi, Priority::Low);

    agent.acmd("game_specialnendlw", game_specialnendlw, Priority::Low);
    agent.acmd("effect_specialnendlw", effect_specialnendlw, Priority::Low);

    agent.acmd("game_specialnendmax", game_specialnendmax, Priority::Low);
    agent.acmd("effect_specialnendmax", effect_specialnendmax, Priority::Low);

    agent.acmd("game_specialnendmaxhi", game_specialnendmaxhi, Priority::Low);
    agent.acmd("effect_specialnendmaxhi", effect_specialnendmaxhi, Priority::Low);

    agent.acmd("game_specialnendmaxlw", game_specialnendmaxlw, Priority::Low);
    agent.acmd("effect_specialnendmaxlw", effect_specialnendmaxlw, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialhi", effect_specialhi, Priority::Low);

    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);
    agent.acmd("effect_specialairhi", effect_specialairhi, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);

    agent.acmd("game_specialairlw", game_speciallw, Priority::Low);

    agent.acmd("game_speciallwenter", game_speciallwenter, Priority::Low);

    agent.acmd("game_speciallwairenter", game_speciallwairenter, Priority::Low);

    agent.acmd("game_speciallwexit", game_speciallwexit, Priority::Low);

    agent.acmd("game_speciallwairexit", game_speciallwairexit, Priority::Low);
}