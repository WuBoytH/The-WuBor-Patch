use super::*;

unsafe extern "C" fn game_guardcancelattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 7.0 / 1.0);
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.0, 361, 0, 0, 80, 5.5, 0.0, 6.5, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 8.0, 361, 0, 0, 80, 5.5, 0.0, 6.5, 16.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 8.0, 361, 0, 0, 80, 5.5, 0.0, 6.5, 20.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    wait(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("sys_flash"), Hash40::new("top"), 0.0, 12.0, 6.0, 0.0, 0.0, 0.0, 1.4, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("pfushigisou_vine"), Hash40::new("vinel5"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("pfushigisou_vine"), Hash40::new("vinel4"), 0, 0, 0, 0, 0, 0, 1, true, 0.5);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pfushigisou_vine_a1"), Hash40::new("tex_pfushigisou_vine_a2"), 5, Hash40::new("vinel5"), -1, 0, 0, Hash40::new("vinel5"), 5.5, 0, 0, true, Hash40::new("null"), Hash40::new("vinel5"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pfushigisou_vine_b1"), Hash40::new("tex_pfushigisou_vine_b2"), 5, Hash40::new("vinel4"), 0, 0, 0, Hash40::new("vinel4"), 6, 0, 0, true, Hash40::new("null"), Hash40::new("vinel4"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pfushigisou_vine_b1"), Hash40::new("tex_pfushigisou_vine_b2"), 5, Hash40::new("vinel3"), 2, 0, 0, Hash40::new("vinel3"), 6, 0, 0, true, Hash40::new("null"), Hash40::new("vinel3"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::EFFECT_FOLLOW(agent, Hash40::new("pfushigisou_vine"), Hash40::new("viner5"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("pfushigisou_vine"), Hash40::new("viner4"), 0, 0, 0, 0, 0, 0, 1, true, 0.5);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pfushigisou_vine_a1"), Hash40::new("tex_pfushigisou_vine_a2"), 5, Hash40::new("viner5"), -1, 0, 0, Hash40::new("viner5"), 5.5, 0, 0, true, Hash40::new("null"), Hash40::new("viner5"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pfushigisou_vine_b1"), Hash40::new("tex_pfushigisou_vine_b2"), 5, Hash40::new("viner4"), 0, 0, 0, Hash40::new("viner4"), 6, 0, 0, true, Hash40::new("null"), Hash40::new("viner4"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_pfushigisou_vine_b1"), Hash40::new("tex_pfushigisou_vine_b2"), 5, Hash40::new("viner3"), 2, 0, 0, Hash40::new("viner3"), 6, 0, 0, true, Hash40::new("null"), Hash40::new("viner3"), 0, 0, 0, 0, 0, 0, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 3.5, 19, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 360, true, 1);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 2, 0, -2.2, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("pfushigisou_vine"), false, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 6);
    }
}

unsafe extern "C" fn sound_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_guard_cancel_attack"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_pfushigisou_attack100"));
        macros::PLAY_SE(agent, Hash40::new("se_pfushigisou_attack100end"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_pfushigisou_rnd_attack"));
    }
}

unsafe extern "C" fn expression_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(agent.module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_guardcancelattack", game_guardcancelattack, Priority::Low);
    agent.acmd("effect_guardcancelattack", effect_guardcancelattack, Priority::Low);
    agent.acmd("sound_guardcancelattack", sound_guardcancelattack, Priority::Low);
    agent.acmd("expression_guardcancelattack", expression_guardcancelattack, Priority::Low);
}