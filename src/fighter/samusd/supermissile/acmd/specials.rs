use crate::imports::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_ready", acmd_stub, Priority::Low);
}