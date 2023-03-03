use crate::imports::acmd_imports::*;

#[acmd_script( agent = "pacman", script = "game_specialhistart", category = ACMD_GAME, low_priority )]
unsafe fn pacman_specialhistart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_TRAMPOLINE_JUMP) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE, false, 0);
        }
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

pub fn install() {
    install_acmd_scripts!(
        pacman_specialhistart
    );
}