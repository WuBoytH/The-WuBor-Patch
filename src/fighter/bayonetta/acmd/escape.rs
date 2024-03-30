use crate::imports::*;

unsafe extern "C" fn bayonetta_escapef(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2ea0f68425), false);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
}

unsafe extern "C" fn bayonetta_escapeairslide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_escapef", bayonetta_escapef);

    agent.acmd("game_escapeairslide", bayonetta_escapeairslide);
}