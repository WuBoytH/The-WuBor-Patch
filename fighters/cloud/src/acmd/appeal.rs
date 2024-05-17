use super::*;

unsafe extern "C" fn game_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        VarModule::set_int64(agent.module_accessor, vars::appeal::int64::ACTION_MOT, hash40("appeal_s_loop"));
        let hold_button = VarModule::get_int(agent.module_accessor, vars::appeal::int::HOLD_BUTTON);
        VarModule::set_int(agent.module_accessor, vars::appeal::int::ACTION_BUTTON, hold_button);
        VarModule::on_flag(agent.module_accessor, vars::appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(agent.module_accessor, vars::appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, vars::appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(agent.module_accessor, vars::appeal::flag::ENABLE_ACTION_IMM);
    }
}

unsafe extern "C" fn game_appealsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::set_int(agent.module_accessor, vars::appeal::int::ACTION_FRAME, 12);
        VarModule::on_flag(agent.module_accessor, vars::appeal::flag::ACTION_BUTTON_CHECK);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.module_accessor, vars::appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, vars::appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.module_accessor, vars::appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(agent.module_accessor, vars::appeal::flag::ENABLE_ACTION_IMM);
    }
}

unsafe extern "C" fn sound_appealsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_cloud_squat"));
    }
    frame(agent.lua_state_agent, 48.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_cloud_rise"));
    }
}

unsafe extern "C" fn expression_appealsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 0.2, 210, 2, 0.1, 0, 10, 30, 20, 10);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn game_appealhil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 72.0);
    let hold_button = VarModule::get_int(agent.module_accessor, vars::appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                false,
                hash40("appeal_hi_l_loop"),
                77
            );
        }
    }
}

unsafe extern "C" fn game_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 72.0);
    let hold_button = VarModule::get_int(agent.module_accessor, vars::appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                false,
                hash40("appeal_hi_r_loop"),
                77
            );
        }
    }
}

unsafe extern "C" fn game_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 60.0);
    let hold_button = VarModule::get_int(agent.module_accessor, vars::appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.module_accessor,
                false,
                hash40("appeal_lw_loop"),
                61
            );
        }
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_appealsl", game_appeals, Priority::Low);
    agent.acmd("game_appealsr", game_appeals, Priority::Low);

    agent.acmd("game_appealsloop", game_appealsloop, Priority::Low);
    agent.acmd("sound_appealsloop", sound_appealsloop, Priority::Low);
    agent.acmd("expression_appealsloop", expression_appealsloop, Priority::Low);

    agent.acmd("game_appealhil", game_appealhil, Priority::Low);
    agent.acmd("game_appealhir", game_appealhir, Priority::Low);

    agent.acmd("game_appeallwl", game_appeallw, Priority::Low);
    agent.acmd("game_appeallwr", game_appeallw, Priority::Low);
}