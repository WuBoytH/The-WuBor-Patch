use crate::imports::*;

unsafe extern "C" fn game_speciallwdashf(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            VarModule::off_flag(agent.module_accessor, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_speciallwdashf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallwdashf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_escape"));
    }
}

unsafe extern "C" fn expression_speciallwdashf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
}

unsafe extern "C" fn game_speciallwdashb(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.module_accessor, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            VarModule::off_flag(agent.module_accessor, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

unsafe extern "C" fn effect_speciallwdashb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 3, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_speciallwdashb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_escape"));
    }
}

unsafe extern "C" fn expression_speciallwdashb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_speciallwdashf", game_speciallwdashf, Priority::Low);
    agent.acmd("effect_speciallwdashf", effect_speciallwdashf, Priority::Low);
    agent.acmd("sound_speciallwdashf", sound_speciallwdashf, Priority::Low);
    agent.acmd("expression_speciallwdashf", expression_speciallwdashf, Priority::Low);

    agent.acmd("game_speciallwdashb", game_speciallwdashb, Priority::Low);
    agent.acmd("effect_speciallwdashb", effect_speciallwdashb, Priority::Low);
    agent.acmd("sound_speciallwdashb", sound_speciallwdashb, Priority::Low);
    agent.acmd("expression_speciallwdashb", expression_speciallwdashb, Priority::Low);
}