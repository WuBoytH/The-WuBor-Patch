use crate::imports::acmd_imports::*;

#[acmd("rosetta", [ "game_specialhistart", "game_specialairhistart" ])]
unsafe fn rosetta_specialhistart(agent: &mut L2CAgentBase) {
    MiscModule::calc_motion_rate_from_end_frame(agent, 0.0, 8.0);
}

#[acmd("rosetta_tico", [ "game_specialhistart", "game_specialairhistart" ])]
unsafe fn rosetta_tico_specialhistart(agent: &mut L2CAgentBase) {
    MiscModule::calc_motion_rate_from_end_frame(agent, 0.0, 8.0);
}

#[acmd("rosetta", "game_specialhi")]
unsafe fn rosetta_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS); // Vanilla
    }
}

#[acmd("rosetta", "game_specialhiend")]
unsafe fn rosetta_specialhiend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES); // Vanilla
    }
}

pub fn install() {
    rosetta_specialhistart::install();
    rosetta_tico_specialhistart::install();
    rosetta_specialhi::install();
    rosetta_specialhiend::install();
}