use crate::imports::acmd_imports::*;

unsafe extern "C" fn kirby_belmontspecialn_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn kirby_belmontspecialairn_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.expression_acmd("expression_simonspecialn", kirby_belmontspecialn_exp);

    agent.expression_acmd("expression_simonspecialairn", kirby_belmontspecialairn_exp);

    agent.expression_acmd("expression_richterspecialn", kirby_belmontspecialn_exp);

    agent.expression_acmd("expression_richterspecialairn", kirby_belmontspecialairn_exp);
}