use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::sv_animcmd;
//use crate::FIGHTER_CUTIN_MANAGER_ADDR;
//use smash::phx::Vector3f;
//use smash::app::{self};
use smash_script::*;
use smash_script::macros;

#[fighter_frame( agent = FIGHTER_KIND_GAOGAEN )]
unsafe fn gaogaen_spinjumpcancel(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND
    && StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_N
    && MotionModule::frame(boma) > 19.0
    && AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
    && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
    }
}

#[script( agent = "gaogaen", script = "game_specialn", category = ACMD_GAME )]
unsafe fn gaogaen_nspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 47, 0, 85, 5.8, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 50, 0, 80, 5.4, 0.0, 11.0, -4.0, Some(0.0), Some(11.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 50, 0, 80, 4.6, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
        macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
            macros::WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
        }
    }
    sv_animcmd::wait(lua_state, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::HIT_RESET_ALL(fighter);
            AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
            AttackModule::set_size(boma, 0, 0.1);
            AttackModule::set_size(boma, 1, 0.1);
            AttackModule::set_size(boma, 2, 0.1);
        }
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 11.0, 4.0, Some(0.0), Some(11.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 11.0, -2.0, Some(0.0), Some(11.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 23.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 29.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 37.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 42.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 50.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 56.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
    }
    if WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::HIT_RESET_ALL(fighter);
            AttackModule::clear_all(boma);
            WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
        }
    }
}

#[script( agent = "gaogaen", script = "game_specialairn", category = ACMD_GAME )]
unsafe fn gaogaen_nspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 47, 0, 85, 4.6, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 50, 0, 80, 4.2, 0.0, 10.0, -4.0, Some(0.0), Some(10.0), Some(-4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 50, 0, 80, 4.6, 0.0, 8.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_START_ROTATION);
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 23.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 29.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 37.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 42.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 50.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 30, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 2, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        AttackModule::set_size(boma, 2, 0.1);
    }
    sv_animcmd::frame(lua_state, 56.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 75, 50, 0, 85, 4.2, 0.0, 10.0, 4.0, Some(0.0), Some(10.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 3.8, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 75, 40, 0, 80, 4.2, 0.0, 8.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_END_ROTATION);
    }
    sv_animcmd::frame(lua_state, 85.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_REQUEST_GRAVITY_DEFAULT);
    }
}

#[script( agent = "gaogaen", script = "game_specialsshoulder", category = ACMD_GAME )]
unsafe fn gaogaen_sspecialshoulder(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 105, 280, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_force_reaction(boma, 0, true, true);
    }
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    sv_animcmd::frame(lua_state, 10.0);
    macros::FT_MOTION_RATE(fighter, 1);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 105, 34, 0, 96, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    // sv_animcmd::frame(lua_state, 17.0);
    // if macros::is_excute(fighter) {
    //     macros::CHECK_FINISH_CAMERA(fighter, 0, 0); //Supposed to be CHECK_FINISH_CAMERA_IS_NOT_HP_MODE
    //     let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
    //     FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
    //     FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 0.0, z: 0.0});
    // }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        macros::REVERSE_LR(fighter);
    }
    sv_animcmd::frame(lua_state, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    sv_animcmd::frame(lua_state, 45.0);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    sv_animcmd::frame(lua_state, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

#[script( agent = "gaogaen", script = "game_specialairsshoulder", category = ACMD_GAME )]
unsafe fn gaogaen_sspecialshoulderair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 105, 280, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_force_reaction(boma, 0, true, true);
    }
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.75);
    sv_animcmd::frame(lua_state, 10.0);
    macros::FT_MOTION_RATE(fighter, 1);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 105, 34, 0, 96, 6.0, 0.0, 6.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    // sv_animcmd::frame(lua_state, 17.0);
    // if macros::is_excute(fighter) {
    //     macros::CHECK_FINISH_CAMERA(fighter, 0, 0); //Supposed to be CHECK_FINISH_CAMERA_IS_NOT_HP_MODE
    //     let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
    //     FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
    //     FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 0.0, z: 0.0});
    // }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        macros::REVERSE_LR(fighter);
    }
    sv_animcmd::frame(lua_state, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    sv_animcmd::frame(lua_state, 45.0);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    sv_animcmd::frame(lua_state, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
    }
}

#[script( agent = "gaogaen", scripts = [ "game_specialslariat", "game_specialairslariat" ], category = ACMD_GAME )]
unsafe fn gaogaen_sspeciallariat(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 11.99);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 145, 454, 0, 20, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_force_reaction(boma, 0, true, true);
        WorkModule::set_float(boma, 9.0, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLOAT_LARIAT_HIT_FRAME);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 16.0, 145, 40, 0, 88, 7.0, 0.0, 0.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        // CHECK_FINISH_CAMERA(0, 0)
        // let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
        // lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.5);
        // lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_DAMAGE_CUT);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        macros::REVERSE_LR(fighter);
    }
    sv_animcmd::frame(lua_state, 50.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_FLAG_ENABLE_GRAVITY);
    }
    sv_animcmd::frame(lua_state, 58.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_S_WORK_ID_FLAG_AIR_CONTROL);
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

#[script( agent = "gaogaen", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn gaogaen_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 361, 70, 0, 50, 9.2, 0.0, 6.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 70, 0, 15, 7.2, 0.0, 6.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(lua_state, 26.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[script( agent = "gaogaen", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn gaogaen_fsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 16.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 16.0, 47, 71, 0, 65, 2.2, 1.4, 0.6, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("legr"), 20.0, 47, 71, 0, 65, 5.4, 8.4, 1.8, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    sv_animcmd::frame(lua_state, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH);
    }
}

#[script( agent = "gaogaen", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn gaogaen_usmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 17.0, 88, 61, 0, 79, 4.4, 4.4, -0.4, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderr"), 17.0, 88, 61, 0, 79, 3.6, 3.4, 0.0, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(lua_state, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
    }
    sv_animcmd::frame(lua_state, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_INSTANCE_WORK_ID_FLAG_IS_FOLLOW_THROUGH);
    }
}

#[script( agent = "gaogaen", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn gaogaen_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(boma, false);
    }
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 16.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 270, 100, 0, 20, 5.4, 0.0, 0.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 361, 67, 0, 50, 5.8, 0.0, 8.2, 0.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 70, 0, 50, 5.4, 0.0, 0.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 70, 0, 50, 5.8, 0.0, 8.2, 0.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(lua_state, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    sv_animcmd::frame(lua_state, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[script( agent = "gaogaen", script = "game_specialairhifall", category = ACMD_GAME )]
unsafe fn gaogaen_upbfall(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 70, 100, 0, 50, 7.0, 0.0, 12.0, 1.0, Some(0.0), Some(7.0), Some(3.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_DISABLE_OPPONENT_PASSIVE);
    }
    macros::FT_MOTION_RATE(fighter, 0.423);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 120, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 54, 54, 0, 135, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(3.0), Some(4.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 100, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 21.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 80, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 28.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 60, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 36.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 40, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
}

#[script( agent = "gaogaen", script = "game_specialairhifall_2", category = ACMD_GAME )]
unsafe fn gaogaen_upbfall2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 70, 100, 0, 50, 7.0, 0.0, 12.0, 1.0, Some(0.0), Some(7.0), Some(2.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_GAOGAEN_STATUS_SPECIAL_HI_FLAG_DISABLE_OPPONENT_PASSIVE);
    }
    macros::FT_MOTION_RATE(fighter, 0.356);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 120, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.0), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        smash_script::macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 54, 54, 0, 135, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(3.0), Some(4.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 14.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 100, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 21.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 80, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 28.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 60, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 36.0);
    if macros::is_excute(fighter) {
        smash_script::macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 312, 100, 40, 0, 6.0, 0.0, 8.0, 2.0, Some(0.0), Some(6.0), Some(3.5), 1.3, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
}

pub fn install() {
    smash_script::replace_fighter_frames!(
        gaogaen_spinjumpcancel
    );
    smash_script::replace_scripts!(
        gaogaen_nspecial,
        gaogaen_nspecialair,
        gaogaen_sspecialshoulder,
        gaogaen_sspecialshoulderair,
        gaogaen_sspeciallariat,
        gaogaen_nair,
        gaogaen_fsmash,
        gaogaen_usmash,
        gaogaen_dair,
        gaogaen_upbfall,
        gaogaen_upbfall2
    );
}