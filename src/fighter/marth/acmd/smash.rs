use crate::imports::acmd_imports::*;
use super::super::helper::*;

#[acmd_script( agent = "marth", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn marth_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 13.0, 361, 75, 0, 48, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.0, 361, 75, 0, 48, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("bust"), 13.0, 361, 75, 0, 48, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 18.0, 361, 80, 0, 80, 3.5, 1.0, 0.0, 7.3, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_attacks4", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_attacks4_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 11, 0, 0, 180, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4)
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
}

#[acmd_script( agent = "marth", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn marth_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 13.0, 89, 90, 0, 45, 5.8, 0.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 17.0, 89, 95, 0, 40, 4.6, 0.0, 0.0, 7.3, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 13.0, 90, 90, 0, 45, 5.8, 0.0, 0.0, -3.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 125, 100, 155, 0, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_attackhi4", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_attackhi4_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 20, 0, -90, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        if !marth_is_unstance(fighter) {
            macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
        }
        else {
            macros::LAST_PARTICLE_SET_COLOR(fighter, 1, 0.6, 0.4);
        }
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 9, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 1);
    }
}

#[acmd_script( agent = "marth", script = "game_attacklw4", category = ACMD_GAME, low_priority )]
unsafe fn marth_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 8.0, 361, 88, 0, 60, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0, 361, 88, 0, 60, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 8.0, 361, 88, 0, 60, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 12.0, 361, 88, 0, 57, 3.5, 1.0, 0.0, 7.3, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.5);
    if macros::is_excute(fighter) {
        let unstance = marth_unstance_game(fighter, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 12.0, 361, 88, 0, 40, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.0, 361, 88, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 12.0, 361, 88, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 17.0, 361, 92, 0, 50, 3.5, 0.5, 0.0, 7.3, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_attacklw4", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_attacklw4_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    let unstance = marth_unstance_effect(fighter, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_sword_light"), Hash40::new("sword1"), 0, 0, 12.6, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("null"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("marth_sword_light"), Hash40::new("sword1"), 0, 0, 12.6, 0, 0, 0, 1, true);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::AFTER_IMAGE_OFF(fighter, 4);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("marth_sword_light"), false, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(unstance.swordflare), false, true);
    }
}

pub fn install() {
    install_acmd_scripts!(
        marth_attacks4, marth_attacks4_eff,
        marth_attackhi4, marth_attackhi4_eff,
        marth_attacklw4, marth_attacklw4_eff
    );
}