use super::*;

unsafe extern "C" fn expression_landinglight(agent: &mut L2CAgentBase) {
    let expression_common : &mut smash::lua2cpp::L2CFighterAnimcmdExpressionCommon = std::mem::transmute(&mut *agent);
    expression_common.expression_LandingLightRumble();
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        // macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
}

unsafe extern "C" fn expression_landingheavy(agent: &mut L2CAgentBase) {
    let expression_common : &mut smash::lua2cpp::L2CFighterAnimcmdExpressionCommon = std::mem::transmute(&mut *agent);
    expression_common.expression_LandingHeavyRumble();
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
        // macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("expression_landinglight", expression_landinglight, Priority::Low);

    agent.acmd("expression_landingheavy", expression_landingheavy, Priority::Low);
}