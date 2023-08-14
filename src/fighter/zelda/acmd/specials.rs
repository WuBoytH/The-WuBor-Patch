use crate::imports::acmd_imports::*;

#[acmd("zelda", [ "game_specialsstart", "game_specialairsstart" ])]
unsafe fn zelda_specialsstart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_1);
    }
}

#[acmd("zelda", [ "game_specialsend", "game_specialairsend" ])]
unsafe fn zelda_specialsend(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_ZELDA_STATUS_SPECIAL_S_FLAG_2);
    }
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install() {
    zelda_specialsstart::install();
    zelda_specialsend::install();
}