use smash::phx::Hash40;
// use smash::hash40;
// use smash::phx::Vector2f;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lib::L2CValue;
use smash_script::*;
use smashline::*;
// use crate::system::IS_FUNNY;
use crate::globals::*;
use crate::commonfuncs::*;

pub static mut DRAGON_INSTALL : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_KAMUI )]
fn kamui_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        
        // Force Ganon's state to the final part of Aerial Flame Choke.

        // if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_FALL {
        //     fighter.change_status(FIGHTER_GANON_STATUS_KIND_SPECIAL_AIR_S_END.into(), false.into());
        // }

        if entry_id(fighter.module_accessor) < 8 {

            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
                DRAGON_INSTALL[entry_id(fighter.module_accessor)] = false;
            }
        }
    }
}

#[status_script(agent = "kamui", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn kamui_speciallwpre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_VISIBILITY);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    L2CValue::I32(0)
}

#[status_script(agent = "kamui", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn kamui_speciallw(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    kamui_speciallwmotion(fighter);
    kamui_speciallwdragonmotion(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(kamui_speciallwmain as *const () as _))
}

unsafe extern "C" fn kamui_speciallwmotion(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_lw_hit"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_lw_hit"), -1.0, 1.0, 0.0, false, false);
        }
    }
    return
}

unsafe extern "C" fn kamui_speciallwdragonmotion(fighter: &mut L2CFighterCommon) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_air_lw_hit"), false, -1.0);
            }
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_air_lw_hit"), true, -1.0);
            }
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KAMUI_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_lw_hit"), false, -1.0);
            }
            else {
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_lw_hit"), true, -1.0);
            }
        }
    }
    return
}

unsafe extern "C" fn kamui_speciallwmain(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut val = 0;
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(L2CValue::I32(0x80)).get_bool() == true
        && fighter.sub_air_check_fall_common().get_bool() == true {
            val = 1;
        }
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        kamui_speciallwmotion(fighter);
        kamui_speciallwdragonmotion(fighter);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(val)
}

#[status_script(agent = "kamui", status = FIGHTER_STATUS_KIND_SPECIAL_LW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn kamui_speciallwend(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[STATUS_KIND].get_i32() != *FIGHTER_STATUS_KIND_FINAL_VISUAL_ATTACK_OTHER {
        if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
            VisibilityModule::set_whole(fighter.module_accessor, true);
        }
    }
    L2CValue::I32(0)
}

#[acmd_script( agent = "kamui", scripts = ["game_speciallwhit", "game_specialairlwhit"], category = ACMD_GAME, low_priority )]
unsafe fn kamui_speciallwhit(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, 0);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, 0.5);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, 1.0);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
}

#[acmd_script( agent = "kamui_waterdragon", scripts = ["game_speciallwhit", "game_specialairlwhit"], category = ACMD_GAME, low_priority )]
unsafe fn kamui_waterdragon_speciallwhit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 12.0, 0.0, 6.0, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 12.0, 0.0, 6.0, -11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 12.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 12.0, false);
        AttackModule::set_optional_hit_sound(fighter.module_accessor, 0, Hash40::new("se_kamui_criticalhit"));
        AttackModule::set_optional_hit_sound(fighter.module_accessor, 1, Hash40::new("se_kamui_criticalhit"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 8.0, 0.0, 21.0, 11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 361, 30, 0, 60, 8.0, 0.0, 21.0, -11.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_OBJECT);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 12.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 12.0, false);
        AttackModule::set_optional_hit_sound(fighter.module_accessor, 2, Hash40::new("se_kamui_criticalhit"));
        AttackModule::set_optional_hit_sound(fighter.module_accessor, 3, Hash40::new("se_kamui_criticalhit"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        kamui_frame
    );
    smashline::install_status_scripts!(
        kamui_speciallwpre,
        kamui_speciallw,
        kamui_speciallwend
    );
    smashline::install_acmd_scripts!(
        kamui_speciallwhit,
        kamui_waterdragon_speciallwhit
    );
}