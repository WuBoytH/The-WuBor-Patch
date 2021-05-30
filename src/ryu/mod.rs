use smash::phx::Hash40;
use smash::hash40;
use smash::lua2cpp::L2CFighterCommon;
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use crate::{IS_FUNNY, _TIME_COUNTER, OPPONENT_BOMA};
use crate::commonfuncs::*;

static mut SPECIAL_LW : [bool; 8] = [false; 8];
static mut CANCEL : [bool; 8] = [false; 8];
static mut EX_FLASH : [bool; 8] = [false; 8];
pub static mut SECRET_SENSATION : [bool; 8] = [false; 8];
pub static mut CAMERA : [bool; 8] = [false; 8];
pub static mut OPPONENT_X : [f32; 8] = [0.0; 8];
pub static mut OPPONENT_Y : [f32; 8] = [0.0; 8];
static mut RYU_X : [f32; 8] = [0.0; 8];
static mut RYU_Y : [f32; 8] = [0.0; 8];
static mut SPECIAL_LW_TIMER : [i16; 8] = [-1; 8];
static mut SEC_SEN_TIMER : [f32; 8] = [-0.6; 8];
static mut OPPONENT_DIRECTION : [f32; 8] = [12.0; 8];
static mut VERT_EXTRA : [f32; 8] = [12.0; 8];
pub static mut SEC_SEN_STATE : [bool; 8] = [false; 8];
static mut SEC_SEN_DIREC : [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_RYU )]
fn ryu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        if get_player_number(boma) < 8 {

            // Jump Cancel Heavy Up-Tilt

            if MotionModule::motion_kind(boma) == hash40("attack_hi3_s") {
                if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
                    if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }

            // Reset Vars

            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
                SPECIAL_LW[get_player_number(boma)] = false;
                CANCEL[get_player_number(boma)] = false;
                EX_FLASH[get_player_number(boma)] = false;
                SPECIAL_LW_TIMER[get_player_number(boma)] = -1;
                SECRET_SENSATION[get_player_number(boma)] = false;
                SEC_SEN_STATE[get_player_number(boma)] = false;
                OPPONENT_BOMA[get_player_number(boma)] = 0;
            }

            // EX Focus Attack Check
            if SPECIAL_LW[get_player_number(boma)] == false {
                if (MotionModule::motion_kind(boma) == hash40("special_n")
                && MotionModule::frame(boma) > 13.0)
                || (MotionModule::motion_kind(boma) == hash40("special_s_start") || MotionModule::motion_kind(boma) == hash40("special_s")
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)))
                || (MotionModule::motion_kind(boma) == hash40("special_hi") || MotionModule::motion_kind(boma) == hash40("special_hi_command")
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
                && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND)
                && CANCEL[get_player_number(boma)] == false {
                    CANCEL[get_player_number(boma)] = true;
                }
                else if CANCEL[get_player_number(boma)]
                && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    CANCEL[get_player_number(boma)] = false;
                }
            }

            if ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0
            && CANCEL[get_player_number(boma)] {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_SPECIAL_LW, true);
            }

            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW
            && CANCEL[get_player_number(boma)] {
                EX_FLASH[get_player_number(boma)] = true;
                _TIME_COUNTER[get_player_number(boma)] = -1;
                if !IS_FUNNY[get_player_number(boma)] {
                    SPECIAL_LW_TIMER[get_player_number(boma)] = 1200;
                    SPECIAL_LW[get_player_number(boma)] = true;
                }
                CANCEL[get_player_number(boma)] = false;
            }

            if SPECIAL_LW_TIMER[get_player_number(boma)] > 0 {
                SPECIAL_LW_TIMER[get_player_number(boma)] = SPECIAL_LW_TIMER[get_player_number(boma)] - 1;
            }
            else if SPECIAL_LW_TIMER[get_player_number(boma)] == 0 {
                SPECIAL_LW_TIMER[get_player_number(boma)] = -1;
                let pos: Vector3f = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                let rot: Vector3f = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                let focuseff: u32 = EffectModule::req_follow(boma, Hash40::new("sys_counter_flash"), Hash40::new("top"), &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                EffectModule::set_rgb(boma, focuseff, 0.0, 0.0, 0.0);
                SPECIAL_LW[get_player_number(boma)] = false;
            }

            // EX Flash

            if EX_FLASH[get_player_number(boma)] {
                if SEC_SEN_STATE[get_player_number(boma)] {
                    if _TIME_COUNTER[get_player_number(boma)] < 0 {
                        _TIME_COUNTER[get_player_number(boma)] = 8;
                    }
                    if _TIME_COUNTER[get_player_number(boma)] <= 4 {
                        macros::COL_NORMAL(fighter);
                        _TIME_COUNTER[get_player_number(boma)] -= 1;
                    }
                    if _TIME_COUNTER[get_player_number(boma)] > 4 {
                        macros::FLASH(fighter, 0, 0.55, 1, 1.0);
                        _TIME_COUNTER[get_player_number(boma)] -= 1;
                    }
                }
                else {
                    if _TIME_COUNTER[get_player_number(boma)] < 0 {
                        _TIME_COUNTER[get_player_number(boma)] = 12;
                    }
                    else if _TIME_COUNTER[get_player_number(boma)] == 0 {
                        macros::COL_NORMAL(fighter);
                        EX_FLASH[get_player_number(boma)] = false;
                        _TIME_COUNTER[get_player_number(boma)] -= 1;
                    }
                    else {
                        macros::FLASH(fighter, 1, 1, 0.0, 0.75);
                        _TIME_COUNTER[get_player_number(boma)] -= 1;
                    }
                }
            }

            // Secret Sensation Code

            if IS_FUNNY[get_player_number(boma)] {

                if ControlModule::check_button_trigger(boma, *CONTROL_PAD_BUTTON_APPEAL_HI)
                && (StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK
                || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_S3
                || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI3
                || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_LW3
                || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_S4
                || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_LW4
                || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_HI4
                || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_ATTACK_DASH)
                && (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD)) {
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_RESET);
                    fighter.change_status(FIGHTER_STATUS_KIND_APPEAL.into(), false.into());
                    if PostureModule::lr(boma) == 1.0 {
                        MotionModule::change_motion(boma, Hash40::new("appeal_hi_r"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    else {
                        MotionModule::change_motion(boma, Hash40::new("appeal_hi_l"), 0.0, 1.0, false, 0.0, false, false);
                    }
                    GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                }

                if SECRET_SENSATION[get_player_number(boma)] {
                    StopModule::end_stop(boma);
                    JostleModule::set_status(boma, false);
                    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
                    DamageModule::set_damage_lock(boma, true);
                    DamageModule::set_no_reaction_no_effect(boma, true);
                    HitModule::set_hit_stop_mul(boma, 0.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                    if CAMERA[get_player_number(boma)] == false {
                        macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_exec"));
                        macros::CAM_ZOOM_IN_arg5(fighter, 5.0, 0.0, 1.5, 0.0, 0.0);
                        macros::SLOW_OPPONENT(fighter, 100.0, 32.0);
                        SlowModule::set_whole(boma, 4, 0);
                        macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
                        RYU_X[get_player_number(boma)] = PostureModule::pos_x(boma);
                        RYU_Y[get_player_number(boma)] = PostureModule::pos_y(boma);
                        if RYU_X[get_player_number(boma)] == OPPONENT_X[get_player_number(boma)] {
                            OPPONENT_DIRECTION[get_player_number(boma)] = 12.0 * PostureModule::lr(boma);
                            SEC_SEN_DIREC[get_player_number(boma)] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                        }
                        else if RYU_X[get_player_number(boma)] < OPPONENT_X[get_player_number(boma)] {
                            OPPONENT_DIRECTION[get_player_number(boma)] = 12.0;
                            if PostureModule::lr(boma) == -1.0 {
                                PostureModule::reverse_lr(boma);
                            }
                            SEC_SEN_DIREC[get_player_number(boma)] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                        }
                        else {
                            OPPONENT_DIRECTION[get_player_number(boma)] = -12.0;
                            if PostureModule::lr(boma) == 1.0 {
                                PostureModule::reverse_lr(boma);
                            }
                            SEC_SEN_DIREC[get_player_number(boma)] = *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F;
                        }
                        if OPPONENT_BOMA[get_player_number(boma)] != 0 {
                            if (RYU_Y[get_player_number(boma)] - OPPONENT_Y[get_player_number(boma)]).abs() <= 12.0
                            && StatusModule::situation_kind(OPPONENT_BOMA[get_player_number(boma)] as *mut BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                                VERT_EXTRA[get_player_number(boma)] = 0.0;
                            }
                        }
                        else {
                            StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
                            WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                            VERT_EXTRA[get_player_number(boma)] = 12.0;
                            RYU_Y[get_player_number(boma)] += 2.0;
                            PostureModule::add_pos_2d(boma, &Vector2f{
                                x: 0.0,
                                y: 2.0
                            });
                        }
                        CAMERA[get_player_number(boma)] = true;
                    }
                    if SEC_SEN_TIMER[get_player_number(boma)] >= 0.0 {
                        if OPPONENT_BOMA[get_player_number(boma)] != 0 {
                            if (RYU_Y[get_player_number(boma)] - OPPONENT_Y[get_player_number(boma)]).abs() <= 12.0
                            && StatusModule::situation_kind(OPPONENT_BOMA[get_player_number(boma)] as *mut BattleObjectModuleAccessor) == *SITUATION_KIND_GROUND {
                                GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                            }
                        }
                        if StatusModule::status_kind(boma) != SEC_SEN_DIREC[get_player_number(boma)] {
                            KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_RESET);
                            StatusModule::change_status_request_from_script(boma, SEC_SEN_DIREC[get_player_number(boma)], true);
                        }
                        if (RYU_Y[get_player_number(boma)] - OPPONENT_Y[get_player_number(boma)]).abs() > 12.0 {
                            StatusModule::set_situation_kind(boma, SituationKind(*SITUATION_KIND_AIR), true);
                        }
                        PostureModule::set_pos_2d(boma, &Vector2f{
                            x: (((OPPONENT_X[get_player_number(boma)] + OPPONENT_DIRECTION[get_player_number(boma)]) * SEC_SEN_TIMER[get_player_number(boma)]) + RYU_X[get_player_number(boma)] * (1.0 - SEC_SEN_TIMER[get_player_number(boma)])),
                            y: (((OPPONENT_Y[get_player_number(boma)] + VERT_EXTRA[get_player_number(boma)]) * SEC_SEN_TIMER[get_player_number(boma)]) + RYU_Y[get_player_number(boma)] * (1.0 - SEC_SEN_TIMER[get_player_number(boma)]))
                        });
                    }
                    SEC_SEN_TIMER[get_player_number(boma)] += 0.08;
                    if SEC_SEN_TIMER[get_player_number(boma)] > 1.0 {
                        SECRET_SENSATION[get_player_number(boma)] = false;
                        CAMERA[get_player_number(boma)] = false;
                        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
                        if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR {
                            StatusModule::change_status_request_from_script(boma, *FIGHTER_RYU_STATUS_KIND_TURN_AUTO, true);
                        }
                        macros::CAM_ZOOM_OUT(fighter);
                        macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
                        SlowModule::clear_whole(boma);
                        JostleModule::set_status(boma, true);
                        SEC_SEN_TIMER[get_player_number(boma)] = -0.6;
                        OPPONENT_BOMA[get_player_number(boma)] = 0;
                    }
                }
                else if MotionModule::motion_kind(boma) == hash40("appeal_hi_r")
                || MotionModule::motion_kind(boma) == hash40("appeal_hi_l") {
                    if MotionModule::frame(boma) == 4.0 {
                        macros::PLAY_SE(fighter, Hash40::new("se_ryu_6c_aura"));
                        SEC_SEN_STATE[get_player_number(boma)] = true;
                        EX_FLASH[get_player_number(boma)] = true;
                        _TIME_COUNTER[get_player_number(boma)] = -1;
                        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
                        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15db57d7a6), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
                        macros::BURN_COLOR(fighter, 0.0, 0.55, 1.0, 0.7);
                    }
                    if MotionModule::frame(boma) <= 30.0
                    && MotionModule::frame(boma) >= 4.0 {
                        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
                        DamageModule::set_damage_lock(boma, true);
                    }
                    else {
                        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x15db57d7a6), false, true);
                        macros::BURN_COLOR_NORMAL(fighter);
                        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
                        DamageModule::set_damage_lock(boma, false);
                        EX_FLASH[get_player_number(boma)] = false;
                        macros::COL_NORMAL(fighter);
                        SEC_SEN_STATE[get_player_number(boma)] = false;
                        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
                    }
                }
                else if SECRET_SENSATION[get_player_number(boma)] == false
                && SEC_SEN_STATE[get_player_number(boma)] {
                    DamageModule::set_damage_lock(boma, false);
                    DamageModule::set_no_reaction_no_effect(boma, false);
                    HitModule::set_hit_stop_mul(boma, 1.0, HitStopMulTarget{_address: *HIT_STOP_MUL_TARGET_SELF as u8}, 0.0);
                    EX_FLASH[get_player_number(boma)] = false;
                    macros::COL_NORMAL(fighter);
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x15db57d7a6), false, true);
                    macros::BURN_COLOR_NORMAL(fighter);
                    SEC_SEN_STATE[get_player_number(boma)] = false;
                    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
                }
            }
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        ryu_frame
    );
}