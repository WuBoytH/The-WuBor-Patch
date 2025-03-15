use super::*;

unsafe extern "C" fn game_guardcancelattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("tail7"), 8.0, 361, 0, 0, 80, 5.5, 3.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 0, 0, 80, 7.5, 0.0, 7.0, 7.3, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        macros::ATTACK(agent, 2, 0, Hash40::new("tail8"), 8.0, 361, 0, 0, 80, 5.0, 6.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_TAIL);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0.0, 12.0, 6.0, 0.0, 0.0, 0.0, 1.4, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 8, 3, 0, 0, 0, 1.2, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.104, 0.024, 0.754);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -4, 0, 1.5, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("tail8"), 8, -0.0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_guard_cancel_attack"));
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ridley_attackhard_s01"));
    }
}

unsafe extern "C" fn expression_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("tail8"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_pierces"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_guardcancelattack", game_guardcancelattack, Priority::Low);
    agent.acmd("effect_guardcancelattack", effect_guardcancelattack, Priority::Low);
    agent.acmd("sound_guardcancelattack", sound_guardcancelattack, Priority::Low);
    agent.acmd("expression_guardcancelattack", expression_guardcancelattack, Priority::Low);
}