use super::*;

unsafe extern "C" fn game_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn effect_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_dash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_trail_dash_start"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_trail_dash_start"), 15);
    }
    wait(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_trail_step_left_l"));
    }
}

unsafe extern "C" fn expression_dash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 37.0);
    if ItemModule::is_have_item(agent.module_accessor, 0) {
        if macros::is_excute(agent) {
            lua_args!(agent, false, 0);
            FT_MOTION_INTP_WAIT_ITEM(agent.lua_state_agent);
        }
        frame(agent.lua_state_agent, 40.0);
        if macros::is_excute(agent) {
            lua_args!(agent, false, 0.1);
            FT_MOTION_INTP_WAIT_ITEM(agent.lua_state_agent);
        }
        frame(agent.lua_state_agent, 47.0);
        if macros::is_excute(agent) {
            lua_args!(agent, false, 0);
            FT_MOTION_INTP_WAIT_ITEM(agent.lua_state_agent);
        }
    }
}

unsafe extern "C" fn game_turndash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_DASH_FLAG_TURN_DASH);
    }
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        WorkModule::enable_transition_term(agent.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn effect_turndash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_turndash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_trail_dash_start"));
        macros::SET_PLAY_INHIVIT(agent, Hash40::new("se_trail_dash_start"), 15);
    }
    wait(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::PLAY_STEP(agent, Hash40::new("se_trail_step_left_l"));
    }
}

unsafe extern "C" fn expression_turndash(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 2);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 32.0);
    if ItemModule::is_have_item(agent.module_accessor, 0) {
        if macros::is_excute(agent) {
            lua_args!(agent, false, 0.001);
            FT_MOTION_INTP_WAIT_ITEM(agent.lua_state_agent);
        }
        frame(agent.lua_state_agent, 37.0);
        if macros::is_excute(agent) {
            lua_args!(agent, false, 0.05);
            FT_MOTION_INTP_WAIT_ITEM(agent.lua_state_agent);
        }
        frame(agent.lua_state_agent, 41.0);
        if macros::is_excute(agent) {
            lua_args!(agent, false, 0.11);
            FT_MOTION_INTP_WAIT_ITEM(agent.lua_state_agent);
        }
        frame(agent.lua_state_agent, 46.0);
        if macros::is_excute(agent) {
            lua_args!(agent, false, 0);
            FT_MOTION_INTP_WAIT_ITEM(agent.lua_state_agent);
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