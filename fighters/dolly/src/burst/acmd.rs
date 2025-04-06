use super::*;

unsafe extern "C" fn game_superspecialtriple1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 42, 100, 25, 30, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(39.0), Some(9.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_superspecialtriple1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

unsafe extern "C" fn sound_superspecialtriple1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_final_superspecial01_03"));
    }
}

unsafe extern "C" fn game_superspecialtriple2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 6.0, 42, 100, 25, 30, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(39.0), Some(9.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_superspecialtriple2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 0.9, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.1);
    }
}

unsafe extern "C" fn sound_superspecialtriple2(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_final_superspecial01_03"));
    }
}

unsafe extern "C" fn game_superspecialtriple3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 70, 65, 0, 100, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(39.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0.0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
        AttackModule::set_no_dead_all(agent.module_accessor, true, false);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_superspecialtriple3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_ground"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_base"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW(agent, Hash40::new("dolly_volcano_flash"), Hash40::new("top"), 0, 0, -3, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_superspecialtriple3(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_dolly_final_superspecial01_03_last"));
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_superspecialtriple1", game_superspecialtriple1, Priority::Low);
    agent.acmd("effect_superspecialtriple1", effect_superspecialtriple1, Priority::Low);
    agent.acmd("sound_superspecialtriple1", sound_superspecialtriple1, Priority::Low);

    agent.acmd("game_superspecialtriple2", game_superspecialtriple2, Priority::Low);
    agent.acmd("effect_superspecialtriple2", effect_superspecialtriple2, Priority::Low);
    agent.acmd("sound_superspecialtriple2", sound_superspecialtriple2, Priority::Low);

    agent.acmd("game_superspecialtriple3", game_superspecialtriple3, Priority::Low);
    agent.acmd("effect_superspecialtriple3", effect_superspecialtriple3, Priority::Low);
    agent.acmd("sound_superspecialtriple3", sound_superspecialtriple3, Priority::Low);
}