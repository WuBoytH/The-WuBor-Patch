use smash::{
    lua2cpp::L2CAgentBase,
    phx::{Hash40, Vector3f},
    app::{lua_bind::*, sv_animcmd::*, *},
    lib::lua_const::*
};
use smash_script::*;
use smashline::*;
use crate::{
    vars::*
};

// pub static mut GO_SAUCE : [f32; 8] = [0.0; 8];

// #[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
// fn dolly_frame(fighter: &mut L2CFighterCommon) {
//     unsafe {
//         if entry_id(fighter.module_accessor) < 8 {
//
//             if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
//                 GO_SAUCE[entry_id(fighter.module_accessor)] = 0.0;
//             }
//
//             if GO_SAUCE[entry_id(fighter.module_accessor)] >= 50.0 {
//                 WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
//             }
//             else {
//                 WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
//             }
//
//             if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2_BLOW
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_FINAL
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE01
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE02
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE03
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE04
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_FINAL_SCENE05
//             && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_DOLLY_STATUS_KIND_FINAL_END {
//                 if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//                     GO_SAUCE[entry_id(fighter.module_accessor)] += AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false) * 0.8;
//                 }
//                 if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
//                     GO_SAUCE[entry_id(fighter.module_accessor)] += AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false) * 0.4;
//                 }
//                 if GO_SAUCE[entry_id(fighter.module_accessor)] > 100.0 {
//                     GO_SAUCE[entry_id(fighter.module_accessor)] = 100.0;
//                 }
//             }
//
//             DAMAGE_TAKEN[entry_id(fighter.module_accessor)] = DamageModule::damage(fighter.module_accessor, 0);
//             if DAMAGE_TAKEN[entry_id(fighter.module_accessor)] > DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)] {
//                 GO_SAUCE[entry_id(fighter.module_accessor)] += (DAMAGE_TAKEN[entry_id(fighter.module_accessor)] - DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]) * 0.2;
//                 if GO_SAUCE[entry_id(fighter.module_accessor)] > 100.0 {
//                     GO_SAUCE[entry_id(fighter.module_accessor)] = 100.0;
//                 }
//             }
//             DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)] = DAMAGE_TAKEN[entry_id(fighter.module_accessor)];
//
//             if smashball::is_training_mode(){
//                 if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
//                     if GO_SAUCE[entry_id(fighter.module_accessor)] > 50.0 {
//                         GO_SAUCE[entry_id(fighter.module_accessor)] -= 50.0
//                     }
//                     else {
//                         GO_SAUCE[entry_id(fighter.module_accessor)] = 0.0;
//                     }
//                 }
//                 if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
//                     if GO_SAUCE[entry_id(fighter.module_accessor)] < 50.0 {
//                         GO_SAUCE[entry_id(fighter.module_accessor)] += 50.0;
//                     }
//                     else {
//                         GO_SAUCE[entry_id(fighter.module_accessor)] = 100.0;
//                     }
//                 }
//             }
//         }
//     }
// }

#[acmd_script( agent = "dolly", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn dolly_dtilt(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 30, 0, 35, 3.5, 0.0, 5.0, 2.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 80, 30, 0, 35, 3.5, 0.0, 4.0, 7.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 86, 30, 0, 35, 3.5, 0.0, 3.0, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 30, 40, 0, 50, 3.5, 0.0, 5.0, 2.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 30, 40, 0, 50, 3.5, 0.0, 4.0, 7.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 3.0, 30, 40, 0, 50, 3.5, 0.0, 3.0, 12.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 2.0, false);
        // AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 4, -7.0, false);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(fighter, 0.7);
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
}

#[acmd_script( agent = "dolly", script = "game_throwf", category = ACMD_GAME, low_priority )]
unsafe fn dolly_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 313, 15, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 11.0, 1.0);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 10.0, y: 0.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    macros::FT_MOTION_RATE(fighter, 1.0 / 3.0);
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "dolly", script = "effect_throwf", category = ACMD_EFFECT, low_priority )]
unsafe fn dolly_fthroweff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 2, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
	    macros::EFFECT(fighter, Hash40::new("dolly_throw_ground"), Hash40::new("top"), 4, 0, 1, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 14, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    	macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, 14, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "dolly", script = "sound_throwf", category = ACMD_SOUND, low_priority )]
unsafe fn dolly_fthrowsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    	macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_hit_kick_m"));
    }
}

#[acmd_script( agent = "dolly", script = "expression_throwf", category = ACMD_EXPRESSION, low_priority )]
unsafe fn dolly_fthrowexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
	    slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    	ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_R, 8);
    }
    frame(fighter.lua_state_agent, 58.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

#[acmd_script( agent = "dolly", script = "game_throwb", category = ACMD_GAME, low_priority )]
unsafe fn dolly_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 313, 15, 0, 80, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::REVERSE_LR(fighter);
        macros::CHECK_FINISH_CAMERA(fighter, 11.0, 1.0);
        let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut smash::app::FighterCutInManager);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
        lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{x: 10.0, y: 0.0, z: 0.0});
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    macros::FT_MOTION_RATE(fighter, 1.0 / 3.0);
    frame(fighter.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
}

#[acmd_script( agent = "dolly", script = "effect_throwb", category = ACMD_EFFECT, low_priority )]
unsafe fn dolly_bthroweff(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, 1, 0, 180, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
	    macros::EFFECT(fighter, Hash40::new("dolly_throw_ground"), Hash40::new("top"), -4, 0, 1, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), -12, 0, 2, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    	macros::LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 12, -14, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
    }
}

#[acmd_script( agent = "dolly", script = "sound_throwb", category = ACMD_SOUND, low_priority )]
unsafe fn dolly_bthrowsnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    	macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dolly_hit_kick_m"));
    }
}

#[acmd_script( agent = "dolly", script = "expression_throwb", category = ACMD_EXPRESSION, low_priority )]
unsafe fn dolly_bthrowexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        FT_ATTACK_ABS_CAMERA_QUAKE(fighter.lua_state_agent);
	    slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    	ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 8);
    }
    frame(fighter.lua_state_agent, 58.0);
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

#[acmd_script( agent = "dolly", script = "game_specialn", category = ACMD_GAME, low_priority )]
unsafe fn dolly_nspecial(fighter: &mut L2CAgentBase) {
    let mut feint = false;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        macros::FT_MOTION_RATE(fighter, 2.0 / 3.0);
    }
    else {
        macros::FT_MOTION_RATE(fighter, 5.0 / 6.0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        feint = true;
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if feint == false {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
    }
}

#[acmd_script( agent = "dolly", script = "game_specialairn", category = ACMD_GAME, low_priority )]
unsafe fn dolly_nspecialair(fighter: &mut L2CAgentBase) {
    let mut feint = false;
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    macros::FT_MOTION_RATE(fighter, 0.75);
    frame(fighter.lua_state_agent, 12.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        feint = true;
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        if feint == false {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_N_WORK_FLAG_GENERATE);
        }
        else {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f {x: 0.0, y: 1.2, z: 0.0});
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
    }
}

#[acmd_script( agent = "dolly", script = "game_specialairlw", category = ACMD_GAME, low_priority )]
unsafe fn dolly_dspecialair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 0.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.3, y: -1.0, z: 0.0 });
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    else {
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 1.3, y: -1.5, z: 0.0 });
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    frame(fighter.lua_state_agent, 1.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: -0.3, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: -1.0, z: 0.0 });
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 70, 0, 80, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 65, 0, 80, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 75, 0, 80, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 70, 0, 80, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(fighter.lua_state_agent, 2.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.0, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: -0.5, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 4.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 5.0);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    else {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) != *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 310, 95, 0, 30, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
            }
        }
        else {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.2);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 7.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 9.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 10.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 11.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.05, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.0, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 13.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
        if macros::is_excute(fighter) {
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.0, z: 0.0 });
        }
    }
    else {
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            KineticModule::add_speed(fighter.module_accessor, &Vector3f { x: 0.0, y: 0.2, z: 0.0 });
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 7.0, 0.0, 8.0, 5.5, Some(0.0), Some(4.0), Some(3.5), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
            }
        }
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 10.0);
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY);
    }
}

#[acmd_script( agent = "dolly", script = "game_specialairhi1", category = ACMD_GAME, low_priority )]
unsafe fn dolly_uspecialair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 98, 110, 110, 30, 4.0, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 75, 110, 110, 30, 4.0, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 0.0, Some(0.0), Some(4.5), Some(6.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 10, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 6.0, Some(0.0), Some(3.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 3.5, 0.0, Some(0.0), Some(3.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 30, 30, 30, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 10, 4.5, 0.0, 9.0, 3.0, Some(0.0), Some(9.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 3.5, -4.0, Some(0.0), Some(3.5), Some(-6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 25.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 83, 65, 0, 90, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 83, 55, 0, 90, 7.0, 0.0, 7.0, 2.0, Some(0.0), Some(14.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
}

#[acmd_script( agent = "dolly", script = "game_specialairhicommand", category = ACMD_GAME, low_priority )]
unsafe fn dolly_uspecialaircomm(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_REVERSE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_DECIDE_STRENGTH);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_XLU);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_HI_WORK_FLAG_JUMP);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 98, 110, 110, 30, 4.5, 0.0, 10.0, 6.5, Some(0.0), Some(10.0), Some(2.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 75, 110, 110, 30, 4.5, 0.0, 10.0, -2.0, Some(0.0), Some(10.0), Some(-3.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 1, true, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 7.5, 0.0, Some(0.0), Some(7.5), Some(6.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 20.0, 1.0, Some(0.0), Some(22.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 40, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 13.0, 3.0, Some(0.0), Some(13.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 7.5, -4.0, Some(0.0), Some(7.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 6.5, 7.5, Some(0.0), Some(6.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 19.0, 1.0, Some(0.0), Some(21.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 12.0, 3.0, Some(0.0), Some(12.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 6.5, -4.0, Some(0.0), Some(6.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 7.5, Some(0.0), Some(5.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 5.5, 0.0, Some(0.0), Some(5.5), Some(6.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 18.0, 1.0, Some(0.0), Some(20.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 11.0, 3.0, Some(0.0), Some(11.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 5.5, -4.0, Some(0.0), Some(5.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.5, 0.0, 4.5, 7.5, Some(0.0), Some(4.5), Some(0.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 17.0, 1.0, Some(0.0), Some(19.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 35, 30, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 45, 40, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.3, 367, 30, 50, 30, 3.0, 0.0, 4.5, -4.0, Some(0.0), Some(4.5), Some(-4.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        macros::HIT_NODE(fighter, Hash40::new("hip"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.3, 90, 30, 50, 30, 3.5, 0.0, 3.5, 5.0, Some(0.0), Some(3.5), Some(-7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.3, 70, 50, 5, 20, 3.5, 0.0, 16.0, 1.0, Some(0.0), Some(18.5), Some(1.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 40, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.3, 90, 50, 45, 20, 4.5, 0.0, 10.0, 3.0, Some(0.0), Some(10.0), Some(-1.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 34.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 83, 73, 0, 90, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 83, 62, 0, 90, 7.0, 0.0, 8.0, 2.0, Some(0.0), Some(15.0), Some(2.0), 2.0, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BODY);
        }
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
}

// #[acmd_script( agent = "dolly", script = "game_superspecial", category = ACMD_GAME, low_priority )]
// unsafe fn dolly_geezah(fighter: &mut L2CAgentBase) {
//     GO_SAUCE[entry_id(fighter.module_accessor)] -= 50.0;
//     original!(fighter);
// }

// #[acmd_script( agent = "dolly", script = "game_superspecial2start", category = ACMD_GAME, low_priority )]
// unsafe fn dolly_bustah(fighter: &mut L2CAgentBase) {
//     GO_SAUCE[entry_id(fighter.module_accessor)] -= 50.0;
//     original!(fighter);
// }

#[acmd_script( agent = "dolly_wave", script = "game_normalw", category = ACMD_GAME, low_priority )]
unsafe fn dolly_wave_lightground(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 55, 20, 0, 50, 2.4, 0.0, 12.0, -5.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0, 55, 20, 0, 50, 1.8, 0.0, 2.0, -10.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 55, 20, 0, 50, 2.4, 0.0, 9.0, -4.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0, 55, 20, 0, 50, 1.8, 0.0, 2.0, -8.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(weapon.lua_state_agent, 6.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 55, 20, 0, 50, 2.4, 0.0, 6.5, -2.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0, 55, 20, 0, 50, 1.8, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script( agent = "dolly_wave", script = "game_normal", category = ACMD_GAME, low_priority )]
unsafe fn dolly_wave_heavyground(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.0, 55, 20, 0, 50, 2.4, 0.0, 12.0, -5.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 9.0, 55, 20, 0, 50, 1.8, 0.0, 2.0, -10.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(weapon.lua_state_agent, 2.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.0, 55, 20, 0, 50, 2.4, 0.0, 9.0, -4.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 9.0, 55, 20, 0, 50, 1.8, 0.0, 2.0, -8.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    frame(weapon.lua_state_agent, 6.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 9.0, 55, 20, 0, 50, 2.4, 0.0, 6.5, -2.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 9.0, 55, 20, 0, 50, 1.8, 0.0, 2.0, -6.0, Some(0.0), Some(2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

pub fn install() {
    // install_agent_frames!(
    //     dolly_frame
    // );
    install_acmd_scripts!(
        dolly_dtilt,
        dolly_fthrow,
        dolly_fthroweff,
        dolly_fthrowsnd,
        dolly_fthrowexp,
        dolly_bthrow,
        dolly_bthroweff,
        dolly_bthrowsnd,
        dolly_bthrowexp,
        dolly_nspecial,
        dolly_nspecialair,
        dolly_dspecialair,
        dolly_uspecialair,
        dolly_uspecialaircomm,
        // dolly_geezah,
        // dolly_bustah,
        dolly_wave_lightground,
        dolly_wave_heavyground
    );
}