use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "robot", scripts = [ "game_appealhil", "game_appealhir" ], category = ACMD_GAME, low_priority )]
unsafe fn robot_appealhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 38.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        if macros::is_excute(fighter) {
            WorkModule::set_int(fighter.module_accessor, 60, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("appeal_hi_loop"),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
        }
    }
}

#[acmd_script( agent = "robot", script = "effect_appealhiloop", category = ACMD_EFFECT, low_priority )]
unsafe fn robot_appealhiloop_eff(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        frame(fighter.lua_state_agent, 23.0);
        println!("hi");
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
            if macros::is_excute(fighter) {
                let lr = PostureModule::lr(fighter.module_accessor);
                let mot = if lr < 0.0 {
                    WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_L)
                }
                else {
                    WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_APPEAL_WORK_INT_MOTION_KIND_R)
                };
                let restart_frame = WorkModule::get_int(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME) as f32;
                MotionModule::change_motion_force_inherit_frame(
                    fighter.module_accessor,
                    Hash40::new_raw(mot),
                    restart_frame,
                    1.0,
                    0.0
                );
            }
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

pub fn install() {
    install_acmd_scripts!(
        robot_appealhi,
        robot_appealhiloop_eff
    );
}