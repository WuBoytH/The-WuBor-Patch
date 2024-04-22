use crate::imports::*;

unsafe extern "C" fn null(_agent: &mut L2CAgentBase) {
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_sticktarget", null);
}