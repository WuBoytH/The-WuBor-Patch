use crate::imports::acmd_imports::*;

#[acmd_script( agent = "zelda", scripts = [ "game_specialsstart", "game_specialairsstart" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn zelda_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_1);
    }
}

#[acmd_script( agent = "zelda", scripts = [ "game_specialsend", "game_specialairsend" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn zelda_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_2);
    }
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_specialsstart", zelda_specialsstart);

    agent.game_acmd("game_specialairsstart", zelda_specialsstart);

    agent.game_acmd("game_specialsend", zelda_specialsend);

    agent.game_acmd("game_specialairsend", zelda_specialsend);
}