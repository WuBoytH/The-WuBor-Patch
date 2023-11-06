use crate::imports::acmd_imports::*;

#[acmd_script( agent = "ridley", scripts = [ "game_specialsstart", "game_specialairsstart" ], category = ACMD_GAME, low_priority )]
unsafe extern "C" fn ridley_specialsstart(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 1);
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 8.0, 6.0, 7.5, 7.5);
    }
    frame(agent.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(agent, 0.5);
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_JUMP);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(agent.module_accessor, 7.0, 6.0, 7.5, 7.5);
    }
    frame(agent.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::CATCH(agent, 0, Hash40::new("top"), 6.8, 0.0, 7.2, 16.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(agent, 1, Hash40::new("top"), 3.3, 0.0, 7.2, 16.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(agent, 2, Hash40::new("top"), 5.0, 0.0, 7.2, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        GrabModule::set_constraint(agent.module_accessor, 0, true);
        GrabModule::set_constraint(agent.module_accessor, 1, true);
        GrabModule::set_constraint(agent.module_accessor, 2, true);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(agent.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 37.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        grab!(agent, MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(agent.lua_state_agent, 49.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargef", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn ridley_specialairhichargef(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 2);
        macros::HIT_NODE(agent, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 40, 66, 0, 75, 8.0, 0.0, 5.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargeb", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn ridley_specialairhichargeb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 2);
        macros::HIT_NODE(agent, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 16.0, 78, 60, 0, 80, 5.0, 0.0, 16.3, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 16.0, 78, 60, 0, 80, 8.0, 0.0, 13.5, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargehi", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn ridley_specialairhichargehi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 2);
        macros::HIT_NODE(agent, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(agent, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 18.0, 64, 46, 0, 85, 6.5, 0.0, 25.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 18.0, 64, 46, 0, 85, 10.0, 0.0, 15.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(agent.lua_state_agent, 25.0);
    if macros::is_excute(agent) {
        HitModule::set_status_all(agent.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(agent.module_accessor);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargelw", category = ACMD_GAME, low_priority )]
unsafe extern "C" fn ridley_specialairhichargelw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        GroundModule::select_cliff_hangdata(agent.module_accessor, 2);
        JostleModule::set_status(agent.module_accessor, false);
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 15.0, 295, 75, 0, 50, 6.5, 0.0, 4.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 15.0, 44, 50, 0, 85, 6.5, 0.0, 4.5, 2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(agent.lua_state_agent, 19.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

pub fn install(agent : &mut smashline::Agent) {
    agent.game_acmd("game_specialsstart", ridley_specialsstart);

    agent.game_acmd("game_specialairsstart", ridley_specialsstart);

    agent.game_acmd("game_specialairhichargef", ridley_specialairhichargef);

    agent.game_acmd("game_specialairhichargeb", ridley_specialairhichargeb);

    agent.game_acmd("game_specialairhichargehi", ridley_specialairhichargehi);

    agent.game_acmd("game_specialairhichargelw", ridley_specialairhichargelw);
}