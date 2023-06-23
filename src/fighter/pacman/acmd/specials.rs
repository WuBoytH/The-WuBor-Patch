use crate::imports::acmd_imports::*;

#[acmd_script( agent = "pacman", script = "game_specialhistart", category = ACMD_GAME, low_priority )]
unsafe fn pacman_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_TRAMPOLINE_JUMP) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_TRAMPOLINE, false, 0);
        }
    }
    macros::FT_MOTION_RATE(agent, 2.0);
    frame(agent.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install() {
    install_acmd_scripts!(
        pacman_specialhistart
    );
}