use crate::imports::*;

unsafe extern "C" fn game_ready(_agent: &mut L2CAgentBase) {
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_ready", game_ready);
}