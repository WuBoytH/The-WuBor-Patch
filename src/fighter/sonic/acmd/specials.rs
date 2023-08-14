use crate::imports::acmd_imports::*;

#[acmd("sonic", "game_specialhi")]
unsafe fn sonic_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::shoot_exist(agent.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP); // Was ALWAYS
    }
}

pub fn install() {
    sonic_specialhi::install();
}