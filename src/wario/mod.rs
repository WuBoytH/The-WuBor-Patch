use smash::phx::Hash40;
use smash::hash40;
use smash::phx::Vector2f;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::commonfuncs::*;
use crate::IS_FUNNY;

static mut FINISH_SIGN : [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_WARIO )]
fn wario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        if get_player_number(boma) < 8 {
            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
                FINISH_SIGN[get_player_number(boma)] = 0;
            }

            if FINISH_SIGN[get_player_number(boma)] == 0 {
                WorkModule::set_int(boma, 0, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL);
            }
            else if FINISH_SIGN[get_player_number(boma)] >= 1
            && FINISH_SIGN[get_player_number(boma)] < 8 {
                WorkModule::set_int(boma, *FIGHTER_WARIO_GASS_LEVEL_M, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL);
            }
            else if FINISH_SIGN[get_player_number(boma)] >= 8
            && FINISH_SIGN[get_player_number(boma)] < 14 {
                WorkModule::set_int(boma, *FIGHTER_WARIO_GASS_LEVEL_L, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL);
            }
            else {
                WorkModule::set_int(boma, *FIGHTER_WARIO_GASS_LEVEL_FLY, *FIGHTER_WARIO_INSTANCE_WORK_ID_INT_GASS_LEVEL);
            }

            if (MotionModule::motion_kind(boma) == hash40("appeal_lw_l")
            || MotionModule::motion_kind(boma) == hash40("appeal_lw_r"))
            && MotionModule::frame(boma) == 10.0 {
                FINISH_SIGN[get_player_number(boma)] += 1;
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