use crate::imports::acmd_imports::*;

#[acmd("marth", "game_speciallwdashf")]
unsafe fn marth_speciallwdashf(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.battle_object, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            VarModule::off_flag(agent.battle_object, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

#[acmd("marth", "effect_speciallwdashf")]
unsafe fn marth_speciallwdashf_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd("marth", "sound_speciallwdashf")]
unsafe fn marth_speciallwdashf_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_escape"));
    }
}

#[acmd("marth", "expression_speciallwdashf")]
unsafe fn marth_speciallwdashf_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
}

#[acmd("marth", "game_speciallwdashb")]
unsafe fn marth_speciallwdashb(agent: &mut L2CAgentBase) {
    if VarModule::is_flag(agent.battle_object, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(agent) {
            macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
            VarModule::off_flag(agent.battle_object, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_NORMAL);
    }
}

#[acmd("marth", "effect_speciallwdashb")]
unsafe fn marth_speciallwdashb_eff(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 3, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd("marth", "sound_speciallwdashb")]
unsafe fn marth_speciallwdashb_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_marth_escape"));
    }
}

#[acmd("marth", "expression_speciallwdashb")]
unsafe fn marth_speciallwdashb_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
}

pub fn install() {
    marth_speciallwdashf::install();
    marth_speciallwdashf_eff::install();
    marth_speciallwdashf_snd::install();
    marth_speciallwdashf_exp::install();
    marth_speciallwdashb::install();
    marth_speciallwdashb_eff::install();
    marth_speciallwdashb_snd::install();
    marth_speciallwdashb_exp::install();
}