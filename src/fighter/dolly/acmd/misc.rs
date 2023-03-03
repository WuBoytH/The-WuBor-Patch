use crate::imports::acmd_imports::*;

#[acmd_script( agent = "dolly", script = "sound_guarddamage", category = ACMD_SOUND, low_priority )]
unsafe fn dolly_guarddamage_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_guard"));
    }
}

pub fn install() {
    install_acmd_scripts!(
        dolly_guarddamage_snd
    );
}