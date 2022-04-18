use {
    smash::{
        lua2cpp::L2CAgentBase,
        // phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::super::vars::*
};

#[acmd_script( agent = "marth", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallw(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_MARTH_INSTANCE_WORK_ID_FLAG_PARRY_XLU) {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
        }
        macros::FT_MOTION_RATE(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

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
        marth_speciallw,
        marth_speciallwenter,
        marth_speciallwexit
    );
}