use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "kirby", scripts = [ "expression_simonspecialn", "expression_richterspecialn" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn kirby_belmontspecialn_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script( agent = "kirby", scripts = [ "expression_simonspecialairn", "expression_richterspecialairn" ], category = ACMD_EXPRESSION, low_priority )]
unsafe fn kirby_belmontspecialairn_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        kirby_belmontspecialn_exp, kirby_belmontspecialairn_exp
    );
}