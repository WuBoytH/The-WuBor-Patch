use crate::imports::acmd_imports::*;

#[acmd("falco", [ "game_appeallwl", "game_appeallwr" ])]
unsafe fn falco_appeallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, falco::instance::flag::KAA);
    }
}

pub fn install() {
    falco_appeallw::install();
}