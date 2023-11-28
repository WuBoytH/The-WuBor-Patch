use crate::imports::acmd_imports::*;

unsafe extern "C" fn ryu_cliffcatch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_cliffcatch", ryu_cliffcatch);
}