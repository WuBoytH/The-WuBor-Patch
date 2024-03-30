use crate::imports::*;

unsafe extern "C" fn palutena_specialhistart(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        JostleModule::set_status(agent.module_accessor, false);
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
}

unsafe extern "C" fn palutena_specialhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn palutena_specialairhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_DIVE);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_PALUTENA_STATUS_SPECIAL_HI_CONTROL_ON);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialhistart", palutena_specialhistart);

    agent.acmd("game_specialairhistart", palutena_specialhistart);

    agent.acmd("game_specialhi", palutena_specialhi);

    agent.acmd("game_specialairhi", palutena_specialairhi);
}