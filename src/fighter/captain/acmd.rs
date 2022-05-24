use std::arch::asm;
use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "captain", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn captain_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    frame(fighter.lua_state_agent, 53.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
    }
}

#[acmd_script( agent = "captain", script = "game_specialnturn", category = ACMD_GAME, low_priority )]
unsafe fn captain_specialnturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
    }
}

#[acmd_script( agent = "captain", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn captain_specialairn(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &mut Vector3f{x: 0.0, y: 0.2, z: 0.0});
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_TURN);
    }
    frame(fighter.lua_state_agent, 53.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
    }
}

#[acmd_script( agent = "captain", script = "game_specialairnturn", category = ACMD_GAME, low_priority )]
unsafe fn captain_specialairnturn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
    }
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_DIR_DECIDE);
        WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_PUNCH_AIR_PHASE);
    }
    frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_FLAG_FALCON_PUNCH_GENERATE_BIRD);
    }
}

#[acmd_script( agent = "captain_falconpunch", scripts = [ "game_specialn", "game_specialairn", "game_specialnturn", "game_specialairnturn" ], category = ACMD_GAME, low_priority )]
unsafe fn captain_falconpunch_specialn(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 15.0, 361, 65, 0, 85, 5.0, 0.0, 0.0, 9.0, Some(0.0), Some(0.0), Some(-2.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    macros::FT_MOTION_RATE(fighter, 1.2);
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 12.0, 361, 65, 0, 85, 5.0, 0.0, 0.0, 9.0, Some(0.0), Some(0.0), Some(-2.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "captain_falconpunch", scripts = [ "effect_specialn", "effect_specialairn", "effect_specialnturn", "effect_specialairnturn" ], category = ACMD_EFFECT, low_priority )]
unsafe fn captain_falconpunch_specialn_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 15.0, 361, 65, 0, 85, 5.0, 0.0, 0.0, 9.0, Some(0.0), Some(0.0), Some(-2.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("captain_fp_body_r"), Hash40::new("captain_fp_body_l"), Hash40::new("center"), 0, 0, -7, 0, 0, 0, 1, true, *EF_FLIP_XY);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("captain_fp_wing_r"), Hash40::new("captain_fp_wing_r"), Hash40::new("wingr2"), -6, 0, -2, -10, -10, 0, 0.7, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("captain_fp_wing_l"), Hash40::new("captain_fp_wing_l"), Hash40::new("wingl2"), 6, 0, -2, -10, 10, 0, 0.7, true, *EF_FLIP_YZ);
    }
    macros::FT_MOTION_RATE(fighter, 1.2);
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 12.0, 361, 65, 0, 85, 5.0, 0.0, 0.0, 9.0, Some(0.0), Some(0.0), Some(-2.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("captain_fp_body"), -1);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("captain_fp_wing_r"), -1);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("captain_fp_wing_l"), -1);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        captain_specialn,
        captain_specialnturn,
        captain_specialairn,
        captain_specialairnturn,
        // captain_falconpunch_specialn,
        captain_falconpunch_specialn_eff
    );
}