use {
    smash::{
        lua2cpp::L2CAgentBase,
        // phx::Hash40,
        // app::{lua_bind::*, sv_animcmd::*},
        // lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "marth", script = "game_speciallwenter", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwenter(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 2.0);
}

#[acmd_script( agent = "marth", script = "game_speciallwexit", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwexit(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 2.0);
}

pub fn install() {
    install_acmd_scripts!(
        marth_speciallwenter,
        marth_speciallwexit
    );
}