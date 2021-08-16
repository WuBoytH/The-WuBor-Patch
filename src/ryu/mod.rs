use smash::phx::Hash40;
use smash::hash40;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::*;
use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use crate::commonfuncs::*;
use crate::gameplay::*;
use crate::vars::*;

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
fn ryu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Jump Cancel Heavy Up-Tilt

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3_s") {
            jump_cancel_check(fighter.module_accessor);
        }

        // Reset Vars

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
            SPECIAL_LW[entry_id(fighter.module_accessor)] = false;
            CANCEL[entry_id(fighter.module_accessor)] = false;
            EX_FLASH[entry_id(fighter.module_accessor)] = false;
            SPECIAL_LW_TIMER[entry_id(fighter.module_accessor)] = -1;
            SECRET_SENSATION[entry_id(fighter.module_accessor)] = false;
            SEC_SEN_STATE[entry_id(fighter.module_accessor)] = false;
            OPPONENT_BOMA[entry_id(fighter.module_accessor)] = 0;
        }

        // EX Focus Attack Check
        if SPECIAL_LW[entry_id(fighter.module_accessor)] == false {
            if (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_n")
            && MotionModule::frame(fighter.module_accessor) > 13.0)
            || (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s_start") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_s")
            && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)))
            || (MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi") || MotionModule::motion_kind(fighter.module_accessor) == hash40("special_hi_command")
            && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND)
            && CANCEL[entry_id(fighter.module_accessor)] == false {
                CANCEL[entry_id(fighter.module_accessor)] = true;
            }
            else if CANCEL[entry_id(fighter.module_accessor)]
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                CANCEL[entry_id(fighter.module_accessor)] = false;
            }
        }

        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
        && CANCEL[entry_id(fighter.module_accessor)] {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
        }

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_LW
        && CANCEL[entry_id(fighter.module_accessor)] {
            EX_FLASH[entry_id(fighter.module_accessor)] = true;
            _TIME_COUNTER[entry_id(fighter.module_accessor)] = -1;
            if !IS_FUNNY[entry_id(fighter.module_accessor)] {
                SPECIAL_LW_TIMER[entry_id(fighter.module_accessor)] = 1200;
                SPECIAL_LW[entry_id(fighter.module_accessor)] = true;
            }
            CANCEL[entry_id(fighter.module_accessor)] = false;
        }

        if SPECIAL_LW_TIMER[entry_id(fighter.module_accessor)] > 0 {
            SPECIAL_LW_TIMER[entry_id(fighter.module_accessor)] = SPECIAL_LW_TIMER[entry_id(fighter.module_accessor)] - 1;
        }
        else if SPECIAL_LW_TIMER[entry_id(fighter.module_accessor)] == 0 {
            SPECIAL_LW_TIMER[entry_id(fighter.module_accessor)] = -1;
            let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
            let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
            let focuseff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
            EffectModule::set_rgb(fighter.module_accessor, focuseff, 0.0, 0.0, 0.0);
            SPECIAL_LW[entry_id(fighter.module_accessor)] = false;
        }

        // EX Flash

        if EX_FLASH[entry_id(fighter.module_accessor)] {
            if SEC_SEN_STATE[entry_id(fighter.module_accessor)] {
                if _TIME_COUNTER[entry_id(fighter.module_accessor)] < 0 {
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] = 8;
                }
                if _TIME_COUNTER[entry_id(fighter.module_accessor)] <= 4 {
                    macros::COL_NORMAL(fighter);
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
                }
                if _TIME_COUNTER[entry_id(fighter.module_accessor)] > 4 {
                    macros::FLASH(fighter, 0, 0.55, 1, 1.0);
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
                }
            }
            else {
                if _TIME_COUNTER[entry_id(fighter.module_accessor)] < 0 {
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] = 12;
                }
                else if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                    macros::COL_NORMAL(fighter);
                    EX_FLASH[entry_id(fighter.module_accessor)] = false;
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
                }
                else {
                    macros::FLASH(fighter, 1, 1, 0.0, 0.75);
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
                }
            }
        }

        // Tatsu in da air

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_LOOP {
            if SPECIAL_S_START_SIT[entry_id(fighter.module_accessor)] == 1
            && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
                fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            }
        }

        // Secret Sensation Code

        if IS_FUNNY[entry_id(fighter.module_accessor)]
        || DamageModule::damage(fighter.module_accessor, 0) >= 200.0 {

            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
            && (StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S3
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI3
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW3
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_S4
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_LW4
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_HI4
            || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_ATTACK_DASH)
            && (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD)) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                if PostureModule::lr(fighter.module_accessor) == 1.0 {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_hi_l"), 0.0, 1.0, false, 0.0, false, false);
                }
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }

            if SECRET_SENSATION[entry_id(fighter.module_accessor)] {
                StopModule::end_stop(fighter.module_accessor);
                JostleModule::set_status(fighter.module_accessor, false);
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
                DamageModule::set_damage_lock(fighter.module_accessor, true);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                if CAMERA[entry_id(fighter.module_accessor)] == false {
                    macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_exec"));
                    macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 1.5, 0.0, 0.0);
                    macros::SLOW_OPPONENT(fighter, 100.0, 32.0);
                    SlowModule::set_whole(fighter.module_accessor, 4, 0);
                    macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
                    RYU_X[entry_id(fighter.module_accessor)] = PostureModule::pos_x(fighter.module_accessor);
                    RYU_Y[entry_id(fighter.module_accessor)] = PostureModule::pos_y(fighter.module_accessor);
                    if RYU_X[entry_id(fighter.module_accessor)] == OPPONENT_X[entry_id(fighter.module_accessor)] {
                        OPPONENT_DIRECTION[entry_id(fighter.module_accessor)] = 12.0 * PostureModule::lr(fighter.module_accessor);
                        SEC_SEN_DIREC[entry_id(fighter.module_accessor)] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                    }
                    else if RYU_X[entry_id(fighter.module_accessor)] < OPPONENT_X[entry_id(fighter.module_accessor)] {
                        OPPONENT_DIRECTION[entry_id(fighter.module_accessor)] = 12.0;
                        if PostureModule::lr(fighter.module_accessor) == -1.0 {
                            PostureModule::reverse_lr(fighter.module_accessor);
                        }
                        SEC_SEN_DIREC[entry_id(fighter.module_accessor)] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                    }
                    else {
                        OPPONENT_DIRECTION[entry_id(fighter.module_accessor)] = -12.0;
                        if PostureModule::lr(fighter.module_accessor) == 1.0 {
                            PostureModule::reverse_lr(fighter.module_accessor);
                        }
                        SEC_SEN_DIREC[entry_id(fighter.module_accessor)] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                    }
                    if OPPONENT_BOMA[entry_id(fighter.module_accessor)] != 0 {
                        if (RYU_Y[entry_id(fighter.module_accessor)] - OPPONENT_Y[entry_id(fighter.module_accessor)]).abs() <= 12.0
                        && StatusModule::situation_kind(OPPONENT_BOMA[entry_id(fighter.module_accessor)] as *mut BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                            VERT_EXTRA[entry_id(fighter.module_accessor)] = 0.0;
                        }
                    }
                    else {
                        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                        VERT_EXTRA[entry_id(fighter.module_accessor)] = 12.0;
                        RYU_Y[entry_id(fighter.module_accessor)] += 2.0;
                        PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                            x: 0.0,
                            y: 2.0
                        });
                    }
                    CAMERA[entry_id(fighter.module_accessor)] = true;
                }
                if SEC_SEN_TIMER[entry_id(fighter.module_accessor)] >= 0.0 {
                    if OPPONENT_BOMA[entry_id(fighter.module_accessor)] != 0 {
                        if (RYU_Y[entry_id(fighter.module_accessor)] - OPPONENT_Y[entry_id(fighter.module_accessor)]).abs() <= 12.0
                        && StatusModule::situation_kind(OPPONENT_BOMA[entry_id(fighter.module_accessor)] as *mut BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                        }
                    }
                    if StatusModule::status_kind(fighter.module_accessor) != SEC_SEN_DIREC[entry_id(fighter.module_accessor)] {
                        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
                        StatusModule::change_status_request_from_script(fighter.module_accessor, SEC_SEN_DIREC[entry_id(fighter.module_accessor)], true);
                    }
                    if (RYU_Y[entry_id(fighter.module_accessor)] - OPPONENT_Y[entry_id(fighter.module_accessor)]).abs() > 12.0 {
                        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                    }
                    PostureModule::set_pos_2d(fighter.module_accessor, &Vector2f{
                        x: (((OPPONENT_X[entry_id(fighter.module_accessor)] + OPPONENT_DIRECTION[entry_id(fighter.module_accessor)]) * SEC_SEN_TIMER[entry_id(fighter.module_accessor)]) + RYU_X[entry_id(fighter.module_accessor)] * (1.0 - SEC_SEN_TIMER[entry_id(fighter.module_accessor)])),
                        y: (((OPPONENT_Y[entry_id(fighter.module_accessor)] + VERT_EXTRA[entry_id(fighter.module_accessor)]) * SEC_SEN_TIMER[entry_id(fighter.module_accessor)]) + RYU_Y[entry_id(fighter.module_accessor)] * (1.0 - SEC_SEN_TIMER[entry_id(fighter.module_accessor)]))
                    });
                }
                SEC_SEN_TIMER[entry_id(fighter.module_accessor)] += 0.08;
                if SEC_SEN_TIMER[entry_id(fighter.module_accessor)] > 1.0 {
                    SECRET_SENSATION[entry_id(fighter.module_accessor)] = false;
                    CAMERA[entry_id(fighter.module_accessor)] = false;
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                    macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_TURN_AUTO, true);
                    }
                    macros::CAM_ZOOM_OUT(fighter);
                    macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
                    SlowModule::clear_whole(fighter.module_accessor);
                    JostleModule::set_status(fighter.module_accessor, true);
                    SEC_SEN_TIMER[entry_id(fighter.module_accessor)] = -0.6;
                    OPPONENT_BOMA[entry_id(fighter.module_accessor)] = 0;
                }
            }
            else if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r")
            || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") {
                if MotionModule::frame(fighter.module_accessor) == 4.0 {
                    macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_aura"));
                    SEC_SEN_STATE[entry_id(fighter.module_accessor)] = true;
                    EX_FLASH[entry_id(fighter.module_accessor)] = true;
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] = -1;
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
                    macros::BURN_COLOR(fighter, 0.0, 0.55, 1.0, 0.7);
                }
                if MotionModule::frame(fighter.module_accessor) <= 30.0
                && MotionModule::frame(fighter.module_accessor) >= 4.0 {
                    smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
                    DamageModule::set_damage_lock(fighter.module_accessor, true);
                }
                else {
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x15db57d7a6), false, true);
                    macros::BURN_COLOR_NORMAL(fighter);
                    smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
                    DamageModule::set_damage_lock(fighter.module_accessor, false);
                    EX_FLASH[entry_id(fighter.module_accessor)] = false;
                    macros::COL_NORMAL(fighter);
                    SEC_SEN_STATE[entry_id(fighter.module_accessor)] = false;
                    HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
                }
            }
            else if SECRET_SENSATION[entry_id(fighter.module_accessor)] == false
            && SEC_SEN_STATE[entry_id(fighter.module_accessor)] {
                DamageModule::set_damage_lock(fighter.module_accessor, false);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, false);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 1.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                EX_FLASH[entry_id(fighter.module_accessor)] = false;
                macros::COL_NORMAL(fighter);
                macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x15db57d7a6), false, true);
                macros::BURN_COLOR_NORMAL(fighter);
                SEC_SEN_STATE[entry_id(fighter.module_accessor)] = false;
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            }
        }
    }
}

#[acmd_script( agent = "ryu", script = "game_specialsstart", category = ACMD_GAME, low_priority )]
unsafe fn ryu_sspecialstart(fighter: &mut L2CAgentBase) {
    SPECIAL_S_START_SIT[entry_id(fighter.module_accessor)] = 0;
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 75, 70, 0, 85, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ryu", script = "game_specialairsstart", category = ACMD_GAME, low_priority )]
unsafe fn ryu_sspecialstartair(fighter: &mut L2CAgentBase) {
    SPECIAL_S_START_SIT[entry_id(fighter.module_accessor)] = 0;
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 1.0, 3.5, 8.5, 8.5);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 75, 70, 0, 85, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RYU_KICK, *ATTACK_REGION_KICK);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_FLAG_COMMAND) {
            SPECIAL_S_START_SIT[entry_id(fighter.module_accessor)] = 1;
            let speedx = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * PostureModule::lr(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
            macros::SET_SPEED_EX(fighter, speedx, -1.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        }
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.1);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movew", category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadokenw(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.0, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 7.0, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 7.0, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(weapon, 0, 1, 2, 2.2);
    }
    wait(weapon.lua_state_agent, 7.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.0, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 7.0, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 7.0, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(weapon, 0, 1, 2, 2.2);
    }
}

#[acmd_script( agent = "ryu_hadoken", script = "game_movem", category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadokenm(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.5, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 7.5, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 7.5, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(weapon, 0, 1, 2, 2.2);
    }
    wait(weapon.lua_state_agent, 7.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.5, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 7.5, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 7.5, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(weapon, 0, 1, 2, 2.2);
    }
}

#[acmd_script( agent = "ryu_hadoken", script = "game_moves", category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadokens(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0, 0, 10, 0, 68, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 8.0, 60, 10, 0, 67, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(weapon, 0, 1, 2, 2.2);
    }
    wait(weapon.lua_state_agent, 7.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 8.0, 0, 10, 0, 58, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 1, 0, Hash40::new("top"), 8.0, 0, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(weapon, 2, 0, Hash40::new("top"), 8.0, 60, 10, 0, 58, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_RYU_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(weapon, 0, 1, 2, 2.2);
    }
}

#[acmd_script( agent = "ryu_hadoken", scripts = ["game_movespw_last", "game_movespm_last", "game_movesps_last"], category = ACMD_GAME, low_priority )]
unsafe fn ryu_hadoken_shaku_end(weapon: &mut L2CAgentBase) {
    if macros::is_excute(weapon) {
        AttackModule::clear_all(weapon.module_accessor);
    }
    frame(weapon.lua_state_agent, 3.0);
    if macros::is_excute(weapon) {
        macros::ATTACK(weapon, 0, 0, Hash40::new("top"), 7.3, 55, 60, 0, 60, 5.0, 0.0, 0.0, -1.0, None, None, None, 1.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATK_SET_SHIELD_SETOFF_MUL_arg4(weapon, 0, 1, 2, 2.5);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        ryu_frame
    );
    smashline::install_acmd_scripts!(
        ryu_sspecialstart,
        ryu_sspecialstartair,
        ryu_hadokenw,
        ryu_hadokenm,
        ryu_hadokens,
        ryu_hadoken_shaku_end
    );
}