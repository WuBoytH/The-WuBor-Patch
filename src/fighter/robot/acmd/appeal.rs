use crate::imports::acmd_imports::*;

unsafe extern "C" fn robot_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 38.0);
    let hold_button = VarModule::get_int(agent.module_accessor, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                true,
                hash40("appeal_hi_loop"),
                63
            );
        }
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_appealhil", robot_appealhi);

    agent.acmd("game_appealhir", robot_appealhi);
}