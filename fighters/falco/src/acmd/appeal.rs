use super::*;

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::falco::instance::flag::KAA);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
}