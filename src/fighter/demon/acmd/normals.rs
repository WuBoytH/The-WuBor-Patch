use {
    smash::{
        lua2cpp::L2CAgentBase,
        app::sv_animcmd::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "demon", script = "game_attacklw3cancel", category = ACMD_GAME )]
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