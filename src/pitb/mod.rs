use smash::{
    lua2cpp::{L2CFighterCommon, L2CAgentBase},
    hash40,
    phx::{Hash40, Vector3f},
    app::{lua_bind::*, sv_animcmd::*, *},
    lib::{lua_const::*, L2CValue}
};
use smash_script::*;
use smashline::*;
use crate::{
    table_const::*,
    vars::*
};

#[status_script(agent = "pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn pitb_specialncharge_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_DIR_S) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold_hi") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold_hi") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
        WorkModule::set_int64(fighter.module_accessor, 0x7cabbcbb5, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
        WorkModule::set_int64(fighter.module_accessor, 0xbd2abc95c, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_n_hold_s") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_hold_s") as i64, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
        WorkModule::set_int64(fighter.module_accessor, 0x684068652, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
        WorkModule::set_int64(fighter.module_accessor, 0xa23431885, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(pitb_specialncharge_charge as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(pitb_specialncharge_loop as *const () as _))
}

unsafe extern "C" fn pitb_specialncharge_charge(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_CHARGE);
    0.into()
}

unsafe extern "C" fn pitb_specialncharge_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PIT_SPECIAL_AIR_N);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST) {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), true, -1.0);
        }
        else {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION_AIR);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION_AIR);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), false, -1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST) {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), true, -1.0);
        }
        else {
            let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_MOTION);
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            let bowmot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_BOW_MOTION);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(bowmot), false, -1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_FIRST);
        }
    }
    let curr_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_INT_CHARGE);
    let max_charge = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("charge_frame"));
    if !ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
    && curr_charge < max_charge {
        let sticky = fighter.global_table[STICK_Y].get_f32();
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_DIR_S) {
            let upsticky = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("up_stick_y"));
            if sticky < upsticky {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR.into(), true.into());
            }
        }
        else {
            let upsticky = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("up_stick_y"));
            if sticky >= upsticky {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR.into(), true.into());
            }
        }
    }
    else {
        if curr_charge >= max_charge {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX);
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        }
        else {
            // let stickx = fighter.global_table[STICK_X].get_f32() * PostureModule::lr(fighter.module_accessor);
            // let turnstickx = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("turn_stick_x"));
            // if stickx <= turnstickx {
            //     fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN.into(), true.into());
            // }
            // else {
                fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), true.into());
            // }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn pitb_specialnturn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::reverse_lr(fighter.module_accessor);
    fighter.sub_shift_status_main(L2CValue::Ptr(pitb_specialnturn_loop as *const () as _))
}

unsafe extern "C" fn pitb_specialnturn_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor)
    || !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_PIT_SPECIAL_AIR_N);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_s_to_s"), 0.0, 1.0, false, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(0xa728bc2ca), false, -1.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_s_to_s"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(0xa728bc2ca), true, -1.0);
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        }
        else {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_s_to_s"), 0.0, 1.0, false, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x6d5ce5c1d), false, -1.0);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_SHOOT_FLAG_FIRST);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_s_to_s"), -1.0, 1.0, 0.0, false, false);
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, Hash40::new_raw(0x6d5ce5c1d), true, -1.0);
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
    }
    L2CValue::I32(0)
}

#[status_script(agent = "pitb", status = FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn pitb_specialnshoot_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    pitb_specialn_endremove(fighter)
}

unsafe extern "C" fn pitb_specialn_endremove(fighter: &mut L2CFighterCommon) -> L2CValue {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_DIR
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_TURN
    && status != *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT {
        ArticleModule::remove(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_PIT_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    L2CValue::I32(0)
}

#[acmd_script( agent = "pitb", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn pitb_ftilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 361, 100, 0, 45, 3.5, 0.0, 7.5, 14.0, Some(0.0), Some(7.5), Some(3.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pitb", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn pitb_dtilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 28, 70, 0, 46, 2.5, 0.0, 3.0, 16.0, Some(0.0), Some(5.0), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 0.72);
}

#[acmd_script( agent = "pitb", script = "game_throwlw", category = ACMD_GAME, low_priority )]
unsafe fn pitb_dthrow(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -3, 3);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 50, 80, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 40, 0, 80, 6.0, 0.0, 2.0, -4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pitb", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn pitb_nair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 367, 100, 0, 0, 8.5, 0.0, 9.0, 5.5, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 4.5, 55, 85, 0, 80, 9.5, 0.0, 9.5, 3.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pitb", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn pitb_fair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 11.0);
    for _ in 0..2 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 367, 48, 0, 20, 2.3, 0.0, 5.0, 10.0, Some(0.0), Some(5.0), Some(19.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 80, 110, 0, 35, 4.0, 0.0, 5.0, 11.0, Some(0.0), Some(5.0), Some(19.0), 2.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pitb", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn pitb_dair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 270, 80, 0, 0, 6.0, 0.0, 0.0, -8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 80, 0, 50, 4.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 60, 80, 0, 50, 6.0, 0.0, -4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 60, 80, 0, 50, 6.0, 0.0, 0.0, 8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PALUTENA);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pitb", scripts = ["game_specialnstart", "game_specialairnstart"], category = ACMD_GAME, low_priority )]
unsafe fn pitb_nspecialstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, false, 0);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "pitb", scripts = ["expression_specialnstart", "expression_specialairnstart"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn pitb_nspecialstartexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_final") as i64);
    }
}

#[acmd_script( agent = "pitb", scripts = ["effect_specialnholds", "effect_specialnholdhi"], category = ACMD_EFFECT, low_priority )]
unsafe fn pitb_nspecialholdeff(fighter: &mut L2CAgentBase) {
    for _ in 0..10 {
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

#[acmd_script( agent = "pitb", scripts = ["effect_specialnstos", "effect_specialairnstos", "effect_specialairnholds", "effect_specialairnholdhi", "effect_specialnstos", "effect_specialairnstos", "effect_specialnstohi", "effect_specialairnstohi", "effect_specialnhitos", "effect_specialairnhitos"], category = ACMD_EFFECT, low_priority )]
unsafe fn pitb_nspecialholdsaireff(_fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "pitb", scripts = ["game_specialnfires", "game_specialairnfires", "game_specialnfirehi", "game_specialairnfirehi"], category = ACMD_GAME, low_priority )]
unsafe fn pitb_nspecialfires(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOWARROW, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

#[acmd_script( agent = "pitb", script = "effect_specialnfirehi", category = ACMD_EFFECT, low_priority )]
unsafe fn pitb_nspecialfirehieff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
	    macros::LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_v"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        macros::EFFECT(fighter, Hash40::new("pitb_pa_max_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT(fighter, Hash40::new("pitb_pa_max"), Hash40::new("top"), 0, 14, 9, -22, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "pitb", script = "effect_specialairnfirehi", category = ACMD_EFFECT, low_priority )]
unsafe fn pitb_nspecialairfirehieff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        macros::EFFECT(fighter, Hash40::new("pitb_pa_max_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
		macros::EFFECT(fighter, Hash40::new("pitb_pa_max"), Hash40::new("top"), 0, 14, 9, -22, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "pitb", scripts = ["expression_specialnfires", "expression_specialairnfires", "expression_specialnfirehi", "expression_specialairnfirehi"], category = ACMD_EXPRESSION, low_priority )]
unsafe fn pitb_nspecialfiresexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, 0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, 0);
        }
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_none") as i64);
    }
    frame(fighter.lua_state_agent, 52.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_normal") as i64);
    }
}

#[acmd_script( agent = "pitb", script = "game_specialsend", category = ACMD_GAME, low_priority )]
unsafe fn pitb_sspecialend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 40, 67, 0, 99, 7.0, 0.0, 4.0, 9.0, Some(0.0), Some(10.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
}

#[acmd_script( agent = "pitb", script = "game_specialairsend", category = ACMD_GAME, low_priority )]
unsafe fn pitb_sspecialendair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 40, 58, 0, 90, 7.0, 5.0, 4.0, 9.0, Some(5.0), Some(10.0), Some(9.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: -0.5, y: 2.5, z: 0.0});
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_PIT_REFLECTOR_KIND_SPECIAL_S, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_S);
    }
    frame(fighter.lua_state_agent, 22.0);
    macros::FT_MOTION_RATE(fighter, 0.83);
}

#[acmd_script( agent = "pitb", scripts = ["game_specialhistart", "game_specialairhistart"], category = ACMD_GAME, low_priority )]
unsafe fn pitb_uspecialstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 2.0/3.0);
    frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "pitb", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn pitb_uspecial(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::ATTACK(fighter, 0, 1, Hash40::new("rot"), 1.4, 100, 100, 150, 0, 4.0, 0.0, 0.0, 6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 1, Hash40::new("rot"), 1.4, 100, 100, 150, 0, 4.0, 0.0, 0.0, -6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 1, Hash40::new("rot"), 1.4, 100, 100, 170, 0, 3.5, 0.0, -5.0, 6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 1, Hash40::new("rot"), 1.4, 100, 100, 170, 0, 3.5, 0.0, -5.0, -6.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 1, Hash40::new("rot"), 6.0, 80, 70, 0, 105, 5.0, 0.0, 3.0, 6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 1, Hash40::new("rot"), 6.0, 80, 70, 0, 105, 5.0, 0.0, 3.0, -6.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    macros::FT_MOTION_RATE(fighter, 1.0/11.0);
    frame(fighter.lua_state_agent, 44.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_FIX_ANGLE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_RUSH_FLAG_BACK_ANGLE);
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pitb", script = "game_specialairhiend", category = ACMD_GAME, low_priority )]
unsafe fn pitb_uspecialend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "pitb", scripts = ["game_speciallwhold", "game_specialairlwhold"], category = ACMD_GAME, low_priority )]
unsafe fn pitb_dspecialhold(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, 1, *FIGHTER_PIT_REFLECTOR_GROUP_SPECIAL_LW);
        macros::ATTACK(fighter, 0, 0, Hash40::new_raw(0x10489b2b69), 5.0, 45, 30, 0, 110, 3.0, 0.0, -2.0, 6.0, Some(0.0), Some(6.0), Some(6.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new_raw(0x104ff6ef70), 5.0, 45, 30, 0, 110, 3.0, 0.0, -2.0, -6.0, Some(0.0), Some(6.0), Some(-6.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    install_status_scripts!(
        pitb_specialncharge_main,
        pitb_specialnturn_main,
        pitb_specialnshoot_end
    );
    install_acmd_scripts!(
        pitb_ftilt,
        pitb_dtilt,
        pitb_dthrow,
        pitb_nair,
        pitb_fair,
        pitb_dair,
        pitb_nspecialstart,
        pitb_nspecialstartexp,
        pitb_nspecialholdeff,
        pitb_nspecialholdsaireff,
        pitb_nspecialfires,
        pitb_nspecialfirehieff,
        pitb_nspecialairfirehieff,
        pitb_nspecialfiresexp,
        pitb_sspecialend,
        pitb_sspecialendair,
        pitb_uspecialstart,
        pitb_uspecial,
        pitb_uspecialend,
        pitb_dspecialhold
    );
}