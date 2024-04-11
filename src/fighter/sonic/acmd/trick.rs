use crate::imports::*;

unsafe extern "C" fn game_trickhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_MOVEMENT);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 14.0, 15.0);
}

unsafe extern "C" fn game_trickf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_MOVEMENT);
    }
    macros::FT_MOTION_RATE(agent, 1.5);
}

unsafe extern "C" fn game_trickb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, sonic::status::flag::TRICK_ENABLE_MOVEMENT);
    }
    MiscModule::calc_motion_rate_from_cancel_frame(agent, 6.0, 10.0);
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_trickhi", game_trickhi);

    agent.acmd("game_trickf", game_trickf);

    agent.acmd("game_trickb", game_trickb);
}