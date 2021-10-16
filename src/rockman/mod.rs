use smash::{
    lua2cpp::{L2CFighterCommon, L2CFighterBase, L2CAgentBase},
    phx::{Hash40, Vector3f},
    app::{lua_bind::*, sv_animcmd::*/*, **/},
    lib::lua_const::*
};
use smash_script::*;
use smashline::*;
use crate::{
    common_funcs::*,
    vars::*
};

#[fighter_frame( agent = FIGHTER_KIND_ROCKMAN )]
fn rockman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if IS_FUNNY[entry_id(fighter.module_accessor)] {
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3 {
                if MotionModule::frame(fighter.module_accessor) > 8.0 {
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_ROCKMAN_HARDKNUCKLE )]
fn samusd_cshot_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if IS_FUNNY[entry_id(weapon.module_accessor)] {
            if MotionModule::motion_kind(weapon.module_accessor) == smash::hash40("regular") {
                let slowdownvec : Vector3f = Vector3f{x: 0.,y: 0.9,z: 0.0};
                KineticModule::mul_speed(weapon.module_accessor, &slowdownvec, *WEAPON_KINETIC_TYPE_NORMAL);
            }
        }
    }
}

#[acmd_script( agent = "rockman_rockbuster", script = "game_regular", category = ACMD_GAME, low_priority )]
unsafe fn rockman_rockbuster_shoot(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        if !IS_FUNNY[entry_id(weapon.module_accessor)] {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 361, 30, 0, 22, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
        }
        else {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 3.0, 361, 0, 0, 0, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    frame(weapon.lua_state_agent, 4.0);
    if macros::is_excute(weapon) {
        if !IS_FUNNY[entry_id(weapon.module_accessor)] {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 361, 10, 0, 12, 2.2, 0.0, 0.0, 0.0, None, None, None, 0.75, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
    frame(weapon.lua_state_agent, 8.0);
    if macros::is_excute(weapon) {
        if !IS_FUNNY[entry_id(weapon.module_accessor)] {
            macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 2.0, 361, 10, 0, 8, 1.8, 0.0, 0.0, 0.0, None, None, None, 0.1, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_ENERGY);
        }
    }
}

pub fn install() {
    install_agent_frames!(
        rockman_frame
    );
    
    install_acmd_scripts!(
        rockman_rockbuster_shoot
    );
}