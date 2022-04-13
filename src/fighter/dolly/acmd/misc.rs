use {
    smash::{
        lua2cpp::L2CAgentBase,
        app::{lua_bind::*, sv_animcmd::*}
    },
    smash_script::*,
    smashline::*,
    super::super::vars::*,
};

#[acmd_script( agent = "dolly", scripts = [ "game_appealhir", "game_appealhil" ], category = ACMD_GAME, low_priority )]
unsafe fn dolly_appealhi(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 30.0);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL) {
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "dolly", scripts = [ "game_appeallwr", "game_appeallwl" ], category = ACMD_GAME, low_priority )]
unsafe fn dolly_appeallw(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 30.0);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL) {
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

#[acmd_script( agent = "dolly", scripts = [ "game_appealsr", "game_appealsl" ], category = ACMD_GAME, low_priority )]
unsafe fn dolly_appeals(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL) {
        macros::FT_MOTION_RATE(fighter, 0.8);
    }
    frame(fighter.lua_state_agent, 30.0);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_IS_SPECIAL_CANCEL) {
        if macros::is_excute(fighter) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        dolly_appealhi,
        dolly_appeallw,
        dolly_appeals
    );
}