use crate::imports::acmd_imports::*;

#[acmd_script( agent = "falco", scripts = [ "game_appeallwl", "game_appeallwr" ], category = ACMD_GAME, low_priority)]
unsafe fn falco_appeallw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, falco::instance::flag::KAA);
    }
}

pub fn install() {
    install_acmd_scripts!(
        falco_appeallw
    );
}