use crate::imports::acmd_imports::*;

#[acmd_script( agent = "sonic", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn sonic_specialhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_GIMMICKJUMP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP); // Was ALWAYS
    }
}

pub fn install() {
    install_acmd_scripts!(
        sonic_specialhi
    );
}