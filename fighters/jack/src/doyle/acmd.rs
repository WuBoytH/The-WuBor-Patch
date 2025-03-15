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

unsafe extern "C" fn game_guardcancelattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 6.0 / 1.0);
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("jack_doyle_magic_flash"), Hash40::new("handl"), 2, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("jack_doyle_crow_line"), Hash40::new("jack_doyle_crow_line"), Hash40::new("top"), 1, 21, 3, 26, -20, -4, 1, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_appear", effect_appear, Priority::Low);

    agent.acmd("effect_return", effect_return, Priority::Low);

    agent.acmd("game_guardcancelattack", game_guardcancelattack, Priority::Low);
    agent.acmd("effect_guardcancelattack", effect_guardcancelattack, Priority::Low);
}