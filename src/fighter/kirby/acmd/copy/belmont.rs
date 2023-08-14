use crate::imports::acmd_imports::*;

#[acmd("kirby", [ "expression_simonspecialn", "expression_richterspecialn" ])]
unsafe fn kirby_belmontspecialn_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd("kirby", [ "expression_simonspecialairn", "expression_richterspecialairn" ])]
unsafe fn kirby_belmontspecialairn_exp(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    kirby_belmontspecialn_exp::install();
    kirby_belmontspecialairn_exp::install();
}