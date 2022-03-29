use {
    smash::{
        lua2cpp::L2CAgentBase,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    wubor_utils::vars::*
};

#[acmd_script( agent = "edge", scripts = [ "game_appealsl", "game_appealsr" ], category = ACMD_GAME, low_priority )]
unsafe fn edge_appeals(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 44.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L | *CONTROL_PAD_BUTTON_APPEAL_S_R) {
        if macros::is_excute(fighter) {
            WorkModule::set_int(
                fighter.module_accessor,
                *CONTROL_PAD_BUTTON_APPEAL_S_L | *CONTROL_PAD_BUTTON_APPEAL_S_R,
                FIGHTER_STATUS_APPEAL_WORK_INT_APPEAL_HELD_BUTTON
            );
            WorkModule::on_flag(fighter.module_accessor, FIGHTER_STATUS_APPEAL_WORK_FLAG_APPEAL_HOLD);
            MotionModule::set_rate(fighter.module_accessor, 0.0);
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        edge_appeals
    );
}