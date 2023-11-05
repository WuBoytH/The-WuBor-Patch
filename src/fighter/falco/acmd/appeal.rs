use crate::imports::acmd_imports::*;

#[acmd_script( agent = "falco", scripts = [ "game_appeallwl", "game_appeallwr" ], category = ACMD_GAME, low_priority)]
unsafe extern "C" fn falco_appeallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, falco::instance::flag::KAA);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_appeallwl", falco_appeallw);
    agent.game_acmd("game_appeallwr", falco_appeallw);
}