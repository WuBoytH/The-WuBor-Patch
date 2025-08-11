use super::*;

unsafe extern "C" fn game_throwhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 5.0 / 9.0);
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 30.0 / 25.0);
    frame(agent.lua_state_agent, 35.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 36.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_MOVE);
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(object as *mut smash::app::Weapon);
        WeaponSpecializer_SimonWhip::set_node_fix_flag_list(
            object as *mut smash::app::Weapon,
            3,
            4,
            8,
            9,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1,
            -1
        );
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_throwhi", game_throwhi, Priority::Low);
}