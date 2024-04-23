use crate::imports::*;

unsafe extern "C" fn game_escapen(_agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn game_escapef(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
}

unsafe extern "C" fn game_escapeb(_agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
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

unsafe extern "C" fn game_escapeairslideforesight(agent: &mut L2CAgentBase) {
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

pub fn install(agent: &mut Agent) {
    agent.acmd("game_escapen", game_escapen, Priority::Low);

    agent.acmd("game_escapef", game_escapef, Priority::Low);

    agent.acmd("game_escapeb", game_escapeb, Priority::Low);

    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);

    agent.acmd("game_escapeairslideforesight", game_escapeairslideforesight, Priority::Low);
}