use super::*;

unsafe extern "C" fn game_appealhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        let is_bonker = VarModule::is_flag(agent.module_accessor, vars::mario::instance::flag::BONKER);
        VarModule::set_flag(agent.module_accessor, vars::mario::instance::flag::BONKER, !is_bonker);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_appealhir", game_appealhi, Priority::Low);

    agent.acmd("game_appealhil", game_appealhi, Priority::Low);
}