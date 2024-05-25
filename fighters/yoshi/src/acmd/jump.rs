use super::*;

unsafe extern "C" fn sound_jumpaerial(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_yoshi_jump01"));
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_yoshi_rnd_jump01"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("sound_jumpaerialfront", sound_jumpaerial, Priority::Low);

    agent.acmd("sound_jumpaerialback", sound_jumpaerial, Priority::Low);
}