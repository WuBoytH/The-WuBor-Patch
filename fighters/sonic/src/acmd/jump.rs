use super::*;

unsafe extern "C" fn effect_jumpmini(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_jump_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, false);
    }
}

unsafe extern "C" fn sound_jumpmini(agent: &mut L2CAgentBase) {
    // frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_sonic_jump_mini"));
    }
}

unsafe extern "C" fn expression_jumpmini(agent: &mut L2CAgentBase) {
    let animcmd: &mut L2CFighterAnimcmdExpressionCommon = std::mem::transmute(agent);
    L2CFighterAnimcmdExpressionCommon::expression_JumpFrontMiniRumble(animcmd);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("effect_jumpmini", effect_jumpmini, Priority::Low);
    agent.acmd("sound_jumpmini", sound_jumpmini, Priority::Low);
    agent.acmd("expression_jumpmini", expression_jumpmini, Priority::Low);

    agent.acmd("sound_jumpfrontmini", sound_jumpmini, Priority::Low);
    agent.acmd("expression_jumpfrontmini", expression_jumpmini, Priority::Low);

    agent.acmd("sound_jumpbackmini", sound_jumpmini, Priority::Low);
    agent.acmd("expression_jumpbackmini", expression_jumpmini, Priority::Low);
}