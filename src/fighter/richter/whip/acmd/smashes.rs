use crate::imports::acmd_imports::*;

unsafe extern "C" fn richter_whip_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_NONE);
    }
    frame(agent.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(agent, 1.5);
    frame(agent.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_COLLIDE);
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 7.0 / 10.0);
    frame(agent.lua_state_agent, 60.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 76.0);
    if macros::is_excute(agent) {
        PhysicsModule::set_2nd_status(agent.module_accessor, *PH2NDARY_CRAW_MOVE);
        agent.clear_lua_stack();
        let object = sv_system::battle_object(agent.lua_state_agent) as *mut BattleObject;
        if !object.is_null() {
            WeaponSpecializer_SimonWhip::set_node_fix_flag_list(
                object as *mut smash::app::Weapon,
                3,
                4,
                5,
                9,
                10,
                11,
                12,
                13,
                14,
                15,
                16,
                17,
                18,
                19,
                20,
                21,
                22,
                23,
                24,
                25,
                26,
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
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_attacks4", richter_whip_attacks4);

    agent.game_acmd("game_attacks4hi", richter_whip_attacks4);

    agent.game_acmd("game_attacks4lw", richter_whip_attacks4);
}