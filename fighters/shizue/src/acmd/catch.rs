use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 5.5, 4.0, Some(0.0), Some(5.5), Some(14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::ENABLE_AREA(agent, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::UNABLE_AREA(agent, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_BUTTERFLYNET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.933);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 5.5, 4.0, Some(0.0), Some(5.5), Some(13.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::ENABLE_AREA(agent, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH_DASH);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::UNABLE_AREA(agent, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH_DASH);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 5.0, 0.0, 5.5, -5.0, Some(0.0), Some(5.5), Some(-14.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::ENABLE_AREA(agent, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH_TURN);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::UNABLE_AREA(agent, *FIGHTER_MURABITO_AREA_KIND_SEARCH_ITEM_CATCH_TURN);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);

    agent.acmd("game_catchdash", game_catchdash, Priority::Low);

    agent.acmd("game_catchturn", game_catchturn, Priority::Low);
}