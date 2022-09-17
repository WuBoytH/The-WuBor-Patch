use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

#[acmd_script( agent = "demon", script = "game_catchattack", category = ACMD_GAME, low_priority )]
unsafe fn demon_pummel(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.7, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 2.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_CATCHATTACK, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "demon", script = "game_catchcommand", category = ACMD_GAME, low_priority )]
unsafe fn demon_catchcommand(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    macros::FT_MOTION_RATE(fighter, 1.6);
    frame(fighter.lua_state_agent, 3.0);
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(fighter, 1.0);
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 6.6, 5.0, Some(0.0), Some(6.6), Some(11.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 2.0, 0.0, 6.6, 3.0, Some(0.0), Some(6.6), Some(13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(fighter);
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DEMON_STATUS_CATCH_COMMAND_FLAG_CHANGE_THROW);
    }
}

pub fn install() {
    install_acmd_scripts!(
        demon_pummel,
        demon_catchcommand
    );
}