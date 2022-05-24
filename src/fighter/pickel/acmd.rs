use std::arch::asm;
use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
};

#[acmd_script( agent = "pickel", script = "game_specialsride", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialsride(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 10, 3.0, 0.0, 8.0, 4.5, Some(0.0), Some(4.0), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    macros::FT_MOTION_RATE(fighter, 1.25);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "pickel", script = "game_specialairsride", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialairsride(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    macros::FT_MOTION_RATE(fighter, 1.25);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "pickel_trolley", script = "game_specialsdrivepartial", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialsdrivepartial(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.5, 40, 45, 0, 45, 3.5, 0.0, 3.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::SEARCH(weapon, 1, 0, Hash40::new("top"), 5.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NI, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

#[acmd_script( agent = "pickel_trolley", script = "game_specialsdriveemptypartial", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialsdriveemptypartial(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 5.0, -6.0, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(weapon.module_accessor, 1, true);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 0.0, 6.0, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(weapon.module_accessor, 2, true);
        macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 3.5, 0.0, 3.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IIE, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(weapon, 4, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 3.5, 0.0, 3.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_down_only(weapon.module_accessor, 4, true);
        macros::SEARCH(weapon, 0, 0, Hash40::new("top"), 3.5, 0.0, 5.0, 1.5, Some(0.0), Some(5.0), Some(-1.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
    }
    wait(weapon.lua_state_agent, 6.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 5.0, 6.0, Some(0.0), Some(0.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(weapon.module_accessor, 0, true);
        macros::ATTACK(weapon, 5, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.5, 0.0, 3.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 60, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::SEARCH(weapon, 0, 0, Hash40::new("top"), 3.5, 0.0, 3.0, 1.5, Some(0.0), Some(3.0), Some(-1.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        macros::SEARCH(weapon, 1, 0, Hash40::new("top"), 5.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NI, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

#[acmd_script( agent = "pickel_forge", script = "game_fallattack", category = ACMD_GAME, low_priority )]
unsafe fn pickel_forge_fallattack(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 10.0, 58, 78, 0, 62, 3.0, 0.0, 3.0, -2.5, Some(0.0), Some(3.0), Some(2.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 10.0, 58, 78, 0, 62, 3.0, 0.0, 3.0, -2.5, Some(0.0), Some(3.0), Some(2.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(weapon.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

#[acmd_script( agent = "pickel_forge", script = "game_fallattackride", category = ACMD_GAME, low_priority )]
unsafe fn pickel_forge_fallattackride(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 18.0, 58, 84, 0, 62, 3.0, 0.0, 3.0, -2.5, Some(0.0), Some(3.0), Some(2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 18.0, 58, 84, 0, 62, 3.0, 0.0, 3.0, -2.5, Some(0.0), Some(3.0), Some(2.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(weapon.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

#[acmd_script( agent = "pickel_forge", script = "game_wait", category = ACMD_GAME, low_priority )]
unsafe fn pickel_forge_wait(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::QUAKE(weapon, *CAMERA_QUAKE_KIND_M);
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 58, 78, 0, 58, 3.0, 0.0, 3.0, -3.0, Some(0.0), Some(3.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_OBJECT);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 5.0, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 10, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_search"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(weapon.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    frame(weapon.lua_state_agent, 1.0);
    if macros::is_excute(weapon) {
        ControlModule::set_rumble(weapon.module_accessor, Hash40::new("rbkind_77_kanatoko_fall"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        AttackModule::clear(weapon.module_accessor, 0, false);
        AttackModule::clear(weapon.module_accessor, 1, false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pickel_specialsride,
        pickel_specialairsride,
        pickel_specialsdrivepartial,
        pickel_specialsdriveemptypartial,
        pickel_forge_fallattack,
        pickel_forge_fallattackride,
        pickel_forge_wait
    );
}