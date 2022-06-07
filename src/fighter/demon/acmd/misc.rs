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

#[acmd_script( agent = "demon", script = "game_appealhil", category = ACMD_GAME, low_priority )]
unsafe fn demon_appealhil(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        if macros::is_excute(fighter) {
            MiscModule::set_appeal_loop(
                fighter.battle_object,
                false,
                hash40("appeal_hi_l_loop"),
                40,
                *CONTROL_PAD_BUTTON_APPEAL_HI
            );
        }
    }
}

#[acmd_script( agent = "demon", script = "game_appealhir", category = ACMD_GAME, low_priority )]
unsafe fn demon_appealhir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 46.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        if macros::is_excute(fighter) {
            if macros::is_excute(fighter) {
                MiscModule::set_appeal_loop(
                    fighter.battle_object,
                    false,
                    hash40("appeal_hi_r_loop"),
                    46,
                    *CONTROL_PAD_BUTTON_APPEAL_HI
                );
            }
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        demon_appealhil,
        demon_appealhir
    );
}