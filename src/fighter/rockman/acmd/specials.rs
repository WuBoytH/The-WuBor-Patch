use crate::imports::acmd_imports::*;

#[acmd_script( agent = "rockman", scripts = [ "game_busterchargeshot", "game_busterairchargeshot" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn rockman_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    macros::FT_MOTION_RATE(agent, 1.0 / 7.0);
    frame(agent.lua_state_agent, 18.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_CHARGESHOT, false, -1);
        VarModule::off_flag(agent.module_accessor, rockman::status::flag::CHARGE_SHOT_KEEP_CHARGE);
    }
}

#[acmd_script( agent = "rockman", scripts = [ "effect_busterchargeshot", "effect_busterairchargeshot" ], category = ACMD_EFFECT, low_priority )]
unsafe extern "C" fn rockman_specialn_eff(agent: &mut L2CAgentBase) {
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

#[acmd_script( agent = "rockman", scripts = [ "sound_busterchargeshot", "sound_busterairchargeshot" ], category = ACMD_SOUND, low_priority )]
unsafe extern "C" fn rockman_specialn_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_rockman_smash_s02"));
    }
}

#[acmd_script( agent = "rockman", scripts = [ "expression_busterchargeshot", "expression_busterairchargeshot" ], category = ACMD_EXPRESSION, low_priority )]
unsafe extern "C" fn rockman_specialn_exp(agent: &mut L2CAgentBase) {
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

#[acmd_script( agent = "rockman", scripts = [ "game_specialhi", "game_specialairhi" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn rockman_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES); // Was ALWAYS_BOTH_SIDES
    }
}

#[acmd_script( agent = "rockman", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn rockman_speciallw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

#[acmd_script( agent = "rockman", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn rockman_specialairlw(agent: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(agent, 5.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_ROCKMAN_GENERATE_ARTICLE_LEAFSHIELD, false, -1);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_busterchargeshot", rockman_specialn);
    agent.effect_acmd("effect_busterchargeshot", rockman_specialn_eff);
    agent.sound_acmd("sound_busterchargeshot", rockman_specialn_snd);
    agent.expression_acmd("expression_busterchargeshot", rockman_specialn_exp);

    agent.game_acmd("game_busterairchargeshot", rockman_specialn);
    agent.effect_acmd("effect_busterairchargeshot", rockman_specialn_eff);
    agent.sound_acmd("sound_busterairchargeshot", rockman_specialn_snd);
    agent.expression_acmd("expression_busterairchargeshot", rockman_specialn_exp);

    agent.game_acmd("game_specialhi", rockman_specialhi);

    agent.game_acmd("game_specialairhi", rockman_specialhi);

    agent.game_acmd("game_speciallw", rockman_speciallw);

    agent.game_acmd("game_specialairlw", rockman_specialairlw);
}