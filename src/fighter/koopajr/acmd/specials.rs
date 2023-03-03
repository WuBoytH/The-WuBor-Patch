use crate::imports::acmd_imports::*;

#[acmd_script( agent = "koopajr", scripts = ["game_speciallw", "game_specialairlw"], category = ACMD_GAME, low_priority )]
unsafe fn koopajr_speciallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_KOOPAJR_GENERATE_ARTICLE_MECHAKOOPA, false, 0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        koopajr_speciallw
    );
}