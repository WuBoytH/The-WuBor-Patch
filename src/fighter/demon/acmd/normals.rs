use crate::imports::acmd_imports::*;

#[acmd_script( agent = "demon", script = "game_attacklw3cancel", category = ACMD_GAME, low_priority )]
unsafe fn demon_attacklw3cancel(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        demon_attacklw3cancel
    );
}