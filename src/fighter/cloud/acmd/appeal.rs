use crate::imports::acmd_imports::*;

#[acmd_script( agent = "cloud", script = "game_appealsl", category = ACMD_GAME, low_priority )]
unsafe fn cloud_appealsl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        VarModule::set_int64(fighter.battle_object, appeal::int64::ACTION_MOT, hash40("appeal_s_loop"));
        VarModule::set_int(fighter.battle_object, appeal::int::ACTION_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_S_L);
        VarModule::on_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(fighter.battle_object, appeal::flag::ENABLE_ACTION_IMM);
    }
}

#[acmd_script( agent = "cloud", script = "game_appealsr", category = ACMD_GAME, low_priority )]
unsafe fn cloud_appealsr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        VarModule::set_int64(fighter.battle_object, appeal::int64::ACTION_MOT, hash40("appeal_s_loop"));
        VarModule::set_int(fighter.battle_object, appeal::int::ACTION_BUTTON, *CONTROL_PAD_BUTTON_APPEAL_S_R);
        VarModule::on_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(fighter.battle_object, appeal::flag::ENABLE_ACTION_IMM);
    }
}

#[acmd_script( agent = "cloud", script = "game_appealsloop", category = ACMD_GAME, low_priority )]
unsafe fn cloud_appealsloop(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        VarModule::set_int(fighter.battle_object, appeal::int::ACTION_FRAME, 12);
        VarModule::on_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        VarModule::on_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_ENABLE_SUCCESS);
    }
    frame(fighter.lua_state_agent, 75.0);
    if macros::is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, appeal::flag::ACTION_BUTTON_CHECK);
        VarModule::on_flag(fighter.battle_object, appeal::flag::ENABLE_ACTION_IMM);
    }
}

#[acmd_script( agent = "cloud", script = "game_appealhil", category = ACMD_GAME, low_priority )]
unsafe fn cloud_appealhil(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 72.0);
    let hold_button = VarModule::get_int(fighter.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(fighter.module_accessor, hold_button) {
        if macros::is_excute(fighter) {
            MiscModule::set_appeal_loop(
                fighter.battle_object,
                false,
                hash40("appeal_hi_l_loop"),
                77
            );
        }
    }
}

#[acmd_script( agent = "cloud", script = "game_appealhir", category = ACMD_GAME, low_priority )]
unsafe fn cloud_appealhir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 72.0);
    let hold_button = VarModule::get_int(fighter.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(fighter.module_accessor, hold_button) {
        if macros::is_excute(fighter) {
            MiscModule::set_appeal_loop(
                fighter.battle_object,
                false,
                hash40("appeal_hi_r_loop"),
                77
            );
        }
    }
}

#[acmd_script( agent = "cloud", scripts = [ "game_appeallwl", "game_appeallwr" ], category = ACMD_GAME, low_priority )]
unsafe fn cloud_appeallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 60.0);
    let hold_button = VarModule::get_int(fighter.battle_object, appeal::int::HOLD_BUTTON);
    if ControlModule::check_button_on(fighter.module_accessor, hold_button) {
        if macros::is_excute(fighter) {
            MiscModule::set_appeal_loop(
                fighter.battle_object,
                false,
                hash40("appeal_lw_loop"),
                61
            );
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        cloud_appealsl,
        cloud_appealsr,
        cloud_appealsloop,
        cloud_appealhil,
        cloud_appealhir,
        cloud_appeallw
    );
}