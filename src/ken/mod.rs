use smash::phx::Hash40;
use smash::hash40;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash::app::*;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash_script::*;
use smashline::*;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use crate::{/*IS_FUNNY, COUNTER_HIT_STATE, */_TIME_COUNTER, OPPONENT_BOMA, DAMAGE_TAKEN, DAMAGE_TAKEN_PREV};
use crate::system::DMG_RATIO;
use crate::commonfuncs::*;

// ---------------------------------------------------------
// This one was all WuBoy. He wanted to give Ken his tools from Street Fighter V
// to differentiate him from Ryu, who’s now primarily based on Super Street Fighter IV.
// Hopefully this incentivises a more ground-based combo game and rushdown,
// and will likely be much more balanced after reworking his damage output.
// ---------------------------------------------------------

// Notes:
// vc_ken_special_l01 is "I hit my boiling point!"
// vc_ken_special_l02 is "Shoryureppa"

pub static mut QUICK_STEP_STATE : [i32; 8] = [0; 8];
/*
State list:
0 = Can Quick Step
2 = Cannot Quick Step
1 = Used to show you're in the first 22 frames of Quick Step.
*/
static mut VS1_CANCEL : [bool; 8] = [false; 8];
pub static mut V_SHIFT : [bool; 8] = [false; 8];
pub static mut V_TRIGGER : [bool; 8] = [false; 8];
static mut VT_ACTIVATION : [bool; 8] = [false; 8];
static mut VT1_CANCEL : [bool; 8] = [false; 8];
pub static mut V_GAUGE : [i32; 8] = [0; 8];
static mut FLASH_MAX : [i32; 8] = [0; 8];
static mut FLASH_COUNTER : [i32; 8] = [0; 8];
pub static mut SHORYUREPPA : [i32; 8] = [0; 8];
pub static mut TATSULOOPS : [[i32; 3]; 8] = [[0; 3]; 8];
static mut CURR_LOOPS : [i32; 8] = [0; 8];
static mut DIFF_X : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_KEN )]
fn ken_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        
        if get_player_number(boma) < 8 {

            // Reset Vars

            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_REBIRTH || sv_information::is_ready_go() == false {
                QUICK_STEP_STATE[get_player_number(boma)] = 0;
                VS1_CANCEL[get_player_number(boma)] = false;
                V_SHIFT[get_player_number(boma)] = false;
                V_TRIGGER[get_player_number(boma)] = false;
                VT1_CANCEL[get_player_number(boma)] = false;
                SHORYUREPPA[get_player_number(boma)] = 0;
                OPPONENT_BOMA[get_player_number(boma)] = 0;
            }

            if sv_information::is_ready_go() == false {
                V_GAUGE[get_player_number(boma)] = 0;
                FLASH_MAX[get_player_number(boma)] = 0;
                FLASH_COUNTER[get_player_number(boma)] = 0;
            }

            // V Gauge Building (only for blocked moves and getting hit)

            if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_SHIELD)
            && MotionModule::motion_kind(boma) != hash40("special_lw")
            && V_TRIGGER[get_player_number(boma)] == false {
                if MotionModule::motion_kind(boma) == hash40("attack_s3_s_w")
                && QUICK_STEP_STATE[get_player_number(boma)] == 1 {
                    V_GAUGE[get_player_number(boma)] += 50;
                    // println!("Quick Step Kick Blocked: {}", V_GAUGE[get_player_number(boma)]);
                }
                else {
                    V_GAUGE[get_player_number(boma)] += AttackModule::get_power(boma, 0, false, 1.0, false) as i32 * 2;
                    if V_GAUGE[get_player_number(boma)] > 900 {
                        V_GAUGE[get_player_number(boma)] = 900;
                    }
                    // println!("Move Blocked: {}", V_GAUGE[get_player_number(boma)]);
                }
                if V_GAUGE[get_player_number(boma)] > 900 {
                    V_GAUGE[get_player_number(boma)] = 900;
                }
            }

            DAMAGE_TAKEN[get_player_number(boma)] = DamageModule::damage(boma, 0);
            if DAMAGE_TAKEN[get_player_number(boma)] > DAMAGE_TAKEN_PREV[get_player_number(boma)]
            && MotionModule::motion_kind(boma) != hash40("special_lw_step_b") {
                V_GAUGE[get_player_number(boma)] += (DAMAGE_TAKEN[get_player_number(boma)] - DAMAGE_TAKEN_PREV[get_player_number(boma)]) as i32 * 2;
                if V_GAUGE[get_player_number(boma)] > 900 {
                    V_GAUGE[get_player_number(boma)] = 900;
                }
                // println!("Got Hit: {}", V_GAUGE[get_player_number(boma)]);
            }
            DAMAGE_TAKEN_PREV[get_player_number(boma)] = DAMAGE_TAKEN[get_player_number(boma)];

            // V-Gauge Effects

            if V_GAUGE[get_player_number(boma)] < 300
            && FLASH_MAX[get_player_number(boma)] != 0 {
                macros::COL_NORMAL(fighter);
                FLASH_MAX[get_player_number(boma)] = 0;
                FLASH_COUNTER[get_player_number(boma)] = 0;
            }
            else if V_GAUGE[get_player_number(boma)] >= 300
            && V_GAUGE[get_player_number(boma)] < 600
            && FLASH_MAX[get_player_number(boma)] != 60 {
                macros::COL_NORMAL(fighter);
                FLASH_MAX[get_player_number(boma)] = 60;
            }
            else if V_GAUGE[get_player_number(boma)] >= 600
            && V_GAUGE[get_player_number(boma)] < 900
            && FLASH_MAX[get_player_number(boma)] != 40 {
                macros::COL_NORMAL(fighter);
                FLASH_MAX[get_player_number(boma)] = 40;
            }
            else if V_GAUGE[get_player_number(boma)] == 900
            && FLASH_MAX[get_player_number(boma)] != 20 {
                macros::COL_NORMAL(fighter);
                FLASH_MAX[get_player_number(boma)] = 20;
            }

            if FLASH_MAX[get_player_number(boma)] != 0 {
                if FLASH_COUNTER[get_player_number(boma)] == 0 {
                    FLASH_COUNTER[get_player_number(boma)] = FLASH_MAX[get_player_number(boma)];
                }
                if FLASH_COUNTER[get_player_number(boma)] == FLASH_MAX[get_player_number(boma)] {
                    macros::FLASH(fighter, 1, 0.2, 0, 0.75);
                }
                if FLASH_COUNTER[get_player_number(boma)] == FLASH_MAX[get_player_number(boma)] - 5 {
                    macros::COL_NORMAL(fighter);
                }
                FLASH_COUNTER[get_player_number(boma)] -= 1;
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
                    VS1_CANCEL[get_player_number(boma)] = true;
                }
                else {
                    VS1_CANCEL[get_player_number(boma)] = false;
                }
            }
            else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_WAIT
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_WALK
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT_B
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT_F
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT_RV
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT_WAIT
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_WALL_JUMP
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_AERIAL
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL_AERIAL
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DAMAGE_FALL {
                VS1_CANCEL[get_player_number(boma)] = true;
            }
            else {
                VS1_CANCEL[get_player_number(boma)] = false;
            }

            if MotionModule::motion_kind(boma) == hash40("special_lw_step_f")
            && QUICK_STEP_STATE[get_player_number(boma)] == 1
            && MotionModule::frame(boma) == 1.0 {
                MotionModule::change_motion(boma, Hash40::new("run"), 0.0, 1.0, true, 0.0, false, false);
            }

            if MotionModule::motion_kind(boma) == hash40("run")
            && QUICK_STEP_STATE[get_player_number(boma)] == 1 {
                if MotionModule::frame(boma) >= 22.0 && MotionModule::frame(boma) <= 23.0
                && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
                    MotionModule::change_motion(boma, Hash40::new("attack_s3_s_w"), 0.0, 1.0, false, 0.0, false, false);
                }
                if MotionModule::frame(boma) >= 31.0 {
                    CancelModule::enable_cancel(boma);
                }
            }

            if MotionModule::motion_kind(boma) == hash40("attack_s3_s_w") {
                if MotionModule::frame(boma) > 26.0 {
                    QUICK_STEP_STATE[get_player_number(boma)] = 0;
                    CancelModule::enable_cancel(boma);
                }
            }

            if (StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_RUN
            && StatusModule::status_kind(boma) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F)
            && QUICK_STEP_STATE[get_player_number(boma)] == 1 {
                QUICK_STEP_STATE[get_player_number(boma)] = 2;
            }

            if ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                if VS1_CANCEL[get_player_number(boma)]
                && QUICK_STEP_STATE[get_player_number(boma)] == 0
                && V_GAUGE[get_player_number(boma)] < 900 {
                    if MotionModule::motion_kind(boma) == hash40("attack_air_b") {
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
                else if VT1_CANCEL[get_player_number(boma)]
                && V_GAUGE[get_player_number(boma)] == 900
                && V_TRIGGER[get_player_number(boma)] == false {
                    VT_ACTIVATION[get_player_number(boma)] = true;
                }
            }

            if QUICK_STEP_STATE[get_player_number(boma)] == 2
            && StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                QUICK_STEP_STATE[get_player_number(boma)] = 0;
            }

            // V Trigger 1

            if (AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT)
            || AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_SHIELD))
            || ((StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_N
            || StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND)
            && MotionModule::frame(boma) >= 13.0) {
                VT1_CANCEL[get_player_number(boma)] = true;
            }
            else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_WAIT
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_WALK
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT_B
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT_F
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT_RV
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SQUAT_WAIT
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_SQUAT
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_AERIAL
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL_AERIAL
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_GUARD
            || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CATCH {
                VT1_CANCEL[get_player_number(boma)] = true;
            }
            else {
                VT1_CANCEL[get_player_number(boma)] = false;
            }

            // if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_GUARD)
            // && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
            // && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_ATTACK_RAW)
            // && VT1_CANCEL[get_player_number(boma)]
            // && V_GAUGE[get_player_number(boma)] == 900
            // && V_TRIGGER[get_player_number(boma)] == false {
            //     VT_ACTIVATION[get_player_number(boma)] = true;
            // }

            if VT_ACTIVATION[get_player_number(boma)] {
                if StatusModule::status_kind(boma) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F {
                    fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F.into(), false.into());
                    V_GAUGE[get_player_number(boma)] = 0;
                }
                if StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F
                && MotionModule::frame(boma) <= 1.0
                && V_TRIGGER[get_player_number(boma)] == false {
                    WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
                    KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_RESET);
                    if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                        GroundModule::set_correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                    }
                    HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_XLU), 0);
                    SlowModule::set_whole(boma, 6, 0);
                    macros::SLOW_OPPONENT(fighter, 100.0, 12.0);
                    macros::FILL_SCREEN_MODEL_COLOR(fighter, 0, 3, 0.2, 0.2, 0.2, 0, 0, 0, 1, 1, *smash::lib::lua_const::EffectScreenLayer::GROUND, 205);
                    if OPPONENT_BOMA[get_player_number(boma)] != 0 {
                        DIFF_X[get_player_number(boma)] = (
                            PostureModule::pos_x(OPPONENT_BOMA[get_player_number(boma)] as *mut BattleObjectModuleAccessor) - PostureModule::pos_x(boma)
                        ).abs();
                        if DIFF_X[get_player_number(boma)] > 50.0 {
                            DIFF_X[get_player_number(boma)] -= 5.0;
                        }
                        else {
                            DIFF_X[get_player_number(boma)] = 0.0;
                        }
                        OPPONENT_BOMA[get_player_number(boma)] = 0;
                    }
                    else {
                        DIFF_X[get_player_number(boma)] = 0.0;
                    }
                    macros::PLAY_SE(fighter, Hash40::new("se_ken_special_l01"));
                    macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_l01"));
                    V_TRIGGER[get_player_number(boma)] = true;
                }
                if MotionModule::frame(boma) >= 4.0
                && MotionModule::frame(boma) < 9.0 {
                    SlowModule::clear_whole(boma);
                    if DIFF_X[get_player_number(boma)] != 0.0 {
                        PostureModule::add_pos_2d(boma, &Vector2f{
                            x: (DIFF_X[get_player_number(boma)] / 5.0) * PostureModule::lr(boma),
                            y: 0.0
                        });
                    }
                    else {
                        PostureModule::add_pos_2d(boma, &Vector2f{
                            x: 10.0 * PostureModule::lr(boma),
                            y: 0.0
                        });
                    }
                }
                if MotionModule::frame(boma) >= 9.0
                && MotionModule::frame(boma) < 19.0 {
                    MotionModule::set_frame(boma, 19.0, true);
                }
                if StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F
                && MotionModule::frame(boma) >= 19.0 {
                    HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
                    macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
                    CancelModule::enable_cancel(boma);
                    VT_ACTIVATION[get_player_number(boma)] = false;
                }
            }

            if StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_HI
            && StatusModule::status_kind(boma) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND
            && StatusModule::status_kind(boma) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_JUMP
            && StatusModule::status_kind(boma) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
            && StatusModule::status_kind(boma) != *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_FALL {
                SHORYUREPPA[get_player_number(boma)] = 0;
            }

            if StatusModule::status_kind(boma) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_LANDING
            && SHORYUREPPA[get_player_number(boma)] == 1 {
                fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND.into(), false.into());
            }

            if V_TRIGGER[get_player_number(boma)] {
                if _TIME_COUNTER[get_player_number(boma)] < 0 {
                    _TIME_COUNTER[get_player_number(boma)] = 32;
                }
                if _TIME_COUNTER[get_player_number(boma)] == 32 {
                    EffectModule::req_follow(boma, Hash40::new("sys_flame"), smash::phx::Hash40::new("footl"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0.2, true, 0, 0, 0, 0, 0, true, true);
                }
                if _TIME_COUNTER[get_player_number(boma)] == 16 {
                    EffectModule::req_follow(boma, Hash40::new("sys_flame"), smash::phx::Hash40::new("footr"), &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 90.0, y: 0.0, z: 0.0}, 0.2, true, 0, 0, 0, 0, 0, true, true);
                }
                _TIME_COUNTER[get_player_number(boma)] -= 1;
            }

            // V Shift

            if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_GUARD
            && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL)
            && V_GAUGE[get_player_number(boma)] >= 300 {
                let stick_x = ControlModule::get_stick_x(boma);
                if (stick_x < -0.5 && PostureModule::lr(boma) == 1.0)
                || (stick_x > 0.5 && PostureModule::lr(boma) == -1.0) {
                    V_GAUGE[get_player_number(boma)] -= 300;
                    if V_GAUGE[get_player_number(boma)] < 0 {
                        V_GAUGE[get_player_number(boma)] = 0;
                    }
                    fighter.change_status(FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B.into(), false.into());
                }
            }

            if MotionModule::motion_kind(boma) == hash40("special_lw_step_b") {
                if MotionModule::frame(boma) <= 1.0
                && V_SHIFT[get_player_number(boma)] == false {
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 1.4, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("neck"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("kneel"), 4, 0, 0, 0, 0, 0, 1.1, true);
                    macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x15a0de794a), Hash40::new("kneer"), 4, 0, 0, 0, 0, 0, 1.1, true);
                }
                if MotionModule::frame(boma) == 9.375 {
                    if V_SHIFT[get_player_number(boma)] {
                        V_GAUGE[get_player_number(boma)] += 150;
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
            
            if MotionModule::motion_kind(boma) != hash40("special_lw_step_b")
            && MotionModule::motion_kind(boma) != hash40("special_lw") {
                V_SHIFT[get_player_number(boma)] = false;
            }

            // Training Mode Tools

            if smashball::is_training_mode(){
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                    if V_GAUGE[get_player_number(boma)] > 300 {
                        V_GAUGE[get_player_number(boma)] -= 300
                    }
                    else {
                        V_GAUGE[get_player_number(boma)] = 0;
                    }
                }
                if ControlModule::check_button_on_trriger(boma, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    if V_GAUGE[get_player_number(boma)] < 900 {
                        V_GAUGE[get_player_number(boma)] += 300;
                    }
                    else {
                        V_GAUGE[get_player_number(boma)] = 900;
                    }
                }
            }
        }
    }
}

// Motion Rate the Run Animation so that it moves at the right speed during Quick Step

#[acmd_script( agent = "ken", script = "game_run", category = ACMD_GAME, low_priority )]
unsafe fn ken_run(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if QUICK_STEP_STATE[get_player_number(boma)] == 1 {
        macros::FT_MOTION_RATE(fighter, 0.7);
    }
}

// Make Quick Step (non-prox light f tilt) have step kick properties

#[acmd_script( agent = "ken", script = "game_attacks3w", category = ACMD_GAME, low_priority )]
unsafe fn ken_ftiltwnp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    sv_animcmd::frame(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 4.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if QUICK_STEP_STATE[get_player_number(boma)] == 1 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 72, 46, 33, 50, 3.8, 0.0, 11.0, 6.5, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 72, 46, 33, 50, 3.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 55, 46, 0, 66, 3.8, 0.0, 11.0, 6.5, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.8, 72, 46, 0, 66, 3.8, 0.0, 11.0, 14.0, Some(0.0), Some(11.0), Some(5.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    sv_animcmd::frame(lua_state, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        HitModule::set_status_all(boma, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    macros::FT_MOTION_RATE(fighter, 0.8);
}

// Nerfed damage on Inazuma Kick, but increased combo potential

#[acmd_script( agent = "ken", script = "game_attackcommand3", category = ACMD_GAME, low_priority )]
unsafe fn ken_attackcommand3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    sv_animcmd::frame(lua_state, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("kneer"), 10.0, 30, 50, 0, 35, 3.0, 6.3, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_PUNCH);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.25);
        AttackModule::set_attack_height_all(boma, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

// V Shift start-up.

#[acmd_script( agent = "ken", script = "game_speciallwstepb", category = ACMD_GAME, low_priority )]
unsafe fn ken_dspecialstepb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        DamageModule::set_damage_lock(boma, true);
        // HitModule::set_whole(boma, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    macros::FT_MOTION_RATE(fighter, 1.6);
    sv_animcmd::frame(lua_state, 8.75);
    if macros::is_excute(fighter) {
        smash_script::damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        DamageModule::set_damage_lock(boma, false);
    }
    if V_SHIFT[get_player_number(boma)] {
        if macros::is_excute(fighter) {
            HitModule::set_whole(boma, HitStatus(*HIT_STATUS_XLU), 0);
        }
    }
}

// V-Shift Break - Shadow-Thunder Kick

#[acmd_script( agent = "ken", script = "game_speciallw", category = ACMD_GAME, low_priority )]
unsafe fn ken_dspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    if macros::is_excute(fighter) {
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        if V_SHIFT[get_player_number(boma)] {
            macros::SLOW_OPPONENT(fighter, 100.0, 18.0);
        }
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 6.0, 30, 98, 100, 0, 3.2, -1.5, -1.0, -1.0, None, None, None, 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 6.0, 30, 98, 100, 0, 3.2, -6.2, -1.0, -1.0, None, None, None, 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 6.0, 30, 98, 100, 0, 3.9, 4.3, -1.7, -1.0, None, None, None, 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 5.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 5.0, false);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
        HitModule::set_whole(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        if V_SHIFT[get_player_number(boma)] {
            macros::CANCEL_FILL_SCREEN(fighter, 0, 5);
            V_SHIFT[get_player_number(boma)] = false;
        }
    }
}

#[acmd_script( agent = "ken", script = "sound_speciallw", category = ACMD_SOUND, low_priority )]
unsafe fn ken_dspecialsnd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
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

#[acmd_script( agent = "ken", script = "expression_speciallw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn ken_dspecialxp(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
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

#[acmd_script( agent = "ken", script = "effect_speciallw", category = ACMD_EFFECT, low_priority )]
unsafe fn ken_dspecialeff(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kneel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_thunder"), Hash40::new("footl"), 0.5, 0, 0, 0, 0, 0, 1.5, true);
        
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLIP(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.5, 1);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new_raw(0x0f998ceac2), Hash40::new_raw(0x0f998ceac2), Hash40::new("top"), -2, 10, 1, -12, 0, 0, 0.7, true, *EF_FLIP_YZ);
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 12.5, 14, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, true, 0.5);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(lua_state, 18.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_thunder"), false, true);
    }
}

// The rest of the scripts below have their properties changed to Fire when V-Trigger is activated

#[acmd_script( agent = "ken", scripts = ["game_specialsstart", "game_specialairsstart"], category = ACMD_GAME, low_priority )]
unsafe fn ken_sspecialstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    DMG_RATIO[get_player_number(boma)] = 0.9;
    if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
        DMG_RATIO[get_player_number(boma)] = 1.2;
    }
    sv_animcmd::frame(lua_state, 7.0);
    if macros::is_excute(fighter) {
        CURR_LOOPS[get_player_number(boma)] = 0;
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 1.0, 3.5, 8.5, 8.5);
    }
    sv_animcmd::frame(lua_state, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * DMG_RATIO[get_player_number(boma)], 0, 50, 100, 0, 4.5, 0.0, 9.0, 4.5, Some(0.0), Some(9.0), Some(4.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 3.0, 3.5, 8.5, 4.5);
    }
    sv_animcmd::wait(lua_state, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
    }
}

#[acmd_script( agent = "ken", script = "game_specials", category = ACMD_GAME, low_priority )]
unsafe fn ken_sspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    DMG_RATIO[get_player_number(boma)] = 0.9;
    if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
        DMG_RATIO[get_player_number(boma)] = 1.2;
    }
    if macros::is_excute(fighter) {
        CURR_LOOPS[get_player_number(boma)] += 1;
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(boma, 5.5, 3.0, 9.0, 3.0);
        macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
    }
    sv_animcmd::wait(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * DMG_RATIO[get_player_number(boma)], 30, 20, 0, 60, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if CURR_LOOPS[get_player_number(boma)] == TATSULOOPS[get_player_number(boma)][1] {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0 * DMG_RATIO[get_player_number(boma)], 30, 30, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0 * DMG_RATIO[get_player_number(boma)], 30, 30, 0, 80, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * DMG_RATIO[get_player_number(boma)], 55, 20, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0 * DMG_RATIO[get_player_number(boma)], 31, 25, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if CURR_LOOPS[get_player_number(boma)] == TATSULOOPS[get_player_number(boma)][2] {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * DMG_RATIO[get_player_number(boma)], 40, 20, 0, 120, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0 * DMG_RATIO[get_player_number(boma)], 40, 20, 0, 120, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * DMG_RATIO[get_player_number(boma)], 55, 20, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0 * DMG_RATIO[get_player_number(boma)], 31, 25, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * DMG_RATIO[get_player_number(boma)], 45, 120, 0, 30, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * DMG_RATIO[get_player_number(boma)], 55, 20, 0, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0 * DMG_RATIO[get_player_number(boma)], 55, 20, 0, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(boma, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        AttackModule::set_size(boma, 1, 0.1);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ken", script = "game_specialairs", category = ACMD_GAME, low_priority )]
unsafe fn ken_sspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    DMG_RATIO[get_player_number(boma)] = 0.9;
    if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
        DMG_RATIO[get_player_number(boma)] = 1.2;
    }
    if macros::is_excute(fighter) {
        CURR_LOOPS[get_player_number(boma)] += 1;
    }
    sv_animcmd::wait(lua_state, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.6);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0 * DMG_RATIO[get_player_number(boma)], 30, 30, 0, 60, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        if CURR_LOOPS[get_player_number(boma)] == TATSULOOPS[get_player_number(boma)][1] {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * DMG_RATIO[get_player_number(boma)], 30, 20, 0, 70, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * DMG_RATIO[get_player_number(boma)], 55, 20, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    else {
        if CURR_LOOPS[get_player_number(boma)] == TATSULOOPS[get_player_number(boma)][2] {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0 * DMG_RATIO[get_player_number(boma)], 30, 20, 0, 120, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * DMG_RATIO[get_player_number(boma)], 55, 20, 0, 45, 3.5, 0.0, 12.5, 12.5, Some(0.0), Some(12.5), Some(2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
        }
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * DMG_RATIO[get_player_number(boma)], 45, 120, 0, 30, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * DMG_RATIO[get_player_number(boma)], 55, 20, 0, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0 * DMG_RATIO[get_player_number(boma)], 55, 20, 0, 45, 3.5, 0.0, 12.5, -11.0, Some(0.0), Some(12.5), Some(-2.0), 1.0, 0.5, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_B, false, 0, 0.0, 9, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(lua_state, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(boma, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(boma, 0, 0.1);
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
}

#[acmd_script( agent = "ken", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn ken_uspecial(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    if SHORYUREPPA[get_player_number(boma)] == 1 {
        DMG_RATIO[get_player_number(boma)] = 0.7
    }
    else if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
        DMG_RATIO[get_player_number(boma)] = 1.0;
    }
    else {
        DMG_RATIO[get_player_number(boma)] = 0.8;
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_W
    && WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2 * DMG_RATIO[get_player_number(boma)], 100, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 6.0);
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0 * DMG_RATIO[get_player_number(boma)], 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[get_player_number(boma)], 80, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[get_player_number(boma)], 95, 10, 0, 95, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0 * DMG_RATIO[get_player_number(boma)], 70, 105, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0 * DMG_RATIO[get_player_number(boma)], 70, 121, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5 * DMG_RATIO[get_player_number(boma)], 70, 116, 0, 80, 5.5, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

// Special Hi Command also allows you to use Shoryureppa in V-Trigger

#[acmd_script( agent = "ken", script = "game_specialhicommand", category = ACMD_GAME, low_priority )]
unsafe fn ken_uspecialcommand(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    if SHORYUREPPA[get_player_number(boma)] == 1 {
        DMG_RATIO[get_player_number(boma)] = 0.7
    }
    else if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
        DMG_RATIO[get_player_number(boma)] = 1.0;
    }
    else {
        DMG_RATIO[get_player_number(boma)] = 0.8;
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if SHORYUREPPA[get_player_number(boma)] == 1 {
        WorkModule::set_int(boma, *FIGHTER_RYU_STRENGTH_S, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH);
    }
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_W
    && WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_M {
        if V_TRIGGER[get_player_number(boma)] {
            SHORYUREPPA[get_player_number(boma)] += 1;
        }
        if SHORYUREPPA[get_player_number(boma)] == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 270, 10, 0, 25, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.5, 270, 10, 0, 25, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.2, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 5.0, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2 * DMG_RATIO[get_player_number(boma)], 100, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(lua_state, 6.0);
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0 * DMG_RATIO[get_player_number(boma)], 80, 58, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[get_player_number(boma)], 80, 100, 100, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        if SHORYUREPPA[get_player_number(boma)] == 1 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 270, 10, 0, 25, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 270, 10, 0, 25, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 28.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 28.0, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[get_player_number(boma)], 95, 10, 0, 95, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0 * DMG_RATIO[get_player_number(boma)], 70, 105, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0 * DMG_RATIO[get_player_number(boma)], 70, 121, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        if SHORYUREPPA[get_player_number(boma)] == 1 {
            AttackModule::clear_all(boma);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 270, 10, 30, 30, 5.5, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 270, 10, 30, 30, 5.5, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.0, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, -7.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(boma, 0, 28.0, false);
            AttackModule::set_add_reaction_frame(boma, 1, 28.0, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.5 * DMG_RATIO[get_player_number(boma)], 70, 116, 0, 80, 5.5, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ken", script = "sound_specialhicommand", category = ACMD_SOUND, low_priority )]
unsafe fn ken_uspecialcommandsnd(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    sv_animcmd::frame(lua_state, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ken_command_success"));
    }
    sv_animcmd::frame(lua_state, 2.0);
    if macros::is_excute(fighter) {
        if SHORYUREPPA[get_player_number(boma)] == 0 {
            macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_h01_command"));
        }
        else {
            macros::PLAY_SE(fighter, Hash40::new("vc_ken_special_l02"));
        }
    }
    sv_animcmd::frame(lua_state, 6.0);
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h03"));
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h01"));
        }
    }
    sv_animcmd::frame(lua_state, 10.0);
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_S {
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_ken_special_h02"));
        }
    }
}

#[acmd_script( agent = "ken", scripts = ["game_specialairhi", "game_specialairhicommand"], category = ACMD_GAME, low_priority )]
unsafe fn ken_uspecialair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
        DMG_RATIO[get_player_number(boma)] = 1.0;
    }
    else {
        DMG_RATIO[get_player_number(boma)] = 0.8;
    }
    sv_animcmd::frame(lua_state, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_REVERSE_LR);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_DECIDE_STRENGTH);
    }
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_W
    && WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) != *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2 * DMG_RATIO[get_player_number(boma)], 90, 100, 90, 0, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.1, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 1, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 6.0);
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0 * DMG_RATIO[get_player_number(boma)], 80, 55, 0, 80, 4.6, 0.0, 10.0, 7.6, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0 * DMG_RATIO[get_player_number(boma)], 80, 54, 0, 70, 4.6, 0.0, 10.0, 7.6, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[get_player_number(boma)], 80, 100, 100, 0, 4.6, 0.0, 14.5, 7.1, Some(0.0), Some(12.5), Some(9.1), 1.22, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_HI_FLAG_JUMP);
    }
    if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_W {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0 * DMG_RATIO[get_player_number(boma)], 70, 70, 0, 60, 5.0, 4.0, -0.4, 0.0, None, None, None, 1.22, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else if WorkModule::get_int(boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0 * DMG_RATIO[get_player_number(boma)], 70, 112, 0, 78, 5.5, 4.0, -0.4, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_PUNCH);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 6.0 * DMG_RATIO[get_player_number(boma)], 70, 107, 0, 76, 6.0, 4.0, -0.4, 0.0, Some(-3.0), Some(-0.4), Some(0.0), 1.4, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_SHORYU, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(lua_state, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(boma, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    sv_animcmd::frame(lua_state, 20.0);
    if macros::is_excute(fighter) {
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "ken_hadoken", script = "game_movew", category = ACMD_GAME, low_priority )]
unsafe fn ken_hadokenw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 0, 10, 0, 45, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
    }
    sv_animcmd::wait(lua_state, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 0, 10, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script( agent = "ken_hadoken", script = "game_movem", category = ACMD_GAME, low_priority )]
unsafe fn ken_hadokenm(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 50, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
    }
    sv_animcmd::wait(lua_state, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 0, 10, 0, 50, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

#[acmd_script( agent = "ken_hadoken", script = "game_moves", category = ACMD_GAME, low_priority )]
unsafe fn ken_hadokens(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let mut property = "collision_attr_normal";
    if V_TRIGGER[get_player_number(boma)] {
        property = "collision_attr_fire";
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 0, 10, 0, 55, 3.5, 0.0, 0.5, -0.5, Some(0.0), Some(-5.2), Some(-0.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 60, 10, 0, 65, 2.8, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-3.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        AttackModule::set_add_reaction_frame(boma, 0, 5.0, false);
    }
    sv_animcmd::wait(lua_state, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 0, 10, 0, 55, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(-2.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 60, 10, 0, 65, 2.5, 0.0, 1.3, -1.25, Some(0.0), Some(-1.3), Some(-1.25), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new(property), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KEN_PUNCH, *ATTACK_REGION_ENERGY);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        ken_frame
    );
    smashline::install_acmd_scripts!(
        ken_run,
        ken_ftiltwnp,
        ken_attackcommand3,
        ken_dspecialstepb,
        ken_dspecial,
        ken_dspecialsnd,
        ken_dspecialxp,
        ken_dspecialeff,
        ken_sspecialstart,
        ken_sspecial,
        ken_sspecialair,
        ken_uspecial,
        ken_uspecialcommand,
        ken_uspecialcommandsnd,
        ken_uspecialair,
        ken_hadokenw,
        ken_hadokenm,
        ken_hadokens
    );
}