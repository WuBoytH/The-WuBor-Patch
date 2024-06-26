use super::*;

unsafe extern "C" fn game_escapeair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        KineticModule::change_kinetic(agent.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_ESCAPEAIRDUMMY, false, -1);
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 32.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        ArticleModule::remove(agent.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_ESCAPEAIRDUMMY, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_escapeairslide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_escapeair", game_escapeair, Priority::Low);
    agent.acmd("effect_escapeair", acmd_stub, Priority::Low);

    agent.acmd("game_escapeairslide", game_escapeairslide, Priority::Low);
}