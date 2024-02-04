use crate::imports::acmd_imports::*;

unsafe extern "C" fn shizue_slingshot_attackairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 8.0 / 6.0);
    frame(agent.lua_state_agent, 9.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

unsafe extern "C" fn shizue_slingshot_attackairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, false, -1);
    }
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 11.0 / 9.0);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::shoot(agent.module_accessor, *WEAPON_SHIZUE_SLINGSHOT_GENERATE_ARTICLE_BULLET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attackairf", shizue_slingshot_attackairf);

    agent.acmd("game_attackairb", shizue_slingshot_attackairb);
}