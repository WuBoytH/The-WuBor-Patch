use crate::imports::acmd_imports::*;

unsafe extern "C" fn rosetta_tico_specialhistart(agent: &mut L2CAgentBase) {
    MiscModule::calc_motion_rate_from_end_frame(agent, 0.0, 8.0);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_specialhistart", rosetta_tico_specialhistart);

    agent.game_acmd("game_specialairhistart", rosetta_tico_specialhistart);

}