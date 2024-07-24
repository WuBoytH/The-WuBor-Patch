use super::*;

unsafe extern "C" fn game_move(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 1.25, 25, 100, 60, 0, 8.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 1.25, 366, 100, 60, 0, 16.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        // macros::ATTACK(agent, 2, 0, Hash40::new("top"), 0.0, 366, 100, 65, 0, 35.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_move(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_ryu_final_shinkuhado2"), false, false, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_final_shinkuhado_bullet"), Hash40::new("top"), 0, 0, 1, 0, 0, 0, 1.2, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("ryu_final_shinkuhado_bullet2"), Hash40::new("top"), 0, 0, 2, 0, 0, 0, 1.5, true);
    }
}

unsafe extern "C" fn game_finish(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 1, Hash40::new("top"), 10.0, 50, 60, 0, 90, 18.0, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn effect_finish(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("ryu_final_shinkuhado_finish"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_move", game_move, Priority::Low);
    agent.acmd("effect_move", effect_move, Priority::Low);

    agent.acmd("game_finish", game_finish, Priority::Low);
    agent.acmd("effect_finish", effect_finish, Priority::Low);
}