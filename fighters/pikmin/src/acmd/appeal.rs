use super::*;

unsafe extern "C" fn game_appeal(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::pikmin::status::flag::APPEAL_CALL_PICKIE);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::pikmin::status::flag::APPEAL_CALL_PICKIE);
    }
    wait(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::pikmin::status::flag::APPEAL_CALL_PICKIE);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_appealsr", game_appeal, Priority::Default);

    agent.acmd("game_appealsl", game_appeal, Priority::Default);

    agent.acmd("game_appealhir", game_appeal, Priority::Default);

    agent.acmd("game_appealhil", game_appeal, Priority::Default);

    agent.acmd("game_appeallwr", game_appeal, Priority::Default);

    agent.acmd("game_appeallwl", game_appeal, Priority::Default);
}