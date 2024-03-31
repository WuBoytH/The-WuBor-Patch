use crate::imports::*;

unsafe extern "C" fn game_specialn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 1.1);
    frame(agent.lua_state_agent, 15.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 17.0);
    if WorkModule::get_int64(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) != *FIGHTER_KIND_KIRBY as u64 {
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("mouth2"), 4.0, -0.9, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 1, Hash40::new("mouth2"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::CATCH(agent, 0, Hash40::new("throw"), 4.0, -0.9, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_A);
            macros::CATCH(agent, 1, Hash40::new("throw"), 3.5, -2.0, -0.5, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_G);
            macros::CATCH(agent, 2, Hash40::new("top"), 5.9, 0.0, 7.0, 9.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_YOSHI, *COLLISION_SITUATION_MASK_GA);
        }
    }
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 1.0, 0, 50, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR_ALL);
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn game_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(agent, 0.85);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_APPEAR);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_SHOOT);
        VarModule::on_flag(agent.module_accessor, yoshi::status::flag::SPECIAL_S_HOP);
    }
    macros::FT_MOTION_RATE(agent, 1.0);
}

unsafe extern "C" fn effect_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_attack_impact"), Hash40::new("top"), 4, 10.5, -11, 0, 0, 0, 2, 0, 0, 0, 0, 0, 360, true);
    }
    frame(agent.lua_state_agent, 16.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), 2, 20, 9, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_specials(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_yoshi_special_h01"));
    }
}

unsafe extern "C" fn expression_specials(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(agent.lua_state_agent, 18.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_lightthrow4item"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_specialn", game_specialn);

    agent.acmd("game_specialairn", game_specialn);

    agent.acmd("game_specials", game_specials);
    agent.acmd("effect_specials", effect_specials);
    agent.acmd("sound_specials", sound_specials);
    agent.acmd("expression_specials", expression_specials);

    agent.acmd("game_specialairs", game_specials);
    agent.acmd("effect_specialairs", effect_specials);
    agent.acmd("sound_specialairs", sound_specials);
    agent.acmd("expression_specialairs", expression_specials);
}