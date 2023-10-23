use crate::imports::acmd_imports::*;
use super::super::helper::*;

#[acmd_script( agent = "marth", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn marth_attack11(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 3.0, 65, 30, 0, 30, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_S, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 3.0, 78, 30, 0, 30, 3.0, 0.5, 0.0, 1.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_S, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 5.0, 90, 30, 0, 55, 3.0, 1.0, 0.0, 7.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "marth", script = "effect_attack11", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_attack11_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 6, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

#[acmd_script( agent = "marth", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn marth_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 0, 0, Hash40::new("armr"), 4.0, 45, 75, 0, 62, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_S, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 4.0, 45, 75, 0, 62, 3.0, 1.0, 0.0, 2.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_S, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 6.0, 45, 75, 0, 62, 3.0, 2.5, -1.5, 7.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_attack12", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_attack12_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 8, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

#[acmd_script( agent = "marth", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn marth_attacks3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 9.0, 45, 75, 0, 62, 3.5, 1.0, 0.0, 2.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 9.0, 45, 75, 0, 62, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("sword1"), 12.0, 45, 75, 0, 62, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_attacks3", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_attacks3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 8, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

#[acmd_script( agent = "marth", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn marth_attackhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 6.0, 100, 100, 0, 65, 3.5, 0.0, 0.0, 0.7, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 5.0, 100, 100, 0, 65, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_S, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("colonells"), 5.0, 100, 100, 0, 65, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_S, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 10.0, 100, 100, 0, 65, 3.5, 0.0, 0.0, 6.7, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 4, 0, Hash40::new("sword1"), 5.0, 100, 100, 0, 65, 3.5, 0.0, 0.0, 0.7, Some(4.5), Some(0.0), Some(0.7), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 5, 0, Hash40::new("sword1"), 10.0, 100, 100, 0, 65, 3.5, 0.0, 0.0, 6.7, Some(6.0), Some(0.0), Some(6.7), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear(agent.module_accessor, 4, false);
        AttackModule::clear(agent.module_accessor, 5, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_cutup"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 6.0, 85, 100, 0, 52, 3.5, 0.0, 0.0, 0.7, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 5.0, 85, 100, 0, 52, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_S, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 2, 0, Hash40::new("colonells"), 5.0, 85, 100, 0, 52, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_S, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 10.0, 85, 100, 0, 52, 3.5, 0.0, 0.0, 6.7, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_attackhi3", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_attackhi3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 8, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 4);
    }
}

#[acmd_script( agent = "marth", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn marth_attacklw3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_game(agent, hash40("collision_attr_sting"), *COLLISION_SOUND_ATTR_CUTUP);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.0, 30, 40, 0, 40, 2.7, 0.0, 2.7, 15.7, Some(0.0), Some(4.4), Some(9.2), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_M, unstance.hit_sound, *ATTACK_REGION_SWORD);
        macros::ATTACK(agent, 1, 0, Hash40::new("sword1"), 10.0, 30, 40, 0, 50, 2.7, 0.0, 0.0, 8.2, Some(0.0), Some(0.0), Some(7.2), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(unstance.hit_effect), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARTH_SWORD, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(agent.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "marth", script = "effect_attacklw3", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_attacklw3_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        let unstance = marth_unstance_effect(agent, hash40("tex_marth_sword1"), hash40("tex_marth_sword2"), hash40("marth_sword_blue"));
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new_raw(unstance.trail1), Hash40::new_raw(unstance.trail2), 8, Hash40::new("sword1"), 0.0, 0.0, 0.5, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new_raw(unstance.swordflare), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 2);
    }
}

pub fn install() {
    install_acmd_scripts!(
        marth_attack11,
        marth_attack11_eff,

        marth_attack12,
        marth_attack12_eff,

        marth_attacks3,
        marth_attacks3_eff,

        marth_attackhi3,
        marth_attackhi3_eff,

        marth_attacklw3,
        marth_attacklw3_eff
    );
}