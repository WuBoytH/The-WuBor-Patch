use crate::imports::acmd_imports::*;

unsafe extern "C" fn jack_jackspecialnjump_eff(_agent: &mut L2CAgentBase) {
}

unsafe extern "C" fn kirby_jackspecialnjump_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.effect_acmd("effect_jackspecialnjump", jack_jackspecialnjump_eff);
    agent.sound_acmd("sound_jackspecialnjump", kirby_jackspecialnjump_snd);
}