// use smash::phx::Hash40;
// use smash::hash40;
use smash::lua2cpp::L2CAgentBase;
// use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
// use crate::system::IS_FUNNY;
// use crate::commonfuncs::*;

#[acmd_script( agent = "pacman", script = "game_specialhistart", category = ACMD_GAME, low_priority )]
unsafe fn pacman_uspecialstart(fighter: &mut L2CAgentBase) {
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
    smashline::install_acmd_scripts!(
        pacman_uspecialstart
    );
}