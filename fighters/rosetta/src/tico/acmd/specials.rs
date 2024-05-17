use super::*;

unsafe extern "C" fn game_specialhistart(agent: &mut L2CAgentBase) {
    MiscModule::calc_motion_rate_from_end_frame(agent, 0.0, 8.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhistart", game_specialhistart, Priority::Low);

    agent.acmd("game_specialairhistart", game_specialhistart, Priority::Low);

}