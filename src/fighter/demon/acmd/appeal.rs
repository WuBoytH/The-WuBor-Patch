use crate::imports::acmd_imports::*;

#[acmd_script( agent = "demon", script = "game_appealhil", category = ACMD_GAME, low_priority )]
unsafe fn demon_appealhil(agent: &mut L2CAgentBase) {
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

#[acmd_script( agent = "demon", script = "game_appealhir", category = ACMD_GAME, low_priority )]
unsafe fn demon_appealhir(agent: &mut L2CAgentBase) {
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

pub fn install() {
    install_acmd_scripts!(
        demon_appealhil,

        demon_appealhir
    );
}