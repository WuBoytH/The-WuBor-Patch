use super::*;

pub fn install(agent: &mut Agent) {
    agent.acmd("game_sticktarget", acmd_stub, Priority::Low);
}