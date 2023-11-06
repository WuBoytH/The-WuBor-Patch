use crate::imports::acmd_imports::*;

#[acmd_script( agent = "richter_whip", scripts = [ "game_attackairf", "game_attackairfhi", "game_attackairflw" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn richter_whip_attackairf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.9);
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
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
}

#[acmd_script( agent = "richter_whip", scripts = [ "game_attackairb", "game_attackairbhi", "game_attackairblw" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn richter_whip_attackairb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.9);
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
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn richter_whip_attackairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.7);
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
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_attackairf", richter_whip_attackairf);

    agent.game_acmd("game_attackairfhi", richter_whip_attackairf);

    agent.game_acmd("game_attackairflw", richter_whip_attackairf);

    agent.game_acmd("game_attackairb", richter_whip_attackairb);

    agent.game_acmd("game_attackairbhi", richter_whip_attackairb);

    agent.game_acmd("game_attackairblw", richter_whip_attackairb);

    agent.game_acmd("game_attackairhi", richter_whip_attackairhi);
}