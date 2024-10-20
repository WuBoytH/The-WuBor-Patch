use super::*;

unsafe extern "C" fn game_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 6.5, 4.0, Some(0.0), Some(6.5), Some(8.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    macros::FT_MOTION_RATE(agent, 1.217);
}

unsafe extern "C" fn game_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 2.4, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(10.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    frame(agent.lua_state_agent, 13.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    macros::FT_MOTION_RATE(agent, 1.18);
}

unsafe extern "C" fn game_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        GrabModule::set_rebound(agent.module_accessor, true);
    }
    frame(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-14.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    let animcmd: &mut L2CFighterAnimcmdGameCommon = std::mem::transmute(&mut *agent);
    L2CFighterAnimcmdGameCommon::game_CaptureCutCommon(animcmd);
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(agent.module_accessor, false);
    }
    macros::FT_MOTION_RATE(agent, 1.08);
}

unsafe extern "C" fn game_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 2.0, 361, 100, 30, 0, 5.5, 0.0, 11.0, 9.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
    }
    wait(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_catchattack(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if get_value_float(agent.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_catch_impact"), Hash40::new("top"), 0.4, 12, 10, 20, 0, 0, 1, true);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_catch_line"), Hash40::new("top"), 0, 14, 0, 8, 0, 0, 1, true);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_catch_claw_r"), Hash40::new("wingr3"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_catch_claw_l"), Hash40::new("wingl3"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    else {
        if macros::is_excute(agent) {
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_catch_impact"), Hash40::new("top"), 0.4, 12, 8, 20, 0, 0, 1, true);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_catch_line"), Hash40::new("top"), 0, 14, 0, 8, 0, 0, 1, true);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_catch_claw_r"), Hash40::new("wingr3"), 0, 0, 0, 0, 0, 0, 1, true);
            macros::EFFECT_FOLLOW_NO_STOP(agent, Hash40::new("metaknight_catch_claw_l"), Hash40::new("wingl3"), 0, 0, 0, 0, 0, 0, 1, true);
        }
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::EFFECT_OFF_KIND(agent, Hash40::new("metaknight_catch"), false, false);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("metaknight_catch_claw_r"), true, true);
        macros::EFFECT_OFF_KIND(agent, Hash40::new("metaknight_catch_claw_l"), true, true);
    }
}

unsafe extern "C" fn expression_catchattack(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_attacks"), 0);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        VisibilityModule::set_status_default_int64(agent.module_accessor, hash40("mantle") as i64, hash40("mantle_normal") as i64);
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