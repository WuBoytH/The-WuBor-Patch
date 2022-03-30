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

#[acmd_script( agent = "demon", script = "game_appealhil", category = ACMD_GAME, low_priority )]
unsafe fn demon_appealhil(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        if macros::is_excute(fighter) {
            WorkModule::set_int(
                fighter.module_accessor,
                *CONTROL_PAD_BUTTON_APPEAL_HI,
                FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_HELD_BUTTON
            );
            WorkModule::set_int(fighter.module_accessor, 40, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("appeal_hi_l_loop"),
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

#[acmd_script( agent = "demon", script = "game_appealhir", category = ACMD_GAME, low_priority )]
unsafe fn demon_appealhir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 46.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) {
        if macros::is_excute(fighter) {
            WorkModule::set_int(
                fighter.module_accessor,
                *CONTROL_PAD_BUTTON_APPEAL_HI,
                FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_HELD_BUTTON
            );
            WorkModule::set_int(fighter.module_accessor, 46, FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_RESTART_FRAME);
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD);
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new("appeal_hi_r_loop"),
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

pub fn install() {
    install_acmd_scripts!(
        demon_appealhil,
        demon_appealhir
    );
}