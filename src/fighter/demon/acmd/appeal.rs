use crate::imports::acmd_imports::*;

unsafe extern "C" fn demon_appealhil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 40.0);
    let hold_button = VarModule::get_int(agent.module_accessor, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                false,
                hash40("appeal_hi_l_loop"),
                40
            );
        }
    }
}

unsafe extern "C" fn demon_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 46.0);
    let hold_button = VarModule::get_int(agent.module_accessor, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                false,
                hash40("appeal_hi_r_loop"),
                46
            );
        }
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_appealhil", demon_appealhil);

    agent.game_acmd("game_appealhir", demon_appealhir);
}