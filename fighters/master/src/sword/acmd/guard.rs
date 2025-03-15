use super::*;

unsafe extern "C" fn game_guardcancelattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 6.0 / 2.0);
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_guardcancelattack", game_guardcancelattack, Priority::Low);
}