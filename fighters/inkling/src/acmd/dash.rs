use super::*;
use crate::helper::*;

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
    }
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
        }
    }
    inkling_generate_squid_helper(agent);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0{
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("dash_l"), false, -1.0);
        }
    }
    else {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("dash_r"), false, -1.0);
        }
    }
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_dash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("inkling_squid_change"), Hash40::new("top"), 0, 3, 0, 0, 0, 0, 1, true);
    }
    let r = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
    let g = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
    let b = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
    if macros::is_excute(agent) {
        macros::LAST_PARTICLE_SET_COLOR(agent, r, g, b);
    }
    if macros::is_excute(agent) {
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_inkling_dash_start"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_inkling_dash_start"), 20);
    }
}

unsafe extern "C" fn expression_dash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        InkPaintModule::set_special_paint(agent.module_accessor, SpecialPaintKind{_address: *SPECIAL_PAINT_SQUID_TO_HUMAN as u8});
    }
    frame(agent.lua_state_agent, 15.0);
    if macros::is_excute(agent) {
        InkPaintModule::set_special_paint(agent.module_accessor, SpecialPaintKind{_address: *SPECIAL_PAINT_SQUID_TO_HUMAN as u8});
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
    frame(agent.lua_state_agent, 33.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_BOY) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            FT_MOTION_INTP_WAIT(agent.lua_state_agent);
        }
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            WorkModule::on_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
        }
    }
    inkling_generate_squid_helper(agent);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("turn_dash_l"), false, -1.0);
        }
    }
    else {
        if macros::is_excute(agent) {
            ArticleModule::change_motion(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("turn_dash_r"), false, -1.0);
        }
    }
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, false);
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_whole(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        ArticleModule::set_visibility_whole(agent.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

unsafe extern "C" fn effect_turndash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_UNSYNC_VIS(agent, Hash40::new("inkling_squid_change"), Hash40::new("head"), 3, 0, 0, 0, 0, 0, 1, true);
    }
    let r = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R);
    let g = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G);
    let b = WorkModule::get_float(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B);
    if macros::is_excute(agent) {
        macros::LAST_PARTICLE_SET_COLOR(agent, r, g, b);
    }
    if macros::is_excute(agent) {
        EffectModule::enable_sync_init_pos_last(agent.module_accessor);
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_turndash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_inkling_dash_start"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_inkling_dash_start"), 20);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_inkling_dash_turn"));
    }
}

unsafe extern "C" fn expression_turndash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4, true);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        InkPaintModule::set_special_paint(agent.module_accessor, SpecialPaintKind{_address: *SPECIAL_PAINT_SQUID_TO_HUMAN as u8});
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        InkPaintModule::set_special_paint(agent.module_accessor, SpecialPaintKind{_address: *SPECIAL_PAINT_SQUID_TO_HUMAN as u8});
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(agent.lua_state_agent, 35.0);
    if WorkModule::is_flag(agent.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_BOY) {
        if macros::is_excute(agent) {
            agent.clear_lua_stack();
            FT_MOTION_INTP_WAIT(agent.lua_state_agent);
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_dash", game_dash, Priority::Low);
    agent.acmd("effect_dash", effect_dash, Priority::Low);
    agent.acmd("sound_dash", sound_dash, Priority::Low);
    agent.acmd("expression_dash", expression_dash, Priority::Low);

    agent.acmd("game_turndash", game_turndash, Priority::Low);
    agent.acmd("effect_turndash", effect_turndash, Priority::Low);
    agent.acmd("sound_turndash", sound_turndash, Priority::Low);
    agent.acmd("expression_turndash", expression_turndash, Priority::Low);
}