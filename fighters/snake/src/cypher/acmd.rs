use super::*;

unsafe extern "C" fn null(_agent: &mut L2CAgentBase) {
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_detach", null, Priority::Low);
}