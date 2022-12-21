use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::sv_animcmd::*,
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "marth", script = "game_speciallwdashf", category = ACMD_GAME )]
unsafe fn marth_speciallwdashf(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(fighter) {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
            VarModule::off_flag(fighter.battle_object, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwdashf", category = ACMD_EFFECT )]
unsafe fn marth_speciallwdashf_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, -4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwdashf", category = ACMD_SOUND )]
unsafe fn marth_speciallwdashf_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_escape"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwdashf", category = ACMD_EXPRESSION )]
unsafe fn marth_speciallwdashf_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
}

#[acmd_script( agent = "marth", script = "game_speciallwdashb", category = ACMD_GAME )]
unsafe fn marth_speciallwdashb(fighter: &mut L2CAgentBase) {
    if VarModule::is_flag(fighter.battle_object, marth::instance::flag::PARRY_XLU) {
        if macros::is_excute(fighter) {
            macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
            VarModule::off_flag(fighter.battle_object, marth::instance::flag::PARRY_XLU);
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwdashb", category = ACMD_EFFECT )]
unsafe fn marth_speciallwdashb_eff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 4, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 3, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwdashb", category = ACMD_SOUND )]
unsafe fn marth_speciallwdashb_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_escape"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwdashb", category = ACMD_EXPRESSION )]
unsafe fn marth_speciallwdashb_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
}

pub fn install() {
    install_acmd_scripts!(
        marth_speciallwdashf, marth_speciallwdashf_eff, marth_speciallwdashf_snd, marth_speciallwdashf_exp,
        marth_speciallwdashb, marth_speciallwdashb_eff, marth_speciallwdashb_snd, marth_speciallwdashb_exp
    );
}