use super::*;

unsafe extern "C" fn game_lookupwait1(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 2.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_lookupwait1", game_lookupwait1, Priority::Low);
}