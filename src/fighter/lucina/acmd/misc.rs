use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::{Hash40, Vector3f},
        app::{lua_bind::*, sv_animcmd::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::super::helper::*,
    wubor_utils::{
        wua_bind::*,
        vars::*
    }
};

// #[acmd_script( agent = "lucina", script = "effect_run", category = ACMD_EFFECT, low_priority )]
// unsafe fn lucina_runeff(fighter: &mut L2CAgentBase) {
//     while {
//         frame(fighter.lua_state_agent, 4.0);
//         if macros::is_excute(fighter) {
//             macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//         }
//         frame(fighter.lua_state_agent, 21.0);
//         if macros::is_excute(fighter) {
//             macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//         }
//         frame(fighter.lua_state_agent, 37.0);
//         if macros::is_excute(fighter) {
//             macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
//         }
//         frame(fighter.lua_state_agent, 56.0);
//         fighter.clear_lua_stack();
//         wait_loop_sync_mot(fighter.lua_state_agent);
//         fighter.pop_lua_stack(1);
//         true
//     } {}
// }

pub fn install() {
    install_acmd_scripts!(
        lucina_runeff
    );
}