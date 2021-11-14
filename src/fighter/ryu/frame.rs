use {
    smash::{
        lua2cpp::L2CFighterCommon,
        hash40,
        phx::{Hash40, Vector3f, Vector2f},
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*,
    crate::{
        common_funcs::*,
        vars::*,
        gameplay::*
    }
};

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
fn ryu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        // Jump Cancel Heavy Up-Tilt

        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3_s") {
            jump_cancel_check_hit(fighter, false);
        }

        // Reset Vars

        if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
            EX_FOCUS[entry_id(fighter.module_accessor)] = false;
            CANCEL[entry_id(fighter.module_accessor)] = false;
            EX_FLASH[entry_id(fighter.module_accessor)] = false;
            SPECIAL_LW_TIMER[entry_id(fighter.module_accessor)] = -1;
            SECRET_SENSATION[entry_id(fighter.module_accessor)] = false;
            SEC_SEN_STATE[entry_id(fighter.module_accessor)] = false;
            OPPONENT_BOMA[entry_id(fighter.module_accessor)] = 0;
        }

        // EX Focus Attack Check
        if EX_FOCUS[entry_id(fighter.module_accessor)] == false {
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
                EX_FOCUS[entry_id(fighter.module_accessor)] = true;
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
            EX_FOCUS[entry_id(fighter.module_accessor)] = false;
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
                    MotionModule::change_motion(
                        fighter.module_accessor,
                        Hash40::new("appeal_hi_r"),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                }
                else {
                    MotionModule::change_motion(
                        fighter.module_accessor,
                        Hash40::new("appeal_hi_l"),
                        0.0,
                        1.0,
                        false,
                        0.0,
                        false,
                        false
                    );
                }
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }

            if SECRET_SENSATION[entry_id(fighter.module_accessor)] {
                StopModule::end_stop(fighter.module_accessor);
                JostleModule::set_status(fighter.module_accessor, false);
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
                DamageModule::set_damage_lock(fighter.module_accessor, true);
                DamageModule::set_no_reaction_no_effect(fighter.module_accessor, true);
                HitModule::set_hit_stop_mul(fighter.module_accessor, 0.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8 }, 0.0);
                if CAMERA[entry_id(fighter.module_accessor)] == false {
                    macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_exec"));
                    if FighterUtil::get_opponent_fighter_num(fighter.module_accessor, true) < 2 {
                        macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 1.5, 0.0, 0.0);
                    }
                    // macros::SLOW_OPPONENT(fighter, 100.0, 32.0);
                    if OPPONENT_BOMA[entry_id(fighter.module_accessor)] != 0 {
                        SlowModule::set(
                            OPPONENT_BOMA[entry_id(fighter.module_accessor)] as *mut BattleObjectModuleAccessor,
                            0,
                            100,
                            32,
                            false,
                            0x50000000
                        );
                    }
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
                    if FighterUtil::get_opponent_fighter_num(fighter.module_accessor, true) < 2 {
                        macros::CAM_ZOOM_OUT(fighter);
                    }
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
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("ryu_savingattack_aura"), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
                    macros::BURN_COLOR(fighter, 0.0, 0.55, 1.0, 0.7);
                }
                if MotionModule::frame(fighter.module_accessor) <= 30.0
                && MotionModule::frame(fighter.module_accessor) >= 4.0 {
                    smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
                    DamageModule::set_damage_lock(fighter.module_accessor, true);
                }
                else {
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("ryu_savingattack_aura"), false, true);
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
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("ryu_savingattack_aura"), false, true);
                macros::BURN_COLOR_NORMAL(fighter);
                SEC_SEN_STATE[entry_id(fighter.module_accessor)] = false;
                HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            }
        }
    }
}

pub fn install() {
    install_agent_frames!(
        ryu_frame
    );
}