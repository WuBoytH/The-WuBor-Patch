// use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
// use crate::IS_FUNNY;
// use crate::globals::*;
// use crate::commonfuncs::*;

// #[fighter_frame( agent = FIGHTER_KIND_MIISWORDSMAN )]
// fn miisword_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
//         if StatusModule::status_kind(boma) == *FIGHTER_MIISWORDSMAN_STATUS_KIND_SPECIAL_S2_ATTACK
//         && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
//             if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
//             || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
//                 CancelModule::enable_cancel(boma);
//             }
//         }
//     }
// }

// Air Gale Stab can now grab the ledge after hitting the opponent.

#[acmd_script( agent = "miiswordsman", script = "game_specialairs2end", category = ACMD_GAME, low_priority )]
unsafe fn miisword_galestabend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MIISWORDSMAN_STATUS_SHIPPU_SLASH_FLAG_ATTACK_END);
    }
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    // smashline::install_agent_frames!(
    //     miisword_frame
    // );
    smashline::install_acmd_scripts!(
        miisword_galestabend
    );
}