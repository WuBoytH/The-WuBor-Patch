use crate::imports::acmd_imports::*;

#[acmd("dolly", "sound_guarddamage")]
unsafe fn dolly_guarddamage_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_guard"));
    }
}

pub fn install() {
    dolly_guarddamage_snd::install();
}