use crate::imports::*;

unsafe extern "C" fn sound_guarddamage(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_guard"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_guarddamage", sound_guarddamage, Priority::Low);
}