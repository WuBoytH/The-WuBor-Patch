use super::*;

unsafe extern "C" fn effect_appear(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("jack_doyle_appear"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::BURN_COLOR(agent, 0.02, 0.15, 2, 0);
        macros::BURN_COLOR_FRAME(agent, 1, 0.02, 0.15, 2, 0.7);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.35);
        macros::LAST_EFFECT_SET_RATE(agent, 1.5);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR(agent, 0.02, 0.15, 2, 0.7);
        macros::BURN_COLOR_FRAME(agent, 12, 0.02, 0.15, 2, 0);
    }
    frame(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
    }
}

unsafe extern "C" fn effect_return(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("jack_doyle_disappear"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_ALPHA(agent, 0.35);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR(agent, 0.02, 0.15, 2, 0);
        macros::BURN_COLOR_FRAME(agent, 40, 0.02, 0.15, 2, 0.7);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::EFFECT_DETACH_KIND(agent, Hash40::new("jack_doyle_disappear"), -1);
    }
    frame(agent.lua_state_agent, 70.0);
    if macros::is_excute(agent) {
        macros::BURN_COLOR_NORMAL(agent);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_appear", effect_appear, Priority::Low);

    agent.acmd("effect_return", effect_return, Priority::Low);
}