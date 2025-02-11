
use super::*;

unsafe extern "C" fn game_cliffcatch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 10.0 / 18.0);
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_cliffcatch", game_cliffcatch, Priority::Low);
}