use crate::imports::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.1);
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 17.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) != *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("mouth2"), 4.0, -0.9, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 1, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("throw"), 4.0, -0.9, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 1, Hash40::new("throw"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 1.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specialsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 10.0, 80, 50, 0, 43, 5.5, 0.0, 5.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 32, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        JostleModule::set_status(agent.module_accessor, false);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialn", game_specialn, Priority::Low);

    agent.acmd("game_specialairn", game_specialn, Priority::Low);

    agent.acmd("game_specialsloop", game_specialsloop, Priority::Low);
}