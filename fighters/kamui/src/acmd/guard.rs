use super::*;

unsafe extern "C" fn game_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, false, -1);
        ArticleModule::change_motion(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, Hash40::new("attack_end"), false, -1.0);
    }
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 8.0 / 1.0);
    if macros::is_excute(agent) {
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, 1.0 / 8.0);
    }
    frame(agent.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::set_rate(agent.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_DRAGONHAND, 1.0);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 361, 0, 0, 80, 4.5, 0.0, 7.5, 11.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 5.0, 361, 0, 0, 80, 5.5, 0.0, 7.5, 15.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 5.0, 361, 0, 0, 80, 5.5, 0.0, 7.5, 19.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BITE);
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
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 1, 8.5, 5, 2.5, -5, 0, 1.425, false, 0.75);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("kamui_bite"), Hash40::new("havel"), 0, 8, 0, 0, 0, 0, 1.6, true);
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_REQ_EFEECT_TRANSFORM_WIND);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_KAMUI_INSTANCE_WORK_ID_FLAG_OFF_EFEECT_TRANSFORM_WIND);
        macros::EFFECT_FLW_POS(agent, Hash40::new("kamui_transform_splash_end"), Hash40::new("neck"), 2, 0, 0, 0, 0, 0, 1, true);
    }
}

unsafe extern "C" fn sound_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_guard_cancel_attack"));
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_kamui_attack100_02"));
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_kamui_rnd_attack"));
    }
}

unsafe extern "C" fn expression_guardcancelattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("havel"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        VisibilityModule::set_int64(agent.module_accessor, hash40("dragon") as i64, hash40("dragon_horn") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("front_hair") as i64, hash40("front_hair_hide") as i64);
        VisibilityModule::set_int64(agent.module_accessor, hash40("hair") as i64, hash40("hair_hide") as i64);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attackm"), 0);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("hair") as i64, hash40("hair_normal") as i64);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("front_hair") as i64, hash40("front_hair_normal") as i64);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_int64(agent.module_accessor, hash40("dragon") as i64, hash40("dragon_none") as i64);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_guardcancelattack", game_guardcancelattack, Priority::Low);
    agent.acmd("effect_guardcancelattack", effect_guardcancelattack, Priority::Low);
    agent.acmd("sound_guardcancelattack", sound_guardcancelattack, Priority::Low);
    agent.acmd("expression_guardcancelattack", expression_guardcancelattack, Priority::Low);
}