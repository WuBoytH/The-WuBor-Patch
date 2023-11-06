use crate::imports::acmd_imports::*;

#[acmd_script( agent = "samusd_supermissile", script = "game_ready", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn samusd_supermissile_ready(_agent: &mut L2CAgentBase) {
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_ready", samusd_supermissile_ready);
}