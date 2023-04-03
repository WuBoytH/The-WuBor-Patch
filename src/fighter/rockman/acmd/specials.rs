use crate::imports::acmd_imports::*;

#[acmd_script( agent = "rockman", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn rockman_speciallw(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 5.0);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "rockman", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn rockman_specialairlw(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 5.0);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article_enable(fighter.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    install_acmd_scripts!(
        rockman_speciallw,
        rockman_specialairlw
    );
}