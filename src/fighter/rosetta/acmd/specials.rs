use crate::imports::acmd_imports::*;

#[acmd_script( agent = "rosetta", scripts = [ "game_specialhistart", "game_specialairhistart" ], category = ACMD_GAME, low_priority )]
unsafe fn rosetta_specialhistart(fighter: &mut L2CAgentBase) {
    MiscModule::calc_motion_rate_from_end_frame(fighter, 0.0, 8.0);
}

#[acmd_script( agent = "rosetta", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn rosetta_specialhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS); // Vanilla
    }
}

#[acmd_script( agent = "rosetta", script = "game_specialhiend", category = ACMD_GAME, low_priority )]
unsafe fn rosetta_specialhiend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES); // Vanilla
    }
}

pub fn install() {
    install_acmd_scripts!(
        rosetta_specialhistart,
        rosetta_specialhi,
        rosetta_specialhiend
    );
}