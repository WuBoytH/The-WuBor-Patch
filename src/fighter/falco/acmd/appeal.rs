use crate::imports::acmd_imports::*;

unsafe extern "C" fn falco_appeallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, falco::instance::flag::KAA);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.game_acmd("game_appeallwl", falco_appeallw);
    agent.game_acmd("game_appeallwr", falco_appeallw);
}