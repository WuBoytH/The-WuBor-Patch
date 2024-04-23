use crate::imports::*;

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_dash", game_dash, Priority::Low);

    agent.acmd("game_turndash", game_turndash, Priority::Low);
}