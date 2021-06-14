use smash::lib::lua_const::*;
use smash::lua2cpp::{L2CAgentBase, L2CFighterCommon};
use smash::app::lua_bind::*;
use smash::hash40;
use smash::app::*;
use smash_script::*;
use smashline::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::phx::Vector2f;
use smash::lib::L2CValue;
use crate::system::{IS_FUNNY, _TIME_COUNTER, DAMAGE_TAKEN, DAMAGE_TAKEN_PREV, QCB, DMG_RATIO};
use crate::commonfuncs::*;
use crate::globals::*;

// ---------------------------------------------------------
// We’ve revamped Lucina with a moveset inspired by Yu Narukami’s appearance in Persona 4 Arena.
// Lucina’s skins have been fully replaced with Yu in the WuBoy Modpack,
// so it was only fitting that this became our premiere challenge for The Bor Patch.
// ---------------------------------------------------------

pub static mut AIR_ACTION : [bool; 8] = [false; 8];
static mut SHADOW_FRENZY : [bool; 8] = [false; 8];
static mut AWAKENING : [bool; 8] = [false; 8];
static mut CAN_ONE_MORE : [bool; 8] = [false; 8];
static mut TRAINING_TOOLS : [bool; 8] = [false; 8];
static mut IS_EX : [bool; 8] = [false; 8];
static mut SP_GAUGE : [f32; 8] = [0.0; 8];
static mut SPENT_SP : [f32; 8] = [0.0; 8];
static mut SP_GAUGE_MAX : [f32; 8] = [100.0; 8];
static mut METER_GAIN : [f32; 8] = [0.0; 8];
static mut GFXCOORDS : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };
static mut ROMAN_MOVE : [f32; 8] = [0.0; 8];
static mut ROMAN_ON_HIT : [bool; 8] = [false; 8];
static mut IS_ROMAN_MOVE : [bool; 8] = [false; 8];
pub static mut HEROIC_GRAB : [bool; 8] = [false; 8];
static mut EX_FLASH : [i32; 8] = [0; 8];
static mut START_SITUATION : [i32; 8] = [0; 8];

pub unsafe fn spent_meter(module_accessor: *mut BattleObjectModuleAccessor, onemore: bool) -> bool {
    let mut spent = false;
    if SP_GAUGE[entry_id(module_accessor)] > 0.0 {
        if SHADOW_FRENZY[entry_id(module_accessor)] {
            if onemore {
                SPENT_SP[entry_id(module_accessor)] = 12.5;
                spent = true;
            }
            else {
                SPENT_SP[entry_id(module_accessor)] = 6.25;
                spent = true;
            }
        }
        else if SP_GAUGE[entry_id(module_accessor)] >= 25.0 {
            SPENT_SP[entry_id(module_accessor)] = 25.0;
            spent = true;
        }
    }
    return spent;
}

// Sets Yu's upper-body invincibility, only used for Big Gamble.

pub unsafe fn upper_invuln(module_accessor: *mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_status_joint(module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("hip"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    else {
        HitModule::set_status_joint(module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("hip"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("shoulderl"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_NORMAL), 0);
        HitModule::set_status_joint(module_accessor, Hash40::new("arml"), HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

// Sets Yu's full invulnerability, only used for Big Gamble.

pub unsafe fn full_invuln(module_accessor: *mut BattleObjectModuleAccessor, is_invuln: bool) {
    if is_invuln {
        HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    else {
        HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

// Checks if you are playing as Shadow Yu.

pub unsafe fn shadow_id(module_accessor: *mut BattleObjectModuleAccessor) -> bool {
    if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 6
    || WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 7 {
        return true;
    }
    else {
        return false;
    }
}

// This is left here for the standalone release of Yu Narukami
// #[skyline::hook(replace=lua_bind::WorkModule::is_enable_transition_term)]
// pub unsafe fn lucina_is_enable_transition_term_replace(fighter.module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
//     let fighter_kind = utility::get_kind(fighter.module_accessor);
//     let ret = original!()(fighter.module_accessor,term);
//     let entry_id(fighter.module_accessor) = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_entry_id(fighter.module_accessor)) as usize;
//     if fighter_kind == *FIGHTER_KIND_LUCINA && entry_id(fighter.module_accessor) < 8 {
//         if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
//             return false;
//         }
//         if HEROIC_GRAB[entry_id(fighter.module_accessor)]
//         && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
//         && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI
//         && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
//         && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN {
//             return false;
//         }
//         if AIR_ACTION[entry_id(fighter.module_accessor)] && IS_FUNNY[entry_id(fighter.module_accessor)] == false {
//             if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S { // Disable Lion's Leap and Heroic Bravery if used once unless in Funny
//                 return false;
//             }
//             else {
//                 return ret;
//             }
//         }
//         if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
//             if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON
//             || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT {
//                 return false;
//             }
//             else {
//                 return ret;
//             }
//         }
//         else {
//             return ret;
//         }
//     }
//     else {
//         return ret;
//     }
// }

#[fighter_frame( agent = FIGHTER_KIND_LUCINA )]
fn lucina_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        if entry_id(fighter.module_accessor) < 8 {

            // Reset Vars
            
            if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH {
                AIR_ACTION[entry_id(fighter.module_accessor)] = false;
                _TIME_COUNTER[entry_id(fighter.module_accessor)] = 0;
                if shadow_id(fighter.module_accessor) {
                    if SHADOW_FRENZY[entry_id(fighter.module_accessor)] {
                        SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE[entry_id(fighter.module_accessor)] / 2.0;
                        SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
                    }
                }
                else {
                    SP_GAUGE_MAX[entry_id(fighter.module_accessor)] = 100.0;
                    SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
                    AWAKENING[entry_id(fighter.module_accessor)] = false;
                }
            }
            if sv_information::is_ready_go() == false {
                DamageModule::set_damage_mul(fighter.module_accessor, 1.0);
                AIR_ACTION[entry_id(fighter.module_accessor)] = false;
                SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
                _TIME_COUNTER[entry_id(fighter.module_accessor)] = 0;
                if !(smashball::is_training_mode() && TRAINING_TOOLS[entry_id(fighter.module_accessor)]) {
                    SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
                    AWAKENING[entry_id(fighter.module_accessor)] = false;
                    TRAINING_TOOLS[entry_id(fighter.module_accessor)] = false;
                    SP_GAUGE_MAX[entry_id(fighter.module_accessor)] = 100.0;
                    DAMAGE_TAKEN[entry_id(fighter.module_accessor)] = 0.0;
                    DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)] = 0.0;
                }
            }

            // Meter Controller

            DAMAGE_TAKEN[entry_id(fighter.module_accessor)] = DamageModule::damage(fighter.module_accessor, 0);
            if DAMAGE_TAKEN[entry_id(fighter.module_accessor)] > DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]
            && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false {
                SP_GAUGE[entry_id(fighter.module_accessor)] += (DAMAGE_TAKEN[entry_id(fighter.module_accessor)] - DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)]) * (1.0/6.0);
            }
            DAMAGE_TAKEN_PREV[entry_id(fighter.module_accessor)] = DAMAGE_TAKEN[entry_id(fighter.module_accessor)];

            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false {
                if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX
                && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_S
                && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2
                && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    IS_EX[entry_id(fighter.module_accessor)] = false;
                }
                if IS_EX[entry_id(fighter.module_accessor)] == false {
                    METER_GAIN[entry_id(fighter.module_accessor)] = AttackModule::get_power(fighter.module_accessor, 0, false, 1.0, false);
                    if shadow_id(fighter.module_accessor) == false {
                        METER_GAIN[entry_id(fighter.module_accessor)] *= 0.75;
                    }
                    if IS_FUNNY[entry_id(fighter.module_accessor)] {
                        METER_GAIN[entry_id(fighter.module_accessor)] *= 3.0;
                    }
                    SP_GAUGE[entry_id(fighter.module_accessor)] += METER_GAIN[entry_id(fighter.module_accessor)];
                }
            }

            if SP_GAUGE[entry_id(fighter.module_accessor)] > SP_GAUGE_MAX[entry_id(fighter.module_accessor)] {
                SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE_MAX[entry_id(fighter.module_accessor)];
            }

            // Normal vs Shadow Effects

            if shadow_id(fighter.module_accessor) == true {
                DamageModule::set_damage_mul(fighter.module_accessor, 0.92);
                DMG_RATIO[entry_id(fighter.module_accessor)] = 0.8;
                if SHADOW_FRENZY[entry_id(fighter.module_accessor)] == true {
                    if !TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
                        if IS_FUNNY[entry_id(fighter.module_accessor)] {
                            SP_GAUGE[entry_id(fighter.module_accessor)] -= 1.0/64.0;
                        }
                        else {
                            SP_GAUGE[entry_id(fighter.module_accessor)] -= 1.0/16.0;
                        }
                    }
                }
                if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_missfoot01")) {
                    SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_missfoot01"), 0);
                    macros::PLAY_SE(fighter, Hash40::new("vc_shadow_missfoot01"));
                }
                if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_missfoot02")) {
                    SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_missfoot02"), 0);
                    macros::PLAY_SE(fighter, Hash40::new("vc_shadow_missfoot02"));
                }
                if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_damage_twinkle")) {
                    SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_damage_twinkle"), 0);
                    macros::PLAY_SE(fighter, Hash40::new("vc_shadow_damage_twinkle"));
                }
                if SoundModule::is_playing(fighter.module_accessor, Hash40::new("vc_lucina_knockout")) {
                    SoundModule::stop_se(fighter.module_accessor, Hash40::new("vc_lucina_knockout"), 0);
                    macros::PLAY_SE(fighter, Hash40::new("vc_shadow_knockout"));
                }
            }
            else {
                DMG_RATIO[entry_id(fighter.module_accessor)] = 1.0;
                if DamageModule::damage(fighter.module_accessor, 0) >= 100.0 {
                    if AWAKENING[entry_id(fighter.module_accessor)] == false
                    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
                    && (!is_damage_check(fighter.module_accessor)
                    || IS_FUNNY[entry_id(fighter.module_accessor)])
                    && sv_information::is_ready_go() {
                        DamageModule::set_damage_mul(fighter.module_accessor, 0.8);
                        SP_GAUGE_MAX[entry_id(fighter.module_accessor)] = 150.0;
                        SP_GAUGE[entry_id(fighter.module_accessor)] += 50.0;
                        AWAKENING[entry_id(fighter.module_accessor)] = true;
                        macros::FT_START_CUTIN(fighter);
                    }
                }
            }

            if SP_GAUGE[entry_id(fighter.module_accessor)] <= 0.0{
                SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
                SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
            }

            // Special Lw Check

            if StatusModule::prev_status_kind(fighter.module_accessor, 0) != StatusModule::status_kind(fighter.module_accessor) {
                CAN_ONE_MORE[entry_id(fighter.module_accessor)] = false;
            }

            if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                if (AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD))
                && MotionModule::motion_kind(fighter.module_accessor) != hash40("catch_attack")
                && (MotionModule::motion_kind(fighter.module_accessor) != hash40("special_hi") || IS_FUNNY[entry_id(fighter.module_accessor)]) {
                    CAN_ONE_MORE[entry_id(fighter.module_accessor)] = true;
                }
    
                if CAN_ONE_MORE[entry_id(fighter.module_accessor)] == true {
                    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                        if spent_meter(fighter.module_accessor, true) {
                            ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = true;
                            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        }
                    }
                    else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                    && SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0
                    && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false
                    && shadow_id(fighter.module_accessor) {
                        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT, true);
                    }
                }
                else if StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_THROW {
                    let throwframe : f32;
                    if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_f") {
                        throwframe = 18.0;
                    }
                    else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_b") {
                        throwframe = 19.0;
                    }
                    else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_hi") {
                        throwframe = 13.0;
                    }
                    else if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_lw") {
                        throwframe = 20.0;
                    }
                    else{
                        throwframe = 20.0;
                    }
                    if MotionModule::frame(fighter.module_accessor) > throwframe && CAN_ONE_MORE[entry_id(fighter.module_accessor)] == false {
                        CAN_ONE_MORE[entry_id(fighter.module_accessor)] = true;
                    }
                    if CAN_ONE_MORE[entry_id(fighter.module_accessor)] == true {
                        if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                            if spent_meter(fighter.module_accessor, true) {
                                ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = true;
                                fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                            }
                        }
                        else if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
                        && SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0
                        && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false
                        && shadow_id(fighter.module_accessor) {
                            fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT.into(), false.into());
                        }
                    }
                }
                else if (!is_damage_check(fighter.module_accessor)
                && MotionModule::motion_kind(fighter.module_accessor) != hash40("special_hi"))
                || IS_FUNNY[entry_id(fighter.module_accessor)] {
                    if ControlModule::get_command_flag_cat(fighter.module_accessor, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_LW != 0 {
                        if spent_meter(fighter.module_accessor, true) {
                            ROMAN_ON_HIT[entry_id(fighter.module_accessor)] = false;
                            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
                        }
                    }
                }
            }

            if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_LW {
                IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] = false;
            }
            else {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                    macros::SLOW_OPPONENT(fighter, 10.0, 20.0);
                }
            }
            if IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] {
                PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
                    x: ROMAN_MOVE[entry_id(fighter.module_accessor)],
                    y: 0.0
                });
                ROMAN_MOVE[entry_id(fighter.module_accessor)] *= 0.9;
            }

            // Meter Effects

            if (SP_GAUGE[entry_id(fighter.module_accessor)] >= 25.0 && SHADOW_FRENZY[entry_id(fighter.module_accessor)] == false)
            || SHADOW_FRENZY[entry_id(fighter.module_accessor)] == true {
                if _TIME_COUNTER[entry_id(fighter.module_accessor)] == 0 {
                    let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("handr"), &Vector3f {x: 1.0, y: 0.0, z: 0.0}, &GFXCOORDS, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
                    let onemoreeff2: u32 = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_elec"), smash::phx::Hash40::new("handl"), &Vector3f {x: 1.0, y: 0.0, z: 0.0}, &GFXCOORDS, 0.3, true, 0, 0, 0, 0, 0, true, true) as u32;
                    EffectModule::set_rate(fighter.module_accessor, onemoreeff, 2.0);
                    EffectModule::set_rate(fighter.module_accessor, onemoreeff2, 2.0);
                    if SHADOW_FRENZY[entry_id(fighter.module_accessor)] {
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.6, 0.0, 1.0);
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff2, 0.6, 0.0, 1.0);
                    }
                    else if SP_GAUGE[entry_id(fighter.module_accessor)] >= 25.0 && SP_GAUGE[entry_id(fighter.module_accessor)] < 50.0 {
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 1.0, 1.0);
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff2, 0.0, 1.0, 1.0);
                    }
                    else if SP_GAUGE[entry_id(fighter.module_accessor)] >= 50.0 && SP_GAUGE[entry_id(fighter.module_accessor)] < 75.0 {
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 0.0, 1.0);
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff2, 0.0, 0.0, 1.0);
                    }
                    else if SP_GAUGE[entry_id(fighter.module_accessor)] >= 75.0 && SP_GAUGE[entry_id(fighter.module_accessor)] < 100.0 {
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 1.0, 0.1, 0.1);
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff2, 1.0, 0.1, 0.1);
                    }
                    else if SP_GAUGE[entry_id(fighter.module_accessor)] >= 100.0 && SP_GAUGE[entry_id(fighter.module_accessor)] < 125.0 {
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 1.0, 0.8, 0.0);
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff2, 1.0, 0.8, 0.0);
                    }
                    else if SP_GAUGE[entry_id(fighter.module_accessor)] >= 125.0 && SP_GAUGE[entry_id(fighter.module_accessor)] < 150.0 {
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 1.0, 0.0, 0.6);
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff2, 1.0, 0.0, 0.6);
                    }
                    else{
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 1.0, 1.0, 1.0);
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff2, 1.0, 1.0, 1.0);
                    }
                    _TIME_COUNTER[entry_id(fighter.module_accessor)] = 4;
                }
                _TIME_COUNTER[entry_id(fighter.module_accessor)] -= 1;
            }

            if EX_FLASH[entry_id(fighter.module_accessor)] > 0 {
                if EX_FLASH[entry_id(fighter.module_accessor)] % 10 == 0 {
                    if SHADOW_FRENZY[entry_id(fighter.module_accessor)] {
                        macros::FLASH(fighter, 0.4, 0.0, 1.0, 1.0);
                    }
                    else {
                        macros::FLASH(fighter, 1.0, 1.0, 0.0, 0.75);
                    }
                }
                else if EX_FLASH[entry_id(fighter.module_accessor)] % 5 == 0 {
                    macros::COL_NORMAL(fighter);
                }
                EX_FLASH[entry_id(fighter.module_accessor)] -= 1;
            }

            // Air Action Reset

            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            || StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_CLIFF {
                AIR_ACTION[entry_id(fighter.module_accessor)] = false;
            }

            // Shadow Frenzy Check

            if MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_l") || MotionModule::motion_kind(fighter.module_accessor) == hash40("appeal_hi_r") {
                if SP_GAUGE[entry_id(fighter.module_accessor)] == 100.0 && shadow_id(fighter.module_accessor) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                    SHADOW_FRENZY[entry_id(fighter.module_accessor)] = true;
                }
            }
            else if SP_GAUGE[entry_id(fighter.module_accessor)] == 0.0 {
                SHADOW_FRENZY[entry_id(fighter.module_accessor)] = false;
            }

            // Move Effects

            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT)
                || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
                    && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)
                    && get_command_stick_direction(fighter.module_accessor, true) == 6 {
                        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_AIR), true);
                        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_S.into(), false.into());
                    }
                    if QCB[entry_id(fighter.module_accessor)] == 3
                    && (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                    || ControlModule::check_button_on_release(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) {
                        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_DASH.into(), false.into());
                    }
                    if QCB[entry_id(fighter.module_accessor)] != 3
                    && (ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)
                    || ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL)) {
                        CancelModule::enable_cancel(fighter.module_accessor);
                    }
                }
            }

            if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("catch_wait")
                || StatusModule::status_kind(fighter.module_accessor) == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
                    fighter.change_status(FIGHTER_STATUS_KIND_THROW.into(), true.into());
                }
                if MotionModule::motion_kind(fighter.module_accessor) == hash40("throw_hi")
                && MotionModule::frame(fighter.module_accessor) > 22.0 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                }
            }
            if StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_WAIT
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_DASH
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_TURN
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_PULL
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_JUMP
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH_CUT
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_CATCH
            && CatchModule::is_catch(fighter.module_accessor) == false
            && StatusModule::status_kind(fighter.module_accessor) != *FIGHTER_STATUS_KIND_THROW {
                HEROIC_GRAB[entry_id(fighter.module_accessor)] = false;
            }

            // Jump Cancels

            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_hi3")
            || (MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_dash")
            && IS_FUNNY[entry_id(fighter.module_accessor)]) {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
                    && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    }
                }
            }
            
            if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_n")
            || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_hi")
            || MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_air_lw") {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                    if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP)
                    && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < 2 {
                        fighter.change_status(FIGHTER_STATUS_KIND_JUMP_AERIAL.into(), false.into());
                    }
                }
            }

            // Training Mode Tools

            if smashball::is_training_mode(){
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                    if SP_GAUGE[entry_id(fighter.module_accessor)] > 25.0 {
                        SP_GAUGE[entry_id(fighter.module_accessor)] -= 25.0
                    }
                    else {
                        SP_GAUGE[entry_id(fighter.module_accessor)] = 0.0;
                    }
                }
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    if SP_GAUGE[entry_id(fighter.module_accessor)] < SP_GAUGE_MAX[entry_id(fighter.module_accessor)] - 25.0 {
                        SP_GAUGE[entry_id(fighter.module_accessor)] += 25.0
                    }
                    else {
                        SP_GAUGE[entry_id(fighter.module_accessor)] = SP_GAUGE_MAX[entry_id(fighter.module_accessor)];
                    }
                }
                if ControlModule::check_button_on_trriger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) {
                    if TRAINING_TOOLS[entry_id(fighter.module_accessor)] {
                        TRAINING_TOOLS[entry_id(fighter.module_accessor)] = false;
                        let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                        let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                        let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 0.0, 0.0, 5.0);
                    }
                    else {
                        TRAINING_TOOLS[entry_id(fighter.module_accessor)] = true;
                        let pos = Vector3f{x: 0.0, y: 13.0, z: 0.0};
                        let rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
                        let onemoreeff: u32 = EffectModule::req_follow(fighter.module_accessor, smash::phx::Hash40{hash: hash40("sys_counter_flash")}, smash::phx::Hash40{hash: hash40("top")}, &pos, &rot, 1.0, false, 0, 0, 0, 0, 0, false, false) as u32;
                        EffectModule::set_rgb(fighter.module_accessor, onemoreeff, 5.0, 0.0, 0.0);
                    }
                }
            }
        }
    }
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_LOOP, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specialnloop(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        AIR_ACTION[entry_id(fighter.module_accessor)] = true;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_loop"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_loop"), 1.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialnloopmain as *const () as _))
}

unsafe extern "C" fn lucina_specialnloopmain(fighter: &mut L2CFighterCommon) -> L2CValue {
    PostureModule::add_pos_2d(fighter.module_accessor, &Vector2f{
        x: 1.6 * PostureModule::lr(fighter.module_accessor),
        y: 0.0
    });
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        EX_FLASH[entry_id(fighter.module_accessor)] = 40;
        IS_EX[entry_id(fighter.module_accessor)] = true;
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END_MAX.into(), false.into());
    }
    else if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_N_END.into(), false.into());
            }
            else {
                HEROIC_GRAB[entry_id(fighter.module_accessor)] = true;
                fighter.change_status(FIGHTER_STATUS_KIND_CATCH_DASH.into(), false.into());
            }
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_RESET);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        StatusModule::set_situation_kind(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), true);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
        START_SITUATION[entry_id(fighter.module_accessor)] = *SITUATION_KIND_AIR;
        AIR_ACTION[entry_id(fighter.module_accessor)] = true;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s1"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
        START_SITUATION[entry_id(fighter.module_accessor)] = *SITUATION_KIND_GROUND;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s1"), 1.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specialsmain as *const () as _))
}

unsafe extern "C" fn lucina_specialsmain(fighter: &mut L2CFighterCommon) -> L2CValue {
    if START_SITUATION[entry_id(fighter.module_accessor)] == *SITUATION_KIND_AIR
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
    }
    else if START_SITUATION[entry_id(fighter.module_accessor)] == *SITUATION_KIND_GROUND
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
    }
    else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        if CancelModule::is_enable_cancel(fighter.module_accessor) {
            fighter.sub_air_check_fall_common();
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE) {
            if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW)
            || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW))
            && ControlModule::get_stick_y(fighter.module_accessor) > -0.4 {
                fighter.change_status(FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2.into(), false.into());
            }
        }
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    L2CValue::I32(0)
}

#[status_script(agent = "lucina", status = FIGHTER_MARTH_STATUS_KIND_SPECIAL_S2, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn lucina_specials2(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK_RAW) {
        START_SITUATION[entry_id(fighter.module_accessor)] = 1;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
    }
    else {
        START_SITUATION[entry_id(fighter.module_accessor)] = 0;
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
    }   
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_specials2main as *const () as _))
}

unsafe extern "C" fn lucina_specials2main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        fighter.sub_wait_ground_check_common(L2CValue::I32(0));
        fighter.sub_air_check_fall_common();
    }
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            if START_SITUATION[entry_id(fighter.module_accessor)] == 0 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_hi"), 1.0, 1.0, false, 0.0, false, false);
            }
            if START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s2_lw"), 1.0, 1.0, false, 0.0, false, false);
            }
        }
    }
    else {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR
            && START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                if START_SITUATION[entry_id(fighter.module_accessor)] == 1 {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_SQUAT_WAIT.into(), false.into());
                }
            }
        }
    }
    L2CValue::I32(0)
}

// A back-handed punch. Yu can act out of this very early, allowing him to pressure with multiple single jabs.
// Deals 2.5 damage.

#[acmd_script( agent = "lucina", script = "game_attack11", category = ACMD_GAME, low_priority )]
unsafe fn lucina_jab1(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 15, 0, 20, 2.0, 0.0, 9.4, 6.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 15, 0, 20, 5.0, 0.0, 9.4, 8.8, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_attack11", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_jab1eff(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 18, -5, 0, 330, 90, 0.95, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -1, 10, 10, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

// A standing roundhouse kick. Jump-Cancellable on hit.
// Deals 4.5 damage, sends at a 50 degree angle.

#[acmd_script( agent = "lucina", script = "game_attack12", category = ACMD_GAME, low_priority )]
unsafe fn lucina_jab2(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 88, 0, 30, 4.2, 5.0, -1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 88, 0, 30, 3.8, -1.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_attack12", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_jab2eff(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -5, 12, 2, 0, 0, 0, 0.95, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), -6, 12, 17, 0, 0, 0, 1.15, 0, 0, 0, 0, 0, 360, true);
    }
}

// Deals 10 damage. Otherwise unchange from Lucina.

#[acmd_script( agent = "lucina", script = "game_attacks3", category = ACMD_GAME, low_priority )]
unsafe fn lucina_ftilt(fighter: &mut L2CAgentBase) {
    if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
        macros::FT_MOTION_RATE(fighter, 0.5);
        HEROIC_GRAB[entry_id(fighter.module_accessor)] = false;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 1.0, 0.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 1.0, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
}

// An upward slash that starts very low to the ground. Jump-Cancellable on hit.
// Deals 6 damage, sends at a 71 degree angle.

#[acmd_script( agent = "lucina", script = "game_attackhi3", category = ACMD_GAME, low_priority )]
unsafe fn lucina_utilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 1.5, 100, 60, 40, 50, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 1.5, 100, 60, 40, 50, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 5.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 75, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(0.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 5.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 75, 60, 0, 65, 2.0, 0.0, -1.0, 1.0, Some(1.0), Some(-1.0), Some(12.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Animation is unchanged from Lucina.
// Deals 6 damage, sends at a 75 degree angle.

#[acmd_script( agent = "lucina", script = "game_attacklw3", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dtilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 71, 40, 0, 57, 2.7, 0.0, 2.7, 16.700001, Some(0.0), Some(4.3), Some(9.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 6.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 71, 40, 0, 57, 2.7, 0.0, 0.0, 8.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// A sliding move inspired by Swift Strike.
// Deals 7 damage, sends at a 65 degree angle, and can be EX'd.
// The EX version is Jump-Cancellable on hit.

#[acmd_script( agent = "lucina", script = "game_attackdash", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dashattack(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 65, 85, 0, 65, 3.6, 5.0, -1.0, 1.5, Some(1.5), Some(-1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 65, 85, 0, 65, 2.5, 0.0, 2.5, -2.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.2, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_XLU);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "lucina", script = "sound_attackdash", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_dashattacksound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_ll"));
    }
}

// A fast, horizontal kick. Jump-Cancellable on hit.
// Deals 4.5 damage, has 40 Base Knockback, 100 Kockback Growth, and is actionable on frame 23.

#[acmd_script( agent = "lucina", script = "game_attackairn", category = ACMD_GAME, low_priority )]
unsafe fn lucina_nair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    macros::FT_MOTION_RATE(fighter, 0.0833333);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 55, 60, 0, 40, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 55, 60, 0, 40, 5.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 55, 60, 0, 40, 3.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 60, 0, 40, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 4, 0, Hash40::new("kneer"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 60, 0, 40, 5.0, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 5, 0, Hash40::new("kneel"), 4.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 60, 0, 40, 3.0, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "effect_attackairn", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_naireff(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), -2, 9.3, -5.5, 0, 10, 0, 1, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 11, 9.3, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false, 0.7);
    }
}

#[acmd_script( agent = "lucina", script = "sound_attackairn", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_nairsnd(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_m"));
    }
}

// Forward and Back Air have been made pretty much identical.
// Deals 10 damage and has 74 Knockback Growth.

#[acmd_script( agent = "lucina", script = "game_attackairf", category = ACMD_GAME, low_priority )]
unsafe fn lucina_fair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.0, 1.0, 0.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.0, 1.0, 0.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "lucina", script = "game_attackairb", category = ACMD_GAME, low_priority )]
unsafe fn lucina_bair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::REVERSE_LR(fighter);
    }
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 0.0, 0.0, 3.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 75, 0, 50, 3.5, 0.0, 0.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Animation is unchanged from Lucina. Jump-Cancellable on hit.
// Deals 8 damage, sends at a 55 degree angle, and has 40 Knockback Growth.

#[acmd_script( agent = "lucina", script = "game_attackairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 55, 40, 0, 40, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 55, 40, 0, 40, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("colonells"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 55, 40, 0, 40, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 55, 40, 0, 40, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Animation is unchanged from Lucina. Jump-Cancellable on hit.
// Deals 8 damage, sends at an 80 degree angle, and has 8 frames of landing lag, so you can combo if you land with it.

#[acmd_script( agent = "lucina", script = "game_attackairlw", category = ACMD_GAME, low_priority )]
unsafe fn lucina_dair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 73, 75, 0, 30, 3.0, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 73, 75, 0, 30, 3.5, 1.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 73, 75, 0, 30, 3.5, 1.0, 0.0, 6.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

// Animation borrowed from Ike Forward Tilt.
// Deals 13.5 damage and will steal your soul at ranges you never thought possible.

#[acmd_script( agent = "lucina", script = "game_attacks4", category = ACMD_GAME, low_priority )]
unsafe fn lucina_fsmash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.5 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 85, 0, 30, 4.0, 0.0, 8.5, 10.0, Some(0.0), Some(3.5), Some(22.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

// Animation is unchanged from Lucina.
// The sweetspot deals 12 damage. Otherwise unchanged.

#[acmd_script( agent = "lucina", script = "game_attackhi4", category = ACMD_GAME, low_priority )]
unsafe fn lucina_usmash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 89, 90, 0, 42, 5.8, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 89, 90, 0, 42, 4.6, 0.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 89, 90, 0, 42, 5.8, 0.0, 0.0, -3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 125, 100, 155, 0, 5.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(-9.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "game_throwhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 60, 40, 20, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        else {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 93, 102, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        if HEROIC_GRAB[entry_id(fighter.module_accessor)] {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 1, 21);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        if HEROIC_GRAB[entry_id(fighter.module_accessor)] == false {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "game_specialnloop", "game_specialairnloop" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_nspecialloop(fighter: &mut L2CAgentBase) {
    macros::FT_MOTION_RATE(fighter, 1.4);
}

// Based off of Heroic Bravery. Borrows Chrom's Forward Smash animation.
// Kirby gets this as well, though it uses the old Shield Breaker animation.
// Uncharged it simply deals a bit of damage and launches away...

#[acmd_script( agent = "lucina", scripts = [ "game_specialnend", "game_specialnendhi", "game_specialnendlw", "game_specialairnend", "game_specialairnendhi", "game_specialairnendlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_nspecialend(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 20, 0, 95, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 20, 0, 95, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 20, 0, 95, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 361, 20, 0, 95, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 0, false);
            AttackModule::clear(fighter.module_accessor, 1, false);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else {
        sv_animcmd::frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.125);
        sv_animcmd::frame(fighter.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        sv_animcmd::frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 20, 0, 95, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 361, 20, 0, 95, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0, 361, 20, 0, 95, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

// ... while fully-charged it will ignore shields, paralyze on hit, and allow Yu to combo without using One More!.

#[acmd_script( agent = "lucina", scripts = [ "game_specialnendmax", "game_specialnendmaxhi", "game_specialnendmaxlw", "game_specialairnendmax", "game_specialairnendmaxhi", "game_specialairnendmaxlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_nspecialendmax(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            if PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) < -0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            else if PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 60, 80, 0, 30, 3.0, 0.0, 5.0, 8.0, Some(0.0), Some(5.0), Some(20.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 60, 80, 0, 30, 3.0, 0.0, 5.0, 25.0, Some(0.0), Some(5.0), Some(27.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 60, 80, 0, 30, 0.9, 0.0, 5.0, 17.0, Some(0.0), Some(5.0), Some(22.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 60, 80, 0, 30, 0.9, 0.0, 5.0, 23.5, Some(0.0), Some(5.0), Some(28.9), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 0, false);
            AttackModule::clear(fighter.module_accessor, 1, false);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        sv_animcmd::frame(fighter.lua_state_agent, 1.0);
        macros::FT_MOTION_RATE(fighter, 0.125);
        sv_animcmd::frame(fighter.lua_state_agent, 9.0);
        macros::FT_MOTION_RATE(fighter, 2.0);
        sv_animcmd::frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            if PostureModule::lr(fighter.module_accessor) == 1.0 && ControlModule::get_stick_x(fighter.module_accessor) < -0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            else if PostureModule::lr(fighter.module_accessor) == -1.0 && ControlModule::get_stick_x(fighter.module_accessor) > 0.75 {
                PostureModule::reverse_lr(fighter.module_accessor);
            }
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 60, 80, 0, 30, 3.3, 0.0, 7.7, 9.1, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.0, 60, 80, 0, 30, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.0, 60, 80, 0, 30, 3.5, 2.0, 1.0, 7.5, Some(9.5), Some(1.0), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 17.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
}

// Based off of Lightning Flash. Borrows Lucina's old Neutral Special animation. Can be EX'd.
// Yu lunges forward with a very active hitbox on his sword. Launches opponents away.
// The EX version starts up faster and deals more damage for slightly less knockback.

#[acmd_script( agent = "lucina", script = "game_specials1", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial1(fighter: &mut L2CAgentBase) {
    let mut dmg : f32;
    let kbg : i32;
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0/3.0);
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        EX_FLASH[entry_id(fighter.module_accessor)] = 40;
        IS_EX[entry_id(fighter.module_accessor)] = true;
        macros::FT_MOTION_RATE(fighter, 0.2);
    }
    else {
        IS_EX[entry_id(fighter.module_accessor)] = false;
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
    }
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 367, 30, 20, 40, 5.0, 0.0, 7.0, 1.0, Some(0.0), Some(7.0), Some(7.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 367, 30, 20, 40, 5.0, 0.0, 7.0, -5.0, Some(0.0), Some(7.0), Some(7.0), 0.4, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        if IS_EX[entry_id(fighter.module_accessor)] == true {
            dmg = 13.0;
            kbg = 45;
        }
        else {
            dmg = 9.0;
            kbg = 66;
        }
        dmg *= DMG_RATIO[entry_id(fighter.module_accessor)];
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 45, kbg, 0, 60, 2.5, 0.0, 8.5, 0.0, Some(0.0), Some(8.5), Some(20.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg, 45, kbg, 0, 60, 2.5, 0.0, 8.5, 25.0, Some(0.0), Some(8.5), Some(27.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specials1", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial1eff(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1388f0ac45), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1475cf263f), Hash40::new("top"), -0.0, 11.0, 12, 0, 0, 0, 1.2, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 9.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
	    macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 18, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new_raw(0x1388f0ac45), false, true);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specials1", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_sspecial1snd(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_jump02"));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucina_special_n01"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_n04"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specials1", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_sspecial1exp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit11"), 0, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_piercel"), 0);
    }
}

// An original move, due to Smash's restrictions, dubbed "Lion's Leap." Borrows Corrin's Dragon Lunge jump animation.
// Yu jumps forward with forward momentum. Can only be used once in the air, and is usable again when Yu touches the ground or grabs a ledge.

#[acmd_script( agent = "lucina", script = "game_specialairs1", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial1air(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    macros::FT_MOTION_RATE(fighter, 24.0/35.0);
    sv_animcmd::frame(fighter.lua_state_agent, 52.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

// Hold Down and press Special during Lion's Leap to perform a divekick. Borrows Zero Suit Samus's down air animation.
// Can be EX'd for more damage.

#[acmd_script( agent = "lucina", script = "game_specialairs2lw", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2lwair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    macros::FT_MOTION_RATE(fighter, 0.1);
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        EX_FLASH[entry_id(fighter.module_accessor)] = 40;
        IS_EX[entry_id(fighter.module_accessor)] = true;
    }
    else {
        IS_EX[entry_id(fighter.module_accessor)] = false;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        let dmg : f32;
        let angle : u64;
        let velx : f32;
        let vely : f32;
        if IS_EX[entry_id(fighter.module_accessor)] == true {
            dmg = 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)];
            angle = 290;
            velx = 1.75;
            vely = -3.5;
        }
        else {
            dmg = 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)];
            angle = 40;
            velx = 1.5;
            vely = -3.0;
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), dmg, angle, 90, 0, 60, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), dmg, angle, 90, 0, 60, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), dmg, 40, 60, 0, 60, 6.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("kneel"), dmg, 40, 60, 0, 60, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 50.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

// Hold Up and press Special during Lion's Leap to perform Raging Lion. Can be EX'd for more damage.

#[acmd_script( agent = "lucina", script = "game_specialairs2hi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2hiair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
        && spent_meter(fighter.module_accessor, false) {
            SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
            EX_FLASH[entry_id(fighter.module_accessor)] = 40;
            IS_EX[entry_id(fighter.module_accessor)] = true;
        }
        else {
            IS_EX[entry_id(fighter.module_accessor)] = false;
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        let dmg : f32;
        let kbg : i32;
        let velx : f32;
        let vely : f32;
        if IS_EX[entry_id(fighter.module_accessor)] == true {
            dmg = 16.0 * DMG_RATIO[entry_id(fighter.module_accessor)];
            kbg = 90;
            velx = 3.0;
            vely = -3.0;
        }
        else {
            dmg = 12.0 * DMG_RATIO[entry_id(fighter.module_accessor)];
            kbg = 66;
            velx = 2.1;
            vely = -2.1;
        }
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, velx, vely, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), dmg, 361, kbg, 0, 45, 2.5, 0.0, 8.5, 4.0, Some(0.0), Some(4.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), dmg - 3.0, 361, kbg - 10, 0, 45, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(11.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), dmg - 3.0, 361, kbg - 10, 0, 45, 2.5, 0.0, 2.5, 3.0, Some(0.0), Some(2.5), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specialairs2hi", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial2hiaireff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1388f0ac45), Hash40::new("haver"), -0.0, 0, 0, 0, 0, 0, 1, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new_raw(0x1475cf263f), Hash40::new("top"), -0.0, 6, 8, 40, 0, 0, 0.9, true);
        macros::EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 40, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 0.6, true);
	    macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specialairs2hi", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_sspecial2hiairsnd(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_n03"));
    }
}

#[acmd_script( agent = "lucina", script = "game_specials2lw", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2lw(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specials2lw", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial2lweff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specials2lw", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_sspecial2lwsnd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_lucina_landing02"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specials2lw", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_sspecial2lwexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

#[acmd_script( agent = "lucina", script = "game_specials2hi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_sspecial2hi(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "lucina", script = "effect_specials2hi", category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_sspecial2hieff(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script( agent = "lucina", script = "sound_specials2hi", category = ACMD_SOUND, low_priority )]
unsafe fn lucina_sspecial2hisnd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_lucina_landing02"));
    }
}

#[acmd_script( agent = "lucina", script = "expression_specials2hi", category = ACMD_EXPRESSION, low_priority )]
unsafe fn lucina_sspecial2hiexp(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
	    macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_impact"), 0, false, 0);
    }
}

// The following two scripts are used for Big Gamble. Can be EX'd.
// The normal version launches opponents nearly straight up.
// The EX version is fully invincible on frame 5 on and is a proper multi-hit,
// sending opponents up. The EX version can NOT be cancelled into One More!.

#[acmd_script( agent = "lucina", script = "game_specialhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uspecial(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        upper_invuln(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        EX_FLASH[entry_id(fighter.module_accessor)] = 60;
        IS_EX[entry_id(fighter.module_accessor)] = true;
    }
    else {
        IS_EX[entry_id(fighter.module_accessor)] = false;
    }
    if IS_EX[entry_id(fighter.module_accessor)] {
        upper_invuln(fighter.module_accessor, false);
        full_invuln(fighter.module_accessor, true);
        sv_animcmd::frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 87, 100, 160, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 367, 100, 85, 0, 6.0, 0.0, 20.0, 10.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 367, 100, 85, 0, 5.3, 0.0, 0.0, 10.0, Some(0.0), Some(17.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 40, 85, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            full_invuln(fighter.module_accessor, false);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
        sv_animcmd::frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        sv_animcmd::frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 3.5);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 60, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 60, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            upper_invuln(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 50, 0, 50, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 50, 0, 50, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 7.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 2, false);
            AttackModule::clear(fighter.module_accessor, 3, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

#[acmd_script( agent = "lucina", script = "game_specialairhi", category = ACMD_GAME, low_priority )]
unsafe fn lucina_uspecialair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 2.0);
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)
    && spent_meter(fighter.module_accessor, false) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        EX_FLASH[entry_id(fighter.module_accessor)] = 60;
        IS_EX[entry_id(fighter.module_accessor)] = true;
    }
    else {
        IS_EX[entry_id(fighter.module_accessor)] = false;
    }
    if IS_EX[entry_id(fighter.module_accessor)] {
        full_invuln(fighter.module_accessor, true);
        sv_animcmd::frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 87, 100, 140, 0, 5.1, 0.0, 11.0, 10.0, Some(0.0), Some(7.0), Some(10.0), 1.8, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.1, 367, 50, 10, 0, 6.0, 0.0, 20.0, 10.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.1, 367, 50, 10, 0, 5.3, 0.0, 0.0, 10.0, Some(0.0), Some(17.0), Some(10.0), 1.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 40, 85, 0, 75, 6.5, 0.0, 12.0, 10.0, Some(0.0), Some(21.0), Some(10.0), 2.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            full_invuln(fighter.module_accessor, false);
        }
        macros::FT_MOTION_RATE(fighter, 1.0);
        sv_animcmd::frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else{
        upper_invuln(fighter.module_accessor, true);
        sv_animcmd::frame(fighter.lua_state_agent, 5.0);
        macros::FT_MOTION_RATE(fighter, 3.5);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 60, 0, 70, 4.0, 0.0, 8.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 60, 0, 70, 4.0, 0.0, 8.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_SPECIAL_HI_SET_LR);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_HI_FLAG_TRANS_MOVE);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            upper_invuln(fighter.module_accessor, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 50, 0, 50, 5.0, 0.0, 0.0, 4.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 7.0 * DMG_RATIO[entry_id(fighter.module_accessor)], 79, 50, 0, 50, 5.0, 0.0, 0.0, -1.5, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 7.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 2, false);
            AttackModule::clear(fighter.module_accessor, 3, false);
            smash_script::notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

// Counter was changed to One More!, though unlike Persona 4 Arena's One More!, this one works more like Guilty Gear's Roman Cancel.
// For the first part of the move, everything on the screen slows down as Yu prepares to, uh, Roman.
// Yu will then strike a pose, creating a large hitbox around him.
// If something gets hit by this hitbox, things will stay slow for another 30 frames,
// allowing Yu to follow-up with whatever he would like.

#[acmd_script( agent = "lucina", scripts = [ "game_speciallw", "game_specialairlw" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_dspecial(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        SP_GAUGE[entry_id(fighter.module_accessor)] -= SPENT_SP[entry_id(fighter.module_accessor)];
        JostleModule::set_status(fighter.module_accessor, false);
        KineticModule::unable_energy_all(fighter.module_accessor);
        if ROMAN_ON_HIT[entry_id(fighter.module_accessor)] {
            AIR_ACTION[entry_id(fighter.module_accessor)] = false;
            macros::SLOW_OPPONENT(fighter, 50.0, 19.0);
            full_invuln(fighter.module_accessor, true);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        let dir = get_command_stick_direction(fighter.module_accessor, false);
        if dir == 5
        || dir == 8
        || dir == 2 {
            ROMAN_MOVE[entry_id(fighter.module_accessor)] = 0.0;
        }
        else if dir == 4
        || dir == 7
        || dir == 1 {
            ROMAN_MOVE[entry_id(fighter.module_accessor)] = -2.0;
        }
        else {
            ROMAN_MOVE[entry_id(fighter.module_accessor)] = 2.0;
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 40.0, 0.0, 10.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 1, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] = true;
    }
    macros::FT_MOTION_RATE(fighter, 5.0);
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    if macros::is_excute(fighter) {
        if ROMAN_ON_HIT[entry_id(fighter.module_accessor)] {
            IS_ROMAN_MOVE[entry_id(fighter.module_accessor)] = false;
            full_invuln(fighter.module_accessor, false);
        }
    }
}

#[acmd_script( agent = "lucina", scripts = [ "effect_speciallw", "effect_specialairlw" ], category = ACMD_EFFECT, low_priority )]
unsafe fn lucina_dspecialeff(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new_raw(0x144a0746f9), Hash40::new("top"), 0, 10, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.3);
    }
}

#[acmd_script( agent = "lucina", scripts = [ "sound_speciallw", "sound_specialairlw" ], category = ACMD_SOUND, low_priority )]
unsafe fn lucina_dspecialsnd(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_special_l01"));
    }
}

// If you press Up Taunt as Shadow Yu at 100 SP, Shadow Yu activates Shadow Frenzy.
// Shadow Yu performs a quick slash that can be cancelled out of very quickly.
// Upon use, his SP will slowly drain to 0, but during Shadow Frenzy,
// EX moves and One More! are performed for less SP.

#[acmd_script( agent = "lucina", scripts = [ "game_speciallwhit", "game_specialairlwhit" ], category = ACMD_GAME, low_priority )]
unsafe fn lucina_dspecialhit(fighter: &mut L2CAgentBase) {
    if shadow_id(fighter.module_accessor) {
        macros::FT_START_CUTIN(fighter);
        macros::SLOW_OPPONENT(fighter, 20.0, 8.0);
        SHADOW_FRENZY[entry_id(fighter.module_accessor)] = true;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword1"), 3.4, 90, 0, 60, 40, 5.0, 1.0, 0.0, 1.5, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.4, 90, 0, 60, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("colonells"), 3.4, 90, 0, 60, 40, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 3.4, 90, 0, 60, 40, 5.0, 1.0, 0.0, 7.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 2, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 3, true, false);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_SPECIAL_EFFECT) {
            AttackModule::set_optional_hit_effect(fighter.module_accessor, 0, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(fighter.module_accessor, 1, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(fighter.module_accessor, 2, Hash40::new("se_lucina_criticalhit"));
            AttackModule::set_optional_hit_effect(fighter.module_accessor, 3, Hash40::new("se_lucina_criticalhit"));
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        lucina_frame
    );
    smashline::install_status_scripts!(
        lucina_specialnloop,
        lucina_specials,
        lucina_specials2
    );
    smashline::install_acmd_scripts!(
        lucina_jab1,
        lucina_jab1eff,
        lucina_jab2,
        lucina_jab2eff,
        lucina_ftilt,
        lucina_utilt,
        lucina_dtilt,
        lucina_dashattack,
        lucina_dashattacksound,
        lucina_nair,
        lucina_naireff,
        lucina_nairsnd,
        lucina_fair,
        lucina_bair,
        lucina_uair,
        lucina_dair,
        lucina_fsmash,
        lucina_usmash,
        lucina_uthrow,
        lucina_nspecialloop,
        lucina_nspecialend,
        lucina_nspecialendmax,
        lucina_sspecial1,
        lucina_sspecial1eff,
        lucina_sspecial1snd,
        lucina_sspecial1exp,
        lucina_sspecial1air,
        lucina_sspecial2lwair,
        lucina_sspecial2hiair,
        lucina_sspecial2hiaireff,
        lucina_sspecial2hiairsnd,
        lucina_sspecial2lw,
        lucina_sspecial2lweff,
        lucina_sspecial2lwsnd,
        lucina_sspecial2lwexp,
        lucina_sspecial2hi,
        lucina_sspecial2hieff,
        lucina_sspecial2hisnd,
        lucina_sspecial2hiexp,
        lucina_uspecial,
        lucina_uspecialair,
        lucina_dspecial,
        lucina_dspecialeff,
        lucina_dspecialsnd,
        lucina_dspecialhit
    );
    // skyline::install_hook!(lucina_is_enable_transition_term_replace);
}