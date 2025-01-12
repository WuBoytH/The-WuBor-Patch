use super::*;

unsafe extern "C" fn expression_landingairn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        // macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

unsafe extern "C" fn expression_landingairf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        // macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

unsafe extern "C" fn expression_landingairb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        // macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

unsafe extern "C" fn expression_landingairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        // macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_landl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 7);
    }
}

unsafe extern "C" fn expression_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        // macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_impact"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 7);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("expression_landingairn", expression_landingairn, Priority::Low);

    agent.acmd("expression_landingairf", expression_landingairf, Priority::Low);

    agent.acmd("expression_landingairb", expression_landingairb, Priority::Low);

    agent.acmd("expression_landingairhi", expression_landingairhi, Priority::Low);

    agent.acmd("expression_landingairlw", expression_landingairlw, Priority::Low);
}