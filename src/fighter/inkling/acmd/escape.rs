use crate::imports::*;
use super::super::helper::*;

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    inkling_generate_squid_helper(agent);
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);
}