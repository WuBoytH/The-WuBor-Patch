use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch"), false, -1.0);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 0.5, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 69.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch_dash"), false, -1.0);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 1.0, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 79.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, Hash40::new("catch_turn"), false, -1.0);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 3.0, 0.0, 0.0, 3.5, Some(0.0), Some(0.0), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        ArticleModule::remove_exist(agent.module_accessor, *FIGHTER_LUCAS_GENERATE_ARTICLE_HIMOHEBI, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);

    agent.acmd("game_catchdash", game_catchdash, Priority::Low);

    agent.acmd("game_catchturn", game_catchturn, Priority::Low);
}