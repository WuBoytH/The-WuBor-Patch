use smash::{
    lua2cpp::{/*L2CFighterCommon, */L2CAgentBase},
    // hash40,
    phx::Hash40,
    app::{lua_bind::*, sv_animcmd::*/*, **/},
    lib::lua_const::*
};
use smash_script::*;
use smashline::*;

#[acmd_script( agent = "trail", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn trail_dair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(fighter, 13.0 / 15.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            // 0x8A = COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 0.8, 367, 50, 40, 10, 3.2, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 0.8, 367, 50, 40, 10, 3.8, 0.4, 0.0, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 0.8, 367, 50, 40, 10, 3.8, 0.4, 3.2, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 0.8, 367, 50, 40, 10, 3.8, 0.4, 6.4, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("waist"), 0.8, 367, 50, 40, 10, 3.2, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 0, Hash40::new("haver"), 0.8, 367, 50, 40, 10, 3.8, 0.4, 0.0, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 6, 0, Hash40::new("haver"), 0.8, 367, 50, 40, 10, 3.8, 0.4, 3.2, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 7, 0, Hash40::new("haver"), 0.8, 367, 50, 40, 10, 3.8, 0.4, 6.4, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 6, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
        }
        wait(fighter.lua_state_agent, 7.0);
    }
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("waist"), 6.0, 58, 90, 0, 66, 3.2, 0.0, 0.0, 0.0, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 6.0, 58, 90, 0, 66, 3.8, 0.4, 0.0, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 6.0, 58, 90, 0, 66, 3.8, 0.4, 3.2, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("haver"), 6.0, 58, 90, 0, 66, 3.8, 0.4, 6.4, 0.8, None, None, None, 0.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, 0x8A, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 45.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "trail", script = "game_landingairlw", category = ACMD_GAME, low_priority )]
unsafe fn trail_dairland(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "trail", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn trail_dspecialattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        // 0x1000007 = FIGHTER_TRAIL_STATUS_SPECIAL_LW_FLOAT_ATTACK_POWER
        WorkModule::set_float(fighter.module_accessor, 0.0, 0x1000007);
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        // 0x8A = COLLISION_SOUND_ATTR_TRAIL_CLEAVE_SINGLE
        macros::ATTACK(fighter, 0, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, 0x8A, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 4.6, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, 0x8A, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("haver"), 5.0, 86, 20, 0, 100, 3.2, 0.0, 9.2, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, 0x8A, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 100, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(9.0), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, 0x8A, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 10.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 10.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 10.0, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
        WorkModule::on_flag(fighter.module_accessor, 0x2100000D);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 80, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(13.0), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, 0x8A, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 86, 20, 0, 80, 9.0, 9.0, 4.0, 11.5, Some(0.0), Some(16.5), Some(11.5), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, 0x8A, *ATTACK_REGION_SWORD);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        trail_dair,
        trail_dairland,
        trail_dspecialattack
    );
}