use smash::phx::Hash40;
use smash::hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::ITEM_MANAGER;
use crate::commonfuncs::*;
use crate::system::{IS_FUNNY/*, IS_DK*/};
use skyline::nn::ro::LookupSymbol;
use crate::globals::*;
use smash::lib::L2CValue;

// ---------------------------------------------------------
// Heck, even giving DK a nerfed form of the barrel is a massive buff. But we nerfed his weight and
// didn’t compensate, and his recovery and disadvantage are still garbage, so...
// ---------------------------------------------------------

// Weight: 127 > 120


#[status_script(agent = "donkey", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn donkey_specials(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if !barrel_check(fighter.module_accessor) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_r"), 1.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_lw_l"), 1.0, 1.0, false, 0.0, false, false);
        }
    }
    else {
        fighter.sub_change_motion_by_situation(L2CValue::Hash40(Hash40::new("special_s")), L2CValue::Hash40(Hash40::new("special_air_s")), L2CValue::Bool(false));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(donkey_specialsmain as *const () as _))
}

unsafe extern "C" fn donkey_specialsmain(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) != hash40("appeal_lw_r")
    && MotionModule::motion_kind(fighter.module_accessor) != hash40("appeal_lw_l") {
        if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s"), -1.0, 1.0, 0.0, false, false);
            }
            else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(L2CValue::I32(112)).get_bool() == false
        && fighter.sub_air_check_fall_common().get_bool() == false {
            return L2CValue::I32(1);
        }
    }
    L2CValue::I32(0)
}


// Forward Tilt has 2 more damage on all versions and has an earlier cancel frame (34 -> 31).

#[acmd_script( agent = "donkey", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn donkey_ftilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.9);
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 10.0, 361, 78, 0, 50, 5.5, 10.0, 0.0, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 361, 78, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0, 361, 78, 0, 50, 3.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "donkey", script = "game_attacks3hi", category = ACMD_GAME, low_priority )]
unsafe fn donkey_ftilthi(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.91);
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 11.0, 361, 78, 0, 50, 5.5, 10.0, 0.0, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 11.0, 361, 78, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 11.0, 361, 78, 0, 50, 3.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "donkey", script = "game_attacks3lw", category = ACMD_GAME, low_priority )]
unsafe fn donkey_ftiltlw(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.91);
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 11.0, 361, 78, 0, 50, 5.5, 10.0, 0.0, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 11.0, 361, 78, 0, 50, 5.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 11.0, 361, 78, 0, 50, 3.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

// Up Tilt has 1 more damage on all hitboxes and has an earlier cancel frame (39 -> 35).

#[acmd_script( agent = "donkey", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn donkey_utilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderr"), 9.0, 100, 105, 0, 40, 4.8, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0, 100, 110, 0, 40, 4.0, 3.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 11.0, 100, 115, 0, 40, 5.3, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

// Down Tilt has a new angle (361 -> 70) and more Knockback Growth (80 -> 120).

#[acmd_script( agent = "donkey", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn donkey_dtilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 6.0, 70, 120, 0, 10, 4.4, 2.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 6.0, 70, 120, 0, 10, 4.4, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 6.0, 70, 120, 0, 10, 5.5, 8.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

// Instead of Headbutt, Donkey Kong now spawns a Barrel item.

#[acmd_script( agent = "donkey", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn donkey_sspecial(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.5);
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
        if ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_BARREL {
            if IS_FUNNY[entry_id(fighter.module_accessor)] == false && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_FALL, true);
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
            }
        }
    }
}

#[acmd_script( agent = "donkey", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn donkey_sspecialair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 30.0/19.0);
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        ItemModule::have_item(fighter.module_accessor, ItemKind(*ITEM_KIND_BARREL), 0, 0, false, false);
        if ItemModule::get_have_item_kind(fighter.module_accessor, 0) == *ITEM_KIND_BARREL {
            if IS_FUNNY[entry_id(fighter.module_accessor)] == false && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_KIND_SUPER_LIFT_FALL, true);
            }
            else {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ITEM_HEAVY_PICKUP, true);
            }
        }
    }
}

pub unsafe fn barrel_check(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    LookupSymbol(
        &mut ITEM_MANAGER,
        "_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E\u{0}"
        .as_bytes()
        .as_ptr(),
    );
    let item_manager = *(ITEM_MANAGER as *mut *mut smash::app::ItemManager);
    // let mut dks = 0;
    // for i in 0..IS_DK.len() {
    //     if IS_DK[i] {
    //         dks += 1;
    //     }
    // }
    // if smash::app::lua_bind::ItemManager::get_num_of_active_item_all(item_manager) >= dks * 2 {
    //     return false;
    // }
    let entry_id = entry_id(module_accessor) as u32;
    if smash::app::lua_bind::ItemManager::get_num_of_ownered_item(item_manager, entry_id, ItemKind(*ITEM_KIND_BARREL)) >= 2 {
        return false;
    }
    return true;
}

pub fn install() {
    smashline::install_status_scripts!(
        donkey_specials
    );
    smashline::install_acmd_scripts!(
        donkey_ftilt,
        donkey_ftilthi,
        donkey_ftiltlw,
        donkey_utilt,
        donkey_dtilt,
        donkey_sspecial,
        donkey_sspecialair
    );
}