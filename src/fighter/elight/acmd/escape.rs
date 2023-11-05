use crate::imports::acmd_imports::*;

#[acmd_script( agent = "elight", script = "game_escapen", category = ACMD_GAME, low_priority )]
unsafe fn elight_escapen(_agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "elight", script = "game_escapef", category = ACMD_GAME, low_priority )]
unsafe fn elight_escapef(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
}

#[acmd_script( agent = "elight", script = "game_escapeb", category = ACMD_GAME, low_priority )]
unsafe fn elight_escapeb(_agent: &mut L2CAgentBase) {
}

#[acmd_script( agent = "elight", script = "game_escapeairslide", category = ACMD_GAME, low_priority )]
unsafe fn elight_escapeairslide(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "elight", script = "game_escapeairslideforesight", category = ACMD_GAME, low_priority )]
unsafe fn elight_escapeairslideforesight(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_GRAVITY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_escapen", elight_escapen);

    agent.game_acmd("game_escapef", elight_escapef);

    agent.game_acmd("game_escapeb", elight_escapeb);

    agent.game_acmd("game_escapeairslide", elight_escapeairslide);

    agent.game_acmd("game_escapeairslideforesight", elight_escapeairslideforesight);
}