use crate::imports::acmd_imports::*;

#[acmd("reflet", ["game_specialhi", "game_specialairhi"])]
unsafe fn reflet_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES); // Was ALWAYS_BOTH_SIDES
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
}

#[acmd("reflet", [ "game_specialhi2", "game_specialairhi2" ])]
unsafe fn reflet_specialhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        // notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES); // Was ALWAYS_BOTH_SIDES
    }
}

pub fn install() {
    reflet_specialhi::install();
    reflet_specialhi2::install();
}