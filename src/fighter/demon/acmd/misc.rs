use {
    smash::{
        lua2cpp::L2CAgentBase,
        hash40,
        app::{lua_bind::*, sv_animcmd::*}
    },
    custom_var::*,
    smash_script::*,
    smashline::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[acmd_script( agent = "demon", script = "game_appealhil", category = ACMD_GAME )]
unsafe fn demon_appealhil(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    let hold_button = VarModule::get_int(fighter.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(fighter.module_accessor, hold_button) {
        if macros::is_excute(fighter) {
            MiscModule::set_appeal_loop(
                fighter.battle_object,
                false,
                hash40("appeal_hi_l_loop"),
                40
            );
        }
    }
}

#[acmd_script( agent = "demon", script = "game_appealhir", category = ACMD_GAME )]
unsafe fn demon_appealhir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 46.0);
    let hold_button = VarModule::get_int(fighter.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(fighter.module_accessor, hold_button) {
        if macros::is_excute(fighter) {
            if macros::is_excute(fighter) {
                MiscModule::set_appeal_loop(
                    fighter.battle_object,
                    false,
                    hash40("appeal_hi_r_loop"),
                    46
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