use crate::imports::acmd_imports::*;

#[acmd_script( agent = "kirby", script = "effect_jackspecialnjump", category = ACMD_EFFECT, low_priority )]
unsafe extern "C" fn jack_jackspecialnjump_eff(_agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "kirby", script = "sound_jackspecialnjump", category = ACMD_SOUND, low_priority )]
unsafe extern "C" fn kirby_jackspecialnjump_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    for _ in 0..4 {
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_common_swing_04"));
        }
        wait(agent.lua_state_agent, 10.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        jack_jackspecialnjump_eff,
        kirby_jackspecialnjump_snd
    );
}
