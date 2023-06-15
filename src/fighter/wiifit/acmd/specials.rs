use crate::imports::acmd_imports::*;

#[acmd_script( agent = "wiifit", script = "game_specialhijump", category = ACMD_GAME, low_priority )]
unsafe fn wiifit_specialhijump(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
    }
}

#[acmd_script( agent = "wiifit", script = "game_specialhiend", category = ACMD_GAME, low_priority )]
unsafe fn wiifit_specialhiend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot_exist(fighter.module_accessor, *FIGHTER_WIIFIT_GENERATE_ARTICLE_HULAHOOP, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

pub fn install() {
    install_acmd_scripts!(
        wiifit_specialhijump,
        wiifit_specialhiend
    );
}