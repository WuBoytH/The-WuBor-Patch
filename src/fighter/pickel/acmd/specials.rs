use crate::imports::acmd_imports::*;

#[acmd_script( agent = "pickel", script = "game_specialsride", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialsride(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 350, 100, 30, 10, 3.0, 0.0, 8.0, 4.5, Some(0.0), Some(4.0), Some(4.5), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA_d, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    macros::FT_MOTION_RATE(fighter, 1.25);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "pickel", script = "game_specialairsride", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialairsride(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
    macros::FT_MOTION_RATE(fighter, 1.25);
    frame(fighter.lua_state_agent, 16.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "pickel_trolley", script = "game_specialsdrivepartial", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialsdrivepartial(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.5, 40, 45, 0, 45, 3.5, 0.0, 3.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::SEARCH(weapon, 1, 0, Hash40::new("top"), 5.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NI, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

#[acmd_script( agent = "pickel_trolley", script = "game_specialsdriveemptypartial", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialsdriveemptypartial(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 5.0, -6.0, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(weapon.module_accessor, 1, true);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 0.0, 6.0, Some(0.0), Some(0.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(weapon.module_accessor, 2, true);
        macros::ATTACK(weapon, 3, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 3.5, 0.0, 3.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_IIE, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(weapon, 4, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 3.5, 0.0, 3.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_down_only(weapon.module_accessor, 4, true);
        macros::SEARCH(weapon, 0, 0, Hash40::new("top"), 3.5, 0.0, 5.0, 1.5, Some(0.0), Some(5.0), Some(-1.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
    }
    wait(weapon.lua_state_agent, 6.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.0, 0.0, 5.0, 6.0, Some(0.0), Some(0.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ignore_ground_shield(weapon.module_accessor, 0, true);
        macros::ATTACK(weapon, 5, 0, Hash40::new("top"), 7.0, 60, 64, 0, 66, 1.5, 0.0, 3.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, f32::NAN, 0.0, 60, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::SEARCH(weapon, 0, 0, Hash40::new("top"), 3.5, 0.0, 3.0, 1.5, Some(0.0), Some(3.0), Some(-1.5), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        macros::SEARCH(weapon, 1, 0, Hash40::new("top"), 5.0, 0.0, 5.0, 0.0, Some(0.0), Some(5.0), Some(0.0), *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NI, 1, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_PICKEL_TROLLEY_INSTANCE_WORK_ID_FLAG_NO_ATTACK_HIT_MOTION);
    }
}

#[acmd_script( agent = "pickel", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn pickel_specialairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        // notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        JostleModule::set_status(fighter.module_accessor, false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 5.0, 70, 100, 0, 30, 2.6, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        // notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 3.0, 70, 100, 0, 30, 1.7, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_SPECIAL_HI_FLAG_ENABLE_CANCEL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        pickel_specialsride,
        pickel_specialairsride,
        pickel_specialsdrivepartial,
        pickel_specialsdriveemptypartial,
        pickel_specialairhi
    );
}