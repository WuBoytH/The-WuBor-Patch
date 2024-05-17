use super::*;

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_sticktarget", acmd_stub, Priority::Low);
}