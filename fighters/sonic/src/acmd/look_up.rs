use super::*;

unsafe extern "C" fn game_lookup(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_lookup", game_lookup, Priority::Low);
}