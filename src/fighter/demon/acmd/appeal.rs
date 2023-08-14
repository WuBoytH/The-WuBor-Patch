use crate::imports::acmd_imports::*;

#[acmd("demon", "game_appealhil")]
unsafe fn demon_appealhil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 40.0);
    let hold_button = VarModule::get_int(agent.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.battle_object,
                false,
                hash40("appeal_hi_l_loop"),
                40
            );
        }
    }
}

#[acmd("demon", "game_appealhir")]
unsafe fn demon_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 46.0);
    let hold_button = VarModule::get_int(agent.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.battle_object,
                false,
                hash40("appeal_hi_r_loop"),
                46
            );
        }
    }
}

pub fn install() {
    demon_appealhil::install();
    demon_appealhir::install();
}