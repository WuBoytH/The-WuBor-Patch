use crate::imports::acmd_imports::*;

unsafe extern "C" fn samusd_supermissile_ready(_agent: &mut L2CAgentBase) {
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_ready", samusd_supermissile_ready);
}