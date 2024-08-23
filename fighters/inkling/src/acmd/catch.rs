use super::*;
use crate::helper::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if macros::is_excute(agent) {
            ArticleModule::generate_article(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    inkling_generate_squid_helper(agent);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("catch_dash_l"), false, -1.0);
        }
    }
    else {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("catch_dash"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 2.6, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    inkling_generate_squid_helper(agent);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("catch_turn_l"), false, -1.0);
        }
    }
    else {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("catch_turn"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.3, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
}

unsafe extern "C" fn game_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 100, 30, 0, 5.8, 0.0, 5.2, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 6.5, 0, 2, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 7, 2, 45, 0, 0, 0.8, true);
        macros::LAST_PARTICLE_SET_COLOR(agent, 0.67, 1.11, 1.2);
    }
}

unsafe extern "C" fn expression_catchattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_catch", game_catch, Priority::Low);

    agent.acmd("game_catchdash", game_catchdash, Priority::Low);

    agent.acmd("game_catchturn", game_catchturn, Priority::Low);

    agent.acmd("game_catchattack", game_catchattack, Priority::Low);
    agent.acmd("effect_catchattack", effect_catchattack, Priority::Low);
    agent.acmd("expression_catchattack", expression_catchattack, Priority::Low);
}