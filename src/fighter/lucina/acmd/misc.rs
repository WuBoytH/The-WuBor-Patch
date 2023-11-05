use crate::imports::acmd_imports::*;

#[acmd_script( agent = "lucina", script = "effect_run", category = ACMD_EFFECT, low_priority )]
unsafe extern "C" fn lucina_run_eff(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 4.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 21.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        frame(agent.lua_state_agent, 56.0);
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "lucina", script = "sound_run", category = ACMD_SOUND, low_priority )]
unsafe extern "C" fn lucina_run_snd(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(agent.lua_state_agent, 5.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_lucina_step_right_l"));
        }
        wait(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_lucina_step_left_l"));
        }
        wait(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_lucina_step_right_l"));
        }
        wait(agent.lua_state_agent, 17.0);
        if macros::is_excute(agent) {
            macros::PLAY_STEP(agent, Hash40::new("se_lucina_step_left_l"));
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

#[acmd_script( agent = "lucina", script = "expression_run", category = ACMD_EXPRESSION, low_priority )]
unsafe extern "C" fn lucina_run_exp(agent: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(agent) {
            slope!(agent, MA_MSC_CMD_SLOPE_SLOPE_INTP, SLOPE_STATUS_LR, 6);
        }
        frame(agent.lua_state_agent, 13.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 31.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 47.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(agent.lua_state_agent, 65.0);
        if macros::is_excute(agent) {
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        agent.clear_lua_stack();
        wait_loop_sync_mot(agent.lua_state_agent);
        agent.pop_lua_stack(1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucina_run_eff,
        lucina_run_snd,
        lucina_run_exp
    );
}