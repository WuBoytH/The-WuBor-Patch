use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use smash::phx::Vector3f;

#[acmd_script( agent = "palutena", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn palutena_dtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 8.5, 55, 100, 0, 40, 2.7, -0.5, 8.0, 0.0, Some(-0.5), Some(-7.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 5.0, 55, 100, 0, 38, 2.7, -0.5, 8.0, 0.0, Some(-0.5), Some(-7.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.5, 55, 100, 0, 40, 2.5, 0.0, 1.0, 19.5, Some(0.0), Some(2.0), Some(9.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 5.0, 55, 100, 0, 38, 2.5, 0.0, 1.0, 19.5, Some(0.0), Some(2.0), Some(9.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(lua_state, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "palutena", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn palutena_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 1.4, 367, 100, 0, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 1.4, 367, 100, 0, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 2, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 3, 0, Hash40::new("stick"), 1.4, 95, 100, 80, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    sv_animcmd::frame(lua_state, 24.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 1.4, 367, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 1.4, 367, 100, 65, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 2, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, 5.2, 0.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 3, 0, Hash40::new("stick"), 1.4, 100, 100, 65, 0, 4.2, 0.0, -5.4, 0.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 1, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, -13.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, -13.0, false);
    }
    sv_animcmd::frame(lua_state, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(lua_state, 29.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("stick"), 5.1, 55, 155, 0, 35, 9.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("stick"), 5.1, 55, 155, 0, 35, 9.5, 0.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
    }
    sv_animcmd::frame(lua_state, 31.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(lua_state, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "palutena", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn palutena_bair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    sv_animcmd::frame(lua_state, 3.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("virtualshield"), *HIT_STATUS_INVINCIBLE);
    }
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_INVINCIBLE);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_INVINCIBLE);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 95, 0, 30, 7.0, 0.0, 10.2, -14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::frame(lua_state, 11.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("virtualshield"), *HIT_STATUS_OFF);
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(lua_state, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "palutena", script = "game_throwb", category = ACMD_GAME, low_priority )]
unsafe fn palutena_bthrow(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 45, 70, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 45, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(lua_state, 19.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(fighter, 14.0, 19.0);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
        lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 6.0, y: 8.0, z: 0.0});
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        palutena_dtilt,
        palutena_nair,
        palutena_bair,
        palutena_bthrow
    );
}