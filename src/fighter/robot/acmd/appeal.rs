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

#[acmd_script( agent = "robot", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME )]
unsafe fn robot_appealhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 38.0);
    let hold_button = VarModule::get_int(fighter.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(fighter.module_accessor, hold_button) {
        if macros::is_excute(fighter) {
            MiscModule::set_appeal_loop(
                fighter.battle_object,
                true,
                hash40("appeal_hi_loop"),
                63
            );
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        robot_appealhi
    );
}