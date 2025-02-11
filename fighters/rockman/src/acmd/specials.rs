use super::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 1.0 / 7.0);
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CHARGESHOT, false, -1);
        VarModule::off_flag(agent.module_accessor, vars::rockman::status::flag::CHARGE_SHOT_KEEP_CHARGE);
    }
}

unsafe extern "C" fn effect_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 8, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW(agent, Hash40::new("rockman_chargeshot_elec"), Hash40::new("havel"), 0, 0, -1.5, 0, 0, 0, 1, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("rockman_chargeshot_shot"), Hash40::new("rockman_chargeshot_shot"), Hash40::new("top"), 0, 7.2, 9, 0, 0, 0, 1, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_rockman_smash_s02"));
    }
}

unsafe extern "C" fn expression_specialn(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_LEFT, *FIGHTER_ROCKMAN_ARMFORM_ROCKBUSTER, 5);
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x1f5b14bb65), *FIGHTER_ROCKMAN_ARM_RIGHT, *FIGHTER_ROCKMAN_ARMFORM_HAND, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
    }
    frame(agent.lua_state_agent, 18.0);
    if WorkModule::get_float(agent.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_HOLD_RATE) < 1.0 {
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::QUAKE(agent, *CAMERA_QUAKE_KIND_M);
            ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beaml"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

unsafe extern "C" fn game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn game_specialairlw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_busterchargeshot", game_specialn, Priority::Low);
    agent.acmd("effect_busterchargeshot", effect_specialn, Priority::Low);
    agent.acmd("sound_busterchargeshot", sound_specialn, Priority::Low);
    agent.acmd("expression_busterchargeshot", expression_specialn, Priority::Low);

    agent.acmd("game_busterairchargeshot", game_specialn, Priority::Low);
    agent.acmd("effect_busterairchargeshot", effect_specialn, Priority::Low);
    agent.acmd("sound_busterairchargeshot", sound_specialn, Priority::Low);
    agent.acmd("expression_busterairchargeshot", expression_specialn, Priority::Low);

    agent.acmd("game_specialhi", game_specialhi, Priority::Low);

    agent.acmd("game_specialairhi", game_specialhi, Priority::Low);

    agent.acmd("game_speciallw", game_speciallw, Priority::Low);

    agent.acmd("game_specialairlw", game_specialairlw, Priority::Low);
}