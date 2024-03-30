use crate::imports::*;

unsafe extern "C" fn demon_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.4, 361, 100, 30, 0, 5.0, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_CATCHATTACK, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn demon_catchattack_eff(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("demon_attack_arc"), Hash40::new("top"), 2.5, 10.7, 4.2, 16, -24, -70, 0.45, true, 1);
        macros::LAST_EFFECT_SET_RATE(agent, 0.95);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::EFFECT_ALPHA(agent, Hash40::new("sys_attack_impact"), Hash40::new("handl"), 1.3, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 360, true, 0.3);
        macros::LAST_EFFECT_SET_RATE(agent, 1.2);
    }
}

unsafe extern "C" fn demon_catchattack_snd(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::PLAY_SEQUENCE(agent, Hash40::new("seq_demon_rnd_catchattack"));
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_demon_throw_catch_swing"));
    }
}

unsafe extern "C" fn demon_catchcommand(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    macros::FT_MOTION_RATE(agent, 1.6);
    frame(agent.lua_state_agent, 3.0);
    macros::FT_START_ADJUST_MOTION_FRAME_arg1(agent, 1.0);
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 6.6, 5.0, Some(0.0), Some(6.6), Some(11.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 2.0, 0.0, 6.6, 3.0, Some(0.0), Some(6.6), Some(13.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_A);
    }
    macros::game_CaptureCutCommon(agent);
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_DEMON_STATUS_CATCH_COMMAND_FLAG_CHANGE_THROW);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_catchattack", demon_catchattack);
    agent.acmd("effect_catchattack", demon_catchattack_eff);
    agent.acmd("sound_catchattack", demon_catchattack_snd);

    agent.acmd("game_catchcommand", demon_catchcommand);
}