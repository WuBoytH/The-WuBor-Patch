use std::arch::asm;
use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "lucina", script = "effect_run", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_run_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 37.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 56.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "lucina", script = "sound_run", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_run_snd(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STEP(fighter, Hash40::new("se_lucina_step_right_l"));
        }
        wait(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STEP(fighter, Hash40::new("se_lucina_step_left_l"));
        }
        wait(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STEP(fighter, Hash40::new("se_lucina_step_right_l"));
        }
        wait(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STEP(fighter, Hash40::new("se_lucina_step_left_l"));
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "lucina", script = "expression_run", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_run_exp(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            slope!(fighter, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 6);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 31.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 47.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 65.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucina_run_eff,
        lucina_run_snd,
        lucina_run_exp
    );
}