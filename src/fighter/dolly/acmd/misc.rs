use crate::imports::acmd_imports::*;

unsafe extern "C" fn dolly_guarddamage_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_guard"));
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.sound_acmd("sound_guarddamage", dolly_guarddamage_snd);
}