use crate::imports::acmd_imports::*;

#[acmd("koopajr", ["game_speciallw", "game_specialairlw"])]
unsafe fn koopajr_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, 0);
    }
}

pub fn install() {
    koopajr_speciallw::install();
}