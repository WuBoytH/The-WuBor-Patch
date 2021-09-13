use smash::{
    lua2cpp::{/*L2CFighterCommon, L2CFighterBase, */L2CAgentBase},
    // hash40,
    phx::{Hash40, Vector3f/*, Vector2f*/},
    app::{lua_bind::*, sv_animcmd::*, *},
    lib::lua_const::*
};
use smash_script::*;
use smashline::*;
// use crate::{
//     commonfuncs::*,
//     vars::*,
//     gameplay::*
// };

// #[inline(always)]
// pub unsafe fn captain_fgc(fighter: &mut L2CFighterCommon) {
//     let status = StatusModule::status_kind(fighter.module_accessor);
//     let mut allowed_cancels : Vec<i32> = [].to_vec();
//     set_hp(fighter, 90.0);
//     if [
//         *FIGHTER_STATUS_KIND_ATTACK,
//         *FIGHTER_STATUS_KIND_ATTACK_DASH
//     ].contains(&status) {
//         allowed_cancels = [
//             *FIGHTER_STATUS_KIND_ATTACK_S3,
//             *FIGHTER_STATUS_KIND_ATTACK_LW3,
//             *FIGHTER_STATUS_KIND_ATTACK_HI3,
//             *FIGHTER_STATUS_KIND_ATTACK_LW4,
//             *FIGHTER_STATUS_KIND_SPECIAL_N,
//             *FIGHTER_STATUS_KIND_SPECIAL_S,
//             *FIGHTER_STATUS_KIND_SPECIAL_LW,
//             *FIGHTER_STATUS_KIND_SPECIAL_HI
//         ].to_vec();
//     }
//     else if [
//         *FIGHTER_STATUS_KIND_ATTACK_S3,
//         *FIGHTER_STATUS_KIND_ATTACK_LW3,
//         *FIGHTER_STATUS_KIND_ATTACK_HI3,
//         *FIGHTER_STATUS_KIND_ATTACK_AIR
//     ].contains(&status) {
//         allowed_cancels = [
//             *FIGHTER_STATUS_KIND_ATTACK_LW4,
//             *FIGHTER_STATUS_KIND_SPECIAL_N,
//             *FIGHTER_STATUS_KIND_SPECIAL_S,
//             *FIGHTER_STATUS_KIND_SPECIAL_LW,
//             *FIGHTER_STATUS_KIND_SPECIAL_HI
//         ].to_vec();
//     }
//     if [
//         *FIGHTER_STATUS_KIND_ATTACK_HI3,
//         *FIGHTER_STATUS_KIND_ATTACK_LW4
//     ].contains(&status)
//     || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_b")
//     || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi") {
//         jump_cancel_check_hit(fighter, false);
//     }
//     if status == *FIGHTER_STATUS_KIND_ATTACK_S4 {
//         dash_cancel_check(fighter, false, false);
//     }
//     if status == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
//         dash_cancel_check(fighter, false, true);
//     }
//     cancel_system(fighter, status, allowed_cancels);
// }

// #[fighter_frame( agent = FIGHTER_KIND_CAPTAIN )]
// fn captain_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
        
//     }
// }

// #[weapon_frame( agent = WEAPON_KIND_CAPTAIN_FALCONPUNCH )]
// fn captain_falconpunch_frame(weapon: &mut L2CFighterBase) {
//     unsafe {
        
//     }
// }

#[acmd_script( agent = "captain", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn captain_nspecial(fighter: &mut L2CAgentBase) {
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
unsafe fn captain_nspecialturn(fighter: &mut L2CAgentBase) {
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
unsafe fn captain_nspecialair(fighter: &mut L2CAgentBase) {
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
unsafe fn captain_nspecialairturn(fighter: &mut L2CAgentBase) {
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
unsafe fn captain_falconpunch_nspecial(fighter: &mut L2CAgentBase) {
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
unsafe fn captain_falconpunch_nspecialeff(fighter: &mut L2CAgentBase) {
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
    // install_agent_frames!(
    //     captain_frame,
    //     captain_falconpunch_frame
    // );
    install_acmd_scripts!(
        captain_nspecial,
        captain_nspecialturn,
        captain_nspecialair,
        captain_nspecialairturn,
        // captain_falconpunch_nspecial,
        captain_falconpunch_nspecialeff
    );
}