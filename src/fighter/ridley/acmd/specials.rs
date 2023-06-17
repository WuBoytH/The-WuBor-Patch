use crate::imports::acmd_imports::*;

#[acmd_script( agent = "ridley", scripts = [ "game_specialsstart", "game_specialairsstart" ], category = ACMD_GAME, low_priority )]
unsafe fn ridley_specialsstart(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 1);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 8.0, 6.0, 7.5, 7.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_START_JUMP);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 7.0, 6.0, 7.5, 7.5);
    }
    frame(fighter.lua_state_agent, 23.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 6.8, 0.0, 7.2, 16.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.3, 0.0, 7.2, 16.0, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_A);
        macros::CATCH(fighter, 2, Hash40::new("top"), 5.0, 0.0, 7.2, 6.5, None, None, None, *FIGHTER_STATUS_KIND_CATCHED_RIDLEY, *COLLISION_SITUATION_MASK_G);
        GrabModule::set_constraint(fighter.module_accessor, 0, true);
        GrabModule::set_constraint(fighter.module_accessor, 1, true);
        GrabModule::set_constraint(fighter.module_accessor, 2, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR, 2);
    }
    frame(fighter.lua_state_agent, 28.0);
    macros::FT_MOTION_RATE(fighter, 0.8);
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(fighter.lua_state_agent, 37.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        grab!(fighter, MA_MSC_CMD_GRAB_CLEAR_ALL);
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RIDLEY_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargef", category = ACMD_GAME, low_priority )]
unsafe fn ridley_specialairhichargef(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 2);
        macros::HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 40, 66, 0, 75, 8.0, 0.0, 5.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargeb", category = ACMD_GAME, low_priority )]
unsafe fn ridley_specialairhichargeb(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 2);
        macros::HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 78, 60, 0, 80, 5.0, 0.0, 16.3, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 78, 60, 0, 80, 8.0, 0.0, 13.5, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargehi", category = ACMD_GAME, low_priority )]
unsafe fn ridley_specialairhichargehi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 2);
        macros::HIT_NODE(fighter, Hash40::new("wingr2"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("wingl2"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 64, 46, 0, 85, 6.5, 0.0, 25.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 64, 46, 0, 85, 10.0, 0.0, 15.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "ridley", script = "game_specialairhichargelw", category = ACMD_GAME, low_priority )]
unsafe fn ridley_specialairhichargelw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, 2);
        JostleModule::set_status(fighter.module_accessor, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 295, 75, 0, 50, 6.5, 0.0, 4.5, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 15.0, 44, 50, 0, 85, 6.5, 0.0, 4.5, 2.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_acmd_scripts!(
        ridley_specialsstart,
        ridley_specialairhichargef,
        ridley_specialairhichargeb,
        ridley_specialairhichargehi,
        ridley_specialairhichargelw
    );
}