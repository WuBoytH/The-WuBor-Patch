use crate::imports::acmd_imports::*;

unsafe extern "C" fn wiifit_specialhijump(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
}

unsafe extern "C" fn wiifit_specialhiend(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_HULAHOOP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_specialhijump", wiifit_specialhijump);

    agent.game_acmd("game_specialhiend", wiifit_specialhiend);
}