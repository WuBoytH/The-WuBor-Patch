use crate::imports::acmd_imports::*;

unsafe extern "C" fn rosetta_specialhistart(agent: &mut L2CAgentBase) {
    MiscModule::calc_motion_rate_from_end_frame(agent, 0.0, 8.0);
}

unsafe extern "C" fn rosetta_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS); // Vanilla
    }
}

unsafe extern "C" fn rosetta_specialhiend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES); // Vanilla
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhistart", rosetta_specialhistart);

    agent.game_acmd("game_specialairhistart", rosetta_specialhistart);

    agent.game_acmd("game_specialhi", rosetta_specialhi);

    agent.game_acmd("game_specialhiend", rosetta_specialhiend);
}