use crate::imports::acmd_imports::*;

#[acmd("cloud", [ "game_appealsl", "game_appealsr" ])]
unsafe fn cloud_appeals(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        VarModule::set_int64(agent.battle_object, appeal::int64::ACTION_MOT, hash40("appeal_s_loop"));
        let hold_button = VarModule::get_int(agent.battle_object, appeal::int::HOLD_BUTTON);
        VarModule::set_int(agent.battle_object, appeal::int::ACTION_BUTTON, hold_button);
        VarModule::on_flag(agent.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(agent.battle_object, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(agent.battle_object, appeal::flag::ENABLE_ACTION_IMM);
    }
}

#[acmd("cloud", "game_appealsloop")]
unsafe fn cloud_appealsloop(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        VarModule::set_int(agent.battle_object, appeal::int::ACTION_FRAME, 12);
        VarModule::on_flag(agent.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        VarModule::on_flag(agent.battle_object, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.battle_object, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        VarModule::off_flag(agent.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(agent.battle_object, appeal::flag::ENABLE_ACTION_IMM);
    }
}

#[acmd("cloud", "sound_appealsloop")]
unsafe fn cloud_appealsloop_snd(agent: &mut L2CAgentBase) {
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

#[acmd("cloud", "expression_appealsloop")]
unsafe fn cloud_appealsloop_exp(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::AREA_WIND_2ND_arg10(agent, 0, 0.2, 210, 2, 0.1, 0, 10, 30, 20, 10);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohits"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd("cloud", "game_appealhil")]
unsafe fn cloud_appealhil(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 72.0);
    let hold_button = VarModule::get_int(agent.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.battle_object,
                false,
                hash40("appeal_hi_l_loop"),
                77
            );
        }
    }
}

#[acmd("cloud", "game_appealhir")]
unsafe fn cloud_appealhir(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 72.0);
    let hold_button = VarModule::get_int(agent.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.battle_object,
                false,
                hash40("appeal_hi_r_loop"),
                77
            );
        }
    }
}

#[acmd("cloud", [ "game_appeallwl", "game_appeallwr" ])]
unsafe fn cloud_appeallw(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 60.0);
    let hold_button = VarModule::get_int(agent.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(agent.module_accessor, hold_button) {
        if macros::is_excute(agent) {
            MiscModule::set_appeal_loop(
                agent.battle_object,
                false,
                hash40("appeal_lw_loop"),
                61
            );
        }
    }
}

pub fn install() {
    cloud_appeals::install();
    cloud_appealsloop::install();
    cloud_appealsloop_snd::install();
    cloud_appealsloop_exp::install();
    cloud_appealhil::install();
    cloud_appealhir::install();
    cloud_appeallw::install();
}