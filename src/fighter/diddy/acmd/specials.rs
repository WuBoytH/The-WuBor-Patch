use crate::imports::acmd_imports::*;

#[acmd("diddy", "game_specialairsjump")]
unsafe fn diddy_specialairsjump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 2);
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 2.5, 4.5, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 4.0, 0.0, 5.5, 4.0, None, None, None, *FIGHTER_STATUS_KIND_CLUNG_DIDDY, *COLLISION_SITUATION_MASK_GA);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 1);
    }
    frame(agent.lua_state_agent, 26.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR, 0);
    }
}

pub fn install() {
    diddy_specialairsjump::install();
}