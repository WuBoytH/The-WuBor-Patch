use crate::imports::acmd_imports::*;
use super::super::helper::*;

#[acmd_script( agent = "inkling", script = "game_escapeairslide", category = ACMD_GAME, low_priority )]
unsafe fn inkling_escapeairslide(fighter: &mut L2CAgentBase) {
    inkling_generate_squid_helper(fighter);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install() {
    install_acmd_scripts!(
        inkling_escapeairslide
    );
}