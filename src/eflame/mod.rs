use smash::phx::Hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterBase, L2CFighterCommon};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use crate::system::IS_FUNNY;
use crate::commonfuncs::*;

static mut CALLBACK : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_EFLAME )]
fn eflame_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if entry_id(fighter.module_accessor) < 8 {
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_S
            && MotionModule::frame(fighter.module_accessor) >= 20.0 {
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH);
                    WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN);
                    WorkModule::enable_transition_term(fighter.module_accessor,*FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
                }
                if ControlModule::check_button_trigger(fighter.module_accessor,*CONTROL_PAD_BUTTON_SPECIAL) == true {
                    CALLBACK[entry_id(fighter.module_accessor)] = true;
                    StatusModule::change_status_request_from_script(fighter.module_accessor,*FIGHTER_EFLAME_STATUS_KIND_SPECIAL_S_CATCH,true);
                }
            }

            if IS_FUNNY[entry_id(fighter.module_accessor)] {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW
                && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
                && is_damage_check(fighter.module_accessor) == false {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
                }
            }
        }
    }
}

#[weapon_frame( agent = WEAPON_KIND_EFLAME_ESWORD )]
fn eflame_esword_frame(weapon: &mut L2CFighterBase) {
    unsafe {
        if CALLBACK[entry_id(weapon.module_accessor)] {
            StatusModule::change_status_request_from_script(weapon.module_accessor,*WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_BACK,true);
            if StatusModule::status_kind(weapon.module_accessor) == *WEAPON_EFLAME_ESWORD_STATUS_KIND_SPECIAL_S_BACK {
                CALLBACK[entry_id(weapon.module_accessor)] = false;
            }
        }
    }
}

#[acmd_script( agent = "eflame", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn eflame_dair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    macros::FT_MOTION_RATE(fighter, 1.333);
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if ArticleModule::is_exist(fighter.module_accessor, 17452) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(fighter.module_accessor, 17452, 60296, Hash40::new_raw(0x07439e926b), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(fighter.module_accessor) == true {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, 60300);
        }
    }
    sv_animcmd::frame(lua_state, 15.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 3.0, 0.0, 0.0, 3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -1.0, 7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -2.0, 11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 2.4, 0.0, -3.0, 14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 270, 80, 0, 30, 3.0, 0.0, 2.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 3.0, 0.0, -2.0, -2.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -7.0, -1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -10.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 2.4, 0.0, -13.0, 1.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 14.0, 270, 80, 0, 30, 3.0, 0.0, 2.0, -3.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 3.0, 0.0, 1.0, -11.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -2.0, -13.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -5.0, -14.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 2.4, 0.0, -8.0, -15.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 60, 85, 0, 65, 3.0, 0.0, 4.0, -7.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 3.0, 0.0, 6.0, -13.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 2.4, 0.0, 5.0, -16.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 2.4, 0.0, 4.0, -19.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 2.4, 0.0, 3.0, -22.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 12.0, 60, 65, 0, 65, 3.0, 0.0, 8.0, -8.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(lua_state, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(lua_state, 26.0);
    if ArticleModule::is_exist(fighter.module_accessor, 17452) {
        if macros::is_excute(fighter) {
            ArticleModule::add_motion_partial(fighter.module_accessor, 17452, 60296, Hash40::new_raw(0x08183db0f4), 5.0, 5.0, false, false, 0.0, false, true, false);
        }
    }
    if MotionModule::is_changing(fighter.module_accessor) == true {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, 60300);
        }
    }
    sv_animcmd::frame(lua_state, 33.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        eflame_frame,
        eflame_esword_frame
    );
    smashline::install_acmd_scripts!(
        eflame_dair
    );
}