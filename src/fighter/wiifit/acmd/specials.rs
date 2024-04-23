use crate::imports::*;

unsafe extern "C" fn game_specialhijump(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
}

unsafe extern "C" fn game_specialhiend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_HULAHOOP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_specialhijump", game_specialhijump, Priority::Low);

    agent.acmd("game_specialhiend", game_specialhiend, Priority::Low);
}