use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::super::vars::*
};

#[acmd_script( agent = "marth", script = "game_speciallwspecials", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwspecials(fighter: &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 7.0);
    // if macros::is_excute(fighter) {
        
    // }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_DASH);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_END);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecials", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecials_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, 0, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 30.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 3.0);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecials", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecials_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_marth_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_escape"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_dash_stop"));
        macros::SET_PLAY_INHIVIT(fighter, Hash40::new("se_marth_dash_stop"), 20);
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecials", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecials_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
}

#[acmd_script( agent = "marth", script = "game_speciallwspecialairs", category = ACMD_GAME, low_priority )]
unsafe fn marth_speciallwspecialairs(fighter: &mut L2CAgentBase) {
    // frame(fighter.lua_state_agent, 7.0);
    // if macros::is_excute(fighter) {
        
    // }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_DASH);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLAG_END);
    }
}

#[acmd_script( agent = "marth", script = "effect_speciallwspecialairs", category = ACMD_EFFECT, low_priority )]
unsafe fn marth_speciallwspecialairs_eff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        let angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        let angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        let angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        let angle = WorkModule::get_float(fighter.module_accessor, FIGHTER_MARTH_STATUS_STANCE_SPECIAL_S_FLOAT_ANGLE);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 9, -8, angle, 180, 0, 1.5, true);
        macros::LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
    }
}

#[acmd_script( agent = "marth", script = "sound_speciallwspecialairs", category = ACMD_SOUND, low_priority )]
unsafe fn marth_speciallwspecialairs_snd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_marth_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_marth_escape"));
    }
}

#[acmd_script( agent = "marth", script = "expression_speciallwspecialairs", category = ACMD_EXPRESSION, low_priority )]
unsafe fn marth_speciallwspecialairs_exp(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(
            fighter.module_accessor,
            Hash40::new("rbkind_dash"),
            0,
            false,
            *BATTLE_OBJECT_ID_INVALID as u32
        )
    }
}

pub fn install() {
    install_acmd_scripts!(
        marth_speciallwspecials, marth_speciallwspecials_eff, marth_speciallwspecials_snd, marth_speciallwspecials_exp,
        marth_speciallwspecialairs, marth_speciallwspecialairs_eff, marth_speciallwspecialairs_snd, marth_speciallwspecialairs_exp
    );
}