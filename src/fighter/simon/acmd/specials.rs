use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "simon", scripts = ["game_specialn", "game_specialairn"], category = ACMD_GAME )]
unsafe fn simon_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, simon::status::flag::SPECIAL_N_SHOOT);
    }
}

#[acmd_script( agent = "simon", scripts = ["expression_specialn", "expression_specialairn"], category = ACMD_EXPRESSION )]
unsafe fn simon_specialn_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_lightthrow4item"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        );
    }
}

pub fn install() {
    install_acmd_scripts!(
        simon_specialn,
        simon_specialn_exp
    );
}