use super::*;

unsafe extern "C" fn game_catchstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.5, 3.5);
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NO(agent, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 18, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 19, *HIT_STATUS_OFF);
    }
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -12.0, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn game_catchdashstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.5, 3.5);
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NO(agent, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 18, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 19, *HIT_STATUS_OFF);
    }
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -12.0, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

unsafe extern "C" fn game_catchturnstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area(agent.module_accessor, 3.5, 3.5);
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::HIT_NO(agent, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 18, *HIT_STATUS_OFF);
        macros::HIT_NO(agent, 19, *HIT_STATUS_OFF);
    }
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, -1.0, Some(0.0), Some(-1.0), Some(7.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("throw"), 2.8, 0.0, -1.0, 0.0, Some(0.0), Some(-1.0), Some(-1.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 51.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catchstart", game_catchstart, Priority::Low);

    agent.acmd("game_catchdashstart", game_catchdashstart, Priority::Low);

    agent.acmd("game_catchturnstart", game_catchturnstart, Priority::Low);
}