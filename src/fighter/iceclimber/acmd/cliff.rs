use crate::imports::*;

unsafe extern "C" fn game_cliffcatch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

unsafe extern "C" fn game_cliffcatch_nana(agent: &mut L2CAgentBase) {
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_POPO_STATUS_CLIFF_CATCH_MOVE_FLAG_SPECIAL_HI_PARTNER) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        }
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_POPO_STATUS_CLIFF_CATCH_FLAG_TO_PULL_PARTNER);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_cliffcatch", game_cliffcatch);
    agent.acmd("game_cliffcatch_nana", game_cliffcatch_nana);
}