use crate::imports::acmd_imports::*;

#[acmd_script( agent = "koopajr", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn koopajr_speciallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, 0);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_speciallw", koopajr_speciallw);

    agent.game_acmd("game_specialairlw", koopajr_speciallw);
}