use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    custom_var::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "sonic", script = "game_throwf", category = ACMD_GAME, low_priority )]
unsafe fn sonic_throwf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 10, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        CatchModule::catch_cut(fighter.module_accessor, false, false);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 25, 75, 70, 60, 4.0, 0.0, 6.0, 6.0, Some(0.0), Some(6.0), Some(18.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, -1.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("colliision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, commons::status::flag::DASH_CANCEL);
    }
}

#[acmd_script( agent = "sonic", script = "effect_throwf", category = ACMD_EFFECT, low_priority )]
unsafe fn sonic_throwf_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7.8, -2, 0, 0, 0, 0.9, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 20, 7.5, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 360, false);
    }
}

#[acmd_script( agent = "sonic", script = "sound_throwf", category = ACMD_SOUND, low_priority )]
unsafe fn sonic_throwf_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

#[acmd_script( agent = "sonic", script = "expression_throwf", category = ACMD_EXPRESSION, low_priority )]
unsafe fn sonic_throwf_exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, CAMERA_QUAKE_KIND_NONE);
        FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
        slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE, SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}

pub fn install() {
    install_acmd_scripts!(
        sonic_throwf, sonic_throwf_eff, sonic_throwf_snd, sonic_throwf_exp
    );
}