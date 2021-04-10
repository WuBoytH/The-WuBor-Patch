use smash::phx::Hash40;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash::app::sv_animcmd;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
// use smash::phx::Vector3f;
// use smash::phx::Vector2f;
// use smash::app::lua_bind::EffectModule;
// use crate::IS_FUNNY;
use crate::commonfuncs::*;

pub static mut QUICK_STEP_STATE : [i32; 8] = [0; 8];
/*
State list:
0 = Can Quick Step
2 = Cannot Quick Step
1 = Used to show you're in the first 22 frames of Quick Step.
*/
static mut VT1_CANCEL : [bool; 8] = [false; 8];
static mut EX_FLASH : [bool; 8] = [false; 8];
pub static mut V_SHIFT : [bool; 8] = [false; 8];

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
unsafe fn ken_frame(fighter: &mut L2CFighterCommon) {
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    
    if get_player_number(boma) < 8 {

        // Reset Vars

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || smash::app::sv_information::is_ready_go() == false {
            QUICK_STEP_STATE[get_player_number(boma)] = 0;
            VT1_CANCEL[get_player_number(boma)] = false;
            EX_FLASH[get_player_number(boma)] = false;
        }

        // V Skill 1
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND1
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_ATTACK_COMMAND2
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_ATTACK_NEAR
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_S3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_LW3
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_S4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_LW4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI4
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_DASH
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_AIR {
            if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD) {
                VT1_CANCEL[get_player_number(boma)] = true;
            }
            else {
                VT1_CANCEL[get_player_number(boma)] = false;
            }
        }
        else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_N
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_S
        || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_AIR_HI_END
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F
        || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B {
            VT1_CANCEL[get_player_number(boma)] = false;
        }
        else {
            VT1_CANCEL[get_player_number(boma)] = true;
        }

        if MotionModule::motion_kind(boma) == smash::hash40("special_lw_step_f")
        && MotionModule::frame(boma) == 1.0 {
            MotionModule::change_motion(boma, Hash40::new("run"), 0.0, 1.0, true, 0.0, false, false);
        }

        if MotionModule::motion_kind(boma) == smash::hash40("run")
        && QUICK_STEP_STATE[get_player_number(boma)] == 1 {
            if MotionModule::frame(boma) >= 22.0 && MotionModule::frame(boma) <= 23.0
            && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                MotionModule::change_motion(boma, Hash40::new("attack_s3_s_w"), 0.0, 1.0, false, 0.0, false, false);
            }
            if MotionModule::frame(boma) >= 31.0 {
                CancelModule::enable_cancel(boma);
            }
        }

        if MotionModule::motion_kind(boma) == smash::hash40("attack_s3_s_w") {
            if MotionModule::frame(boma) > 26.0 {
                CancelModule::enable_cancel(boma);
            }
        }

        if (StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_RUN
        && StatusModule::status_kind(boma) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F)
        && MotionModule::motion_kind(boma) != smash::hash40("attack_s3_s_w")
        && QUICK_STEP_STATE[get_player_number(boma)] == 1 {
            QUICK_STEP_STATE[get_player_number(boma)] = 2;
        }

        if ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && VT1_CANCEL[get_player_number(boma)]
        && QUICK_STEP_STATE[get_player_number(boma)] == 0 {
            if MotionModule::motion_kind(boma) == smash::hash40("attack_air_b") {
                PostureModule::reverse_lr(boma);
            }
            fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F.into(), false.into());
            if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                QUICK_STEP_STATE[get_player_number(boma)] = 1;
            }
            else {
                QUICK_STEP_STATE[get_player_number(boma)] = 2;
            }
        }

        if QUICK_STEP_STATE[get_player_number(boma)] == 2
        && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
            QUICK_STEP_STATE[get_player_number(boma)] = 0;
        }

        // V Shift

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_GUARD {
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                let stick_x = ControlModule::get_stick_x(boma);
                if (stick_x < -0.5 && PostureModule::lr(boma) == 1.0)
                || (stick_x > 0.5 && PostureModule::lr(boma) == -1.0) {
                    fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), false.into());
                }
            }
        }

        if MotionModule::motion_kind(boma) == smash::hash40("special_lw_step_b") {
            if MotionModule::frame(boma) <= 1.0
            && V_SHIFT[get_player_number(boma)] == false {
                macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
                macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
            }
            // if MotionModule::frame(boma) <= 5.625 {
            //     CaptureModule::set_ignore_catching(boma, true);
            // }
            // else {
            //     CaptureModule::set_ignore_catching(boma, false);
            // }
            if MotionModule::frame(boma) == 6.25 {
                if V_SHIFT[get_player_number(boma)] {
                    SlowModule::set_whole(boma, 5, 0);
                    macros::SLOW_OPPONENT(fighter, 10.0, 2.0);
                    macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
                }
            }
            if MotionModule::frame(boma) == 12.5 {
                SlowModule::clear_whole(boma);
                if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40::new("special_lw"), 0.0, 1.0, false, 0.0, false, false);
                }
                else if V_SHIFT[get_player_number(boma)] {
                    macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
                    V_SHIFT[get_player_number(boma)] = false;
                }
            }
        }
        
        if MotionModule::motion_kind(boma) != smash::hash40("special_lw_step_b")
        && MotionModule::motion_kind(boma) != smash::hash40("special_lw") {
            V_SHIFT[get_player_number(boma)] = false;
        }

        // EX Flash

        // if EX_FLASH[get_player_number(boma)] {
        //     if SEC_SEN_STATE[get_player_number(boma)] {
        //         if FLASH_TIMER[get_player_number(boma)] < 0 {
        //             FLASH_TIMER[get_player_number(boma)] = 8;
        //         }
        //         if FLASH_TIMER[get_player_number(boma)] <= 4 {
        //             macros::COL_NORMAL(fighter);
        //             FLASH_TIMER[get_player_number(boma)] -= 1;
        //         }
        //         if FLASH_TIMER[get_player_number(boma)] > 4 {
        //             macros::FLASH(fighter, 0, 0.55, 1, 1.75);
        //             FLASH_TIMER[get_player_number(boma)] -= 1;
        //         }
        //     }
        //     else {
        //         if FLASH_TIMER[get_player_number(boma)] < 0 {
        //             FLASH_TIMER[get_player_number(boma)] = 12;
        //         }
        //         else if FLASH_TIMER[get_player_number(boma)] == 0 {
        //             macros::COL_NORMAL(fighter);
        //             EX_FLASH[get_player_number(boma)] = false;
        //             FLASH_TIMER[get_player_number(boma)] -= 1;
        //         }
        //         else {
        //             macros::FLASH(fighter, 1, 1, 0.0, 1.5);
        //             FLASH_TIMER[get_player_number(boma)] -= 1;
        //         }
        //     }
        // }
    }
}

// Motion Rate the Run Animation

#[script( agent = "ken", script = "game_run", category = ACMD_GAME )]
unsafe fn ken_run(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if QUICK_STEP_STATE[get_player_number(boma)] == 1 {
        macros::FT_MOTION_RATE(fighter, 0.7);
    }
}

// V Shift Related

#[script( agent = "ken", script = "game_speciallwstepb", category = ACMD_GAME )]
unsafe fn ken_dspecialstepb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        DamageModule::set_damage_lock(boma, true);
        // HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    macros::FT_MOTION_RATE(fighter, 1.6);
    sv_animcmd::frame(lua_state, 5.625);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        DamageModule::set_damage_lock(boma, false);
    }
    if V_SHIFT[get_player_number(boma)] {
        if macros::is_excute(fighter) {
            HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
        }
    }
}

#[script( agent = "ken", script = "game_speciallw", category = ACMD_GAME )]
unsafe fn ken_dspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        if V_SHIFT[get_player_number(boma)] {
            macros::SLOW_OPPONENT(fighter, 100.0, 18.0);
        }
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 10.0, 50, 98, 100, 0, 3.2, -1.5, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 10.0, 50, 98, 100, 0, 3.2, -6.2, -1.0, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 10.0, 50, 98, 100, 0, 3.9, 4.3, -1.7, -1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_whole(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
        if V_SHIFT[get_player_number(boma)] {
            macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
            V_SHIFT[get_player_number(boma)] = false;
        }
    }
}

#[script( agent = "ken", script = "sound_speciallw", category = ACMD_SOUND )]
unsafe fn ken_dspecialsnd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ken_smash_s01"));
        macros::PLAY_SE(fighter, Hash40::new("vc_ken_attack09"));
    }
    sv_animcmd::frame(lua_state, 38.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ken_step_left_m"));
    }
}

#[script( agent = "ken", script = "expression_speciallw", category = ACMD_EXPRESSION )]
unsafe fn ken_dspecialxp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        smash_script::slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        smash_script::slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(boma, false, 0);
    }
    sv_animcmd::frame(lua_state, 13.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_nohit1"), 0, false, 0);
        macros::AREA_WIND_2ND_arg10(fighter, 0, 0.8, 180, 8, 0.8, -10, 7, 20, 14, 80);
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attack1"), 0);
    }
    sv_animcmd::frame(lua_state, 28.0);
    if macros::is_excute(fighter) {
        AreaModule::erase_wind(boma, 0);
    }
}

#[script( agent = "ken", script = "effect_speciallw", category = ACMD_EFFECT )]
unsafe fn ken_dspecialeff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x0f998ceac2), Hash40::new_raw(0x0f998ceac2), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, true, *EF_FLIP_YZ);
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 14, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
}

// Make Quick Step (non-prox light f tilt) have step kick properties

// #[script( agent = "ken", script = "game_attacks3w", category = ACMD_GAME )]
// unsafe fn ken_ftiltwnp(fighter: &mut L2CAgentBase) {
//     let lua_state = fighter.lua_state_agent;
//     let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
//     if macros::is_excute(fighter) {
//         WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
//     }
//     sv_animcmd::frame(lua_state, 1.0);
//     macros::FT_MOTION_RATE(fighter, 1.0);
//     sv_animcmd::frame(lua_state, 4.0);
//     if macros::is_excute(fighter) {
//         macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
//         macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
//     }
//     sv_animcmd::frame(lua_state, 8.0);
//     if macros::is_excute(fighter) {
//         macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 55, 46, 0, 66, 3.8, 0.0, 11.0, 6.5, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
//         macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.8, 72, 46, 0, 66, 3.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
//     }
//     sv_animcmd::frame(lua_state, 12.0);
//     if macros::is_excute(fighter) {
//         AttackModule::clear_all(boma);
//         WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
//         HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
//     }
//     macros::FT_MOTION_RATE(fighter, 0.8);
// }

pub fn install() {
    smash_script::replace_fighter_frames!(
        ken_frame
    );
    smash_script::replace_scripts!(
        ken_run,
        ken_dspecialstepb,
        ken_dspecial,
        ken_dspecialsnd,
        ken_dspecialxp,
        ken_dspecialeff,
    //     ken_ftiltwnp
    );
}