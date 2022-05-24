use std::arch::asm;
use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::wua_bind::*
};

#[acmd_script( agent = "robot", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME, low_priority )]
unsafe fn robot_appealhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 38.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        if macros::is_excute(fighter) {
            MiscModule::set_appeal_loop(
                fighter.module_accessor,
                true,
                hash40("appeal_hi_loop"),
                63,
                *CONTROL_PAD_BUTTON_APPEAL_HI
            );
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        robot_appealhi
    );
}