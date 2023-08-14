use crate::imports::acmd_imports::*;

#[acmd("kirby", "effect_jackspecialnjump")]
unsafe fn jack_jackspecialnjump_eff(_agent: &mut L2CAgentBase) {
}

#[acmd("kirby", "sound_jackspecialnjump")]
unsafe fn kirby_jackspecialnjump_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

pub fn install() {
    jack_jackspecialnjump_eff::install();
    kirby_jackspecialnjump_snd::install();
}
