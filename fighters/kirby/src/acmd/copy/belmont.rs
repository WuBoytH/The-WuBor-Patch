use super::*;

unsafe extern "C" fn expression_belmontspecialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn expression_belmontspecialairn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("expression_simonspecialn", expression_belmontspecialn, Priority::Low);

    agent.acmd("expression_simonspecialairn", expression_belmontspecialairn, Priority::Low);

    agent.acmd("expression_richterspecialn", expression_belmontspecialn, Priority::Low);

    agent.acmd("expression_richterspecialairn", expression_belmontspecialairn, Priority::Low);
}