use crate::imports::acmd_imports::*;

#[acmd_script( agent = "reflet", scripts = ["game_specialhi", "game_specialairhi"], category = ACMD_GAME, low_priority )]
unsafe fn reflet_specialhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, 0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES); // Was ALWAYS_BOTH_SIDES
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_TRY_2ND);
    }
}

#[acmd_script( agent = "reflet", scripts = [ "game_specialhi2", "game_specialairhi2" ], category = ACMD_GAME, low_priority )]
unsafe fn reflet_specialhi2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_REFLET_GENERATE_ARTICLE_ELWIND, false, -1);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_REFLET_STATUS_SPECIAL_HI_FLAG_JUMP);
        // notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES); // Was ALWAYS_BOTH_SIDES
    }
}

pub fn install() {
    install_acmd_scripts!(
        reflet_specialhi,
        reflet_specialhi2
    );
}