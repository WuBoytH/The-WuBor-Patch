use crate::imports::*;

unsafe extern "C" fn game_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(
                object as *mut smash::app::Weapon
            );
        }
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
}

unsafe extern "C" fn effect_attackairlw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(
                object as *mut smash::app::Weapon,
                true
            );
        }
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_straight"), Hash40::new("hookshot3"), -6, 0, 0, 180, 50, 90, 1.1, true);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("simon_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, true, 0.65);
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("simon_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_chain_2_visibility(
                object as *mut smash::app::Weapon,
                false
            );
        }
    }
}

unsafe extern "C" fn game_landingairlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_MOVE);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::reset_node_fix_flag_list(
                object as *mut smash::app::Weapon
            );
        }
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attackairlw", game_attackairlw, Priority::Low);
    agent.acmd("effect_attackairlw", effect_attackairlw, Priority::Low);

    agent.acmd("game_landingairlw", game_landingairlw, Priority::Low);
}