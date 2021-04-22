use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
//use smash::phx::Vector3f;
//use smash::app::BattleObjectModuleAccessor;
//use smash::app::lua_bind::EffectModule;
use crate::IS_FUNNY;
use crate::commonfuncs::*;

static mut SPIN_SPEED : [f32; 8] = [1.56; 8];

#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);

        if IS_FUNNY[get_player_number(boma)] && SPIN_SPEED[get_player_number(boma)] != 3.0 {
            SPIN_SPEED[get_player_number(boma)] = 3.0;
        }
        else if !IS_FUNNY[get_player_number(boma)] && SPIN_SPEED[get_player_number(boma)] != 1.56 {
            SPIN_SPEED[get_player_number(boma)] = 1.56;
        }

        if MotionModule::motion_kind(boma) == smash::hash40("special_hi") {
            if MotionModule::frame(boma) > 6.0 && MotionModule::frame(boma) < 46.0 {
                let facing_dirn = PostureModule::lr(boma);
                if facing_dirn > 0.0 {
                    macros::SET_SPEED_EX(fighter, &SPIN_SPEED[get_player_number(boma)] * ControlModule::get_stick_x(boma), 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
                else{
                    macros::SET_SPEED_EX(fighter, -&SPIN_SPEED[get_player_number(boma)] * ControlModule::get_stick_x(boma), 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            }
        }
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn toonlink_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    macros::FT_MOTION_RATE(fighter, 0.7);
    sv_animcmd::frame(lua_state, 8.0);
    macros::FT_MOTION_RATE(fighter, 1.1);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 8.0, 82, 70, 0, 55, 4.2, 5.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.0, 82, 70, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.0, 82, 70, 0, 55, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(lua_state, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.6186);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 43.0);
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