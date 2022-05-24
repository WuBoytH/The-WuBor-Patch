use std::arch::asm;
use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::super::vars::*
};

#[acmd_script( agent = "chrom", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn chrom_speciallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_CHROM_STATUS_SPECIAL_LW_FLAG_CHANGE_KINETIC);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 280, 80, 80, 0, 4.4, 0.0, 6.5, 12.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 280, 80, 80, 0, 4.4, 0.0, 6.5, 18.5, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 280, 80, 80, 0, 4.4, 0.0, 6.5, 6.0, None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

#[acmd_script( agent = "chrom", script = "effect_specialairlw", category = ACMD_EFFECT, low_priority )]
unsafe fn chrom_speciallw_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_chrom_sword1"), Hash40::new("tex_chrom_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("chrom_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind_s"), Hash40::new("top"), 0, 7, -2, 0, 180, -90, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 2);
        macros::LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
}

#[acmd_script( agent = "chrom", script = "sound_specialairlw", category = ACMD_SOUND, low_priority )]
unsafe fn chrom_speciallw_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_chrom_special_l01"));
    }
}

#[acmd_script( agent = "chrom", script = "expression_specialairlw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn chrom_speciallw_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

#[acmd_script( agent = "chrom", script = "game_speciallwhit", category = ACMD_GAME, low_priority )]
unsafe fn chrom_speciallwhit(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 65, 115, 0, 65, 10.0, 0.0, 6.0, 11.0, Some(0.0), Some(11.0), Some(11.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 80, 80, 0, 55, 4.0, 0.0, 4.0, 11.0, Some(0.0), Some(4.0), Some(30.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 80, 80, 0, 55, 5.0, 0.0, 5.0, 30.0, Some(0.0), Some(5.0), Some(50.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 80, 80, 0, 55, 6.0, 0.0, 6.0, 50.0, Some(0.0), Some(6.0), Some(70.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 3, false);
    }
}

#[acmd_script( agent = "chrom", script = "effect_speciallwhit", category = ACMD_EFFECT, low_priority )]
unsafe fn chrom_speciallwhit_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("chrom_tenku_landing"), Hash40::new("top"), 0, 0, 10, 70, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("chrom_tenku_landing"), Hash40::new("top"), 0, 0, 25, 70, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, true);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("chrom_tenku_landing"), Hash40::new("top"), 0, 0, 42, 70, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "chrom", script = "sound_speciallwhit", category = ACMD_SOUND, low_priority )]
unsafe fn chrom_speciallwhit_snd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_l"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_chrom_rnd_special_l"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_l"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_l"));
    }
}

#[acmd_script( agent = "chrom", script = "expression_speciallwhit", category = ACMD_EXPRESSION, low_priority )]
unsafe fn chrom_speciallwhit_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_explosion"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        chrom_speciallw, chrom_speciallw_eff, chrom_speciallw_snd, chrom_speciallw_exp,
        chrom_speciallwhit, chrom_speciallwhit_eff, chrom_speciallwhit_snd, chrom_speciallwhit_exp
    );
}