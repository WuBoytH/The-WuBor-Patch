use crate::imports::acmd_imports::*;

// Temporary, for debug purposes
#[acmd_script( agent = "lucario", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME, low_priority )]
unsafe fn lucario_appealhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        if macros::is_excute(fighter) {
            VarModule::inc_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL);
        }
    }
    else {
        if macros::is_excute(fighter) {
            VarModule::set_int(fighter.battle_object, lucario::instance::int::AURA_LEVEL, 0);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucario_appealhi
    );
}