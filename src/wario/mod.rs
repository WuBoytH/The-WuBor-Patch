// use smash::phx::Hash40;
use smash::hash40;
use smash::lua2cpp::{/*L2CAgentBase, */L2CFighterCommon};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::commonfuncs::*;
// use crate::IS_FUNNY;

pub static mut FINISH_SIGN : [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_WARIO )]
fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        if MotionModule::motion_kind(boma) == hash40("throw_b") {
            if MotionModule::frame(boma) >= 10.0 && MotionModule::frame(boma) < 57.0 {
                macros::SET_SPEED_EX(fighter, 1.1 * PostureModule::lr(boma) * ControlModule::get_stick_x(boma), 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
            if MotionModule::frame(boma) == 57.0 {
                macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }

        if get_player_number(boma) < 8 {
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
                FINISH_SIGN[get_player_number(boma)] = 0;
            }

            if (MotionModule::motion_kind(boma) == hash40("appeal_lw_l")
            || MotionModule::motion_kind(boma) == hash40("appeal_lw_r"))
            && MotionModule::frame(boma) == 10.0 {
                FINISH_SIGN[get_player_number(boma)] += 1;
                if FINISH_SIGN[get_player_number(boma)] > 15 {
                    FINISH_SIGN[get_player_number(boma)] = 15;
                }
            }

            if (StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_LW
            || StatusModule::status_kind(boma) != *FIGHTER_WARIO_STATUS_KIND_SPECIAL_LW_LANDING)
            && StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                FINISH_SIGN[get_player_number(boma)] = 0;
            }
        }
    }
}

// #[acmd_script( agent = "wario", scripts = ["game_speciallwmr", "game_specialairlwmr"], category = ACMD_GAME, low_priority )]
// unsafe fn wario_dspecialmid(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = sv_system::battle_object_module_accessor(lua_state);
//     
// }

pub fn install() {
    smashline::install_agent_frames!(
        wario_frame
    );
    // smashline::install_acmd_scripts!(
    //     wario_dspecialmid
    // );
}