use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
// use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
//use smash::phx::Vector3f;
use crate::system::IS_FUNNY;
use crate::commonfuncs::*;

static mut SPIN_SPEED : [f32; 8] = [1.56; 8];

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if IS_FUNNY[entry_id(fighter.module_accessor)] && SPIN_SPEED[entry_id(fighter.module_accessor)] != 3.0 {
            SPIN_SPEED[entry_id(fighter.module_accessor)] = 3.0;
        }
        else if !IS_FUNNY[entry_id(fighter.module_accessor)] && SPIN_SPEED[entry_id(fighter.module_accessor)] != 1.56 {
            SPIN_SPEED[entry_id(fighter.module_accessor)] = 1.56;
        }

        // Toon Link can now move during his grounded Spin Attack.

        if MotionModule::motion_kind(fighter.module_accessor) == smash::hash40("special_hi") {
            if MotionModule::frame(fighter.module_accessor) > 6.0 && MotionModule::frame(fighter.module_accessor) < 46.0 {
                let stickx = ControlModule::get_stick_x(fighter.module_accessor);
                let lr = PostureModule::lr(fighter.module_accessor);
                let speed = SPIN_SPEED[entry_id(fighter.module_accessor)];
                macros::SET_SPEED_EX(fighter, lr * speed * stickx, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_dashattack(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, 1.1);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 8.0, 82, 70, 0, 55, 4.2, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.0, 82, 70, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 82, 70, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.6186);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 43.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    smashline::install_agent_frames!(
        toonlink_frame
    );
    smashline::install_acmd_scripts!(
        toonlink_dashattack
    );
}