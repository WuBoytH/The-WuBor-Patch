use std::arch::asm;
use {
    smash::{
        lua2cpp::L2CAgentBase,
        phx::Hash40,
        app::{lua_bind::*, sv_animcmd::*},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    super::super::vars::*
};

#[acmd_script( agent = "edge", scripts = [ "game_specialhistart", "game_specialairhistart" ], category = ACMD_GAME, low_priority )]
unsafe fn edge_specialhi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if !WorkModule::is_flag(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL) {
            WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_EDGE_INSTANCE_WORK_ID_INT_SPECIAL_HI_CANCEL_COUNT);
        }
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_EDGE_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_CANCEL);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_DECIDED_RUSH);
    }
    macros::FT_MOTION_RATE(fighter, 22.0 / 17.0);
}

#[acmd_script( agent = "edge", script = "game_specialhi1", category = ACMD_GAME, low_priority )]
unsafe fn edge_specialhi1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        let rush_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE).abs();
        let angle_ratio = 1.0 - (rush_angle / 90.0).clamp(0.0, 1.0);
        let angle = 125.0 - (35.0 * angle_ratio).clamp(0.0, 35.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, angle as u64, 60, 0, 70, 3.0, 0.0, -2.0, 9.0, Some(0.0), Some(-2.0), Some(-4.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "edge", script = "game_specialairhi2end", category = ACMD_GAME, low_priority )]
unsafe fn edge_specialairhi2end(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 3.0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_NONE);
        let rush_angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLOAT_RUSH_DEGREE) * -1.0;
        let base_angle = if rush_angle < 0.0 { 360.0 } else { 0.0 } as f32;
        let angle_ratio = rush_angle / 90.0;
        let angle_diff = 60.0 * angle_ratio;
        let mut angle = base_angle + angle_diff;
        if angle < 15.0 || angle > 345.0 { angle = 361.0 }
        let bkb;
        let kbg;
        if angle < 360.0 && angle > 60.0 {
            bkb = 55;
            kbg = 30;
        }
        else {
            bkb = 64;
            kbg = 104;
        }
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot"), 7.0, angle as u64, kbg, 0, bkb, 10.0, 0.0, 0.0, 6.0, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("rot"), 7.0, angle as u64, kbg, 0, bkb, 10.0, 0.0, 0.0, 18.0, None, None, None, 2.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("rot"), 2.3, 60, 60, 0, 90, 8.5, 0.0, 0.0, -2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_EDGE_STATUS_SPECIAL_HI_FLAG_ENABLE_CONTROL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        edge_specialhi,
        edge_specialhi1,
        edge_specialairhi2end
    );
}