use crate::imports::acmd_imports::*;

#[acmd_script( agent = "brave", scripts = [ "game_specialhi1", "game_specialairhi1" ], category = ACMD_GAME, low_priority )]
unsafe fn brave_specialhi1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, -1);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

#[acmd_script( agent = "brave", scripts = [ "game_specialhi2", "game_specialairhi2" ], category = ACMD_GAME, low_priority )]
unsafe fn brave_specialhi2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, -1);
    }
    wait(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 40.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

#[acmd_script( agent = "brave", scripts = [ "game_specialhi3", "game_specialairhi3" ], category = ACMD_GAME, low_priority )]
unsafe fn brave_specialhi3(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_BRAVE_GENERATE_ARTICLE_TORNADO, false, -1);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_JUMP_START);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_BRAVE_STATUS_SPECIAL_HI_FLAG_REVERT_ANGLE);
    }
}

pub fn install() {
    install_acmd_scripts!(
        brave_specialhi1,
        brave_specialhi2,
        brave_specialhi3
    );
}