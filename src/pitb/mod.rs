// use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase/*, L2CFighterCommon*/};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
// use crate::system::IS_FUNNY;
// use crate::commonfuncs::*;

#[acmd_script( agent = "pitb", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn pitb_uspecial(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    macros::FT_MOTION_RATE(fighter, 1.0/11.0);
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BACK_ANGLE);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pitb", script = "game_specialairhiend", category = ACMD_GAME, low_priority )]
unsafe fn pitb_uspecialend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        pitb_uspecial,
        pitb_uspecialend
    );
}