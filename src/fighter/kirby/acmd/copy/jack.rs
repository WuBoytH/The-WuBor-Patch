use crate::imports::*;

unsafe extern "C" fn effect_jackspecialnjump(_agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn sound_jackspecialnjump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_jackspecialnjump", effect_jackspecialnjump, Priority::Low);
    agent.acmd("sound_jackspecialnjump", sound_jackspecialnjump, Priority::Low);
}