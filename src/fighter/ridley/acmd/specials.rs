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

#[acmd_script( agent = "ridley", scripts = [ "game_specialsstart", "game_specialairsstart" ], category = ACMD_GAME, low_priority )]
unsafe fn ridley_specialsstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 8.0, 6.0, 7.5, 7.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_JUMP);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 7.0, 6.0, 7.5, 7.5);
    }
    frame(fighter.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 6.8, 0.0, 7.2, 16.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.3, 0.0, 7.2, 16.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 2, Hash40::new("top"), 5.0, 0.0, 7.2, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(fighter.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 37.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ridley_specialsstart
    );
}