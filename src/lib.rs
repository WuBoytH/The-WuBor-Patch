#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

// use smash::phx::*;
use smash::hash40;
// use smash::phx::Hash40;
use smash::lib::L2CValue;
use smash::lib::L2CAgent;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::app::FighterManager;
use skyline::hooks::{getRegionAddress, Region};
// use skyline::nn::ro::LookupSymbol;

static mut INT_OFFSET : usize = 0x4E19D0;
// static mut INT64_OFFSET : usize = 0x4E19F0;
static mut FLOAT_OFFSET : usize = 0x4E19D0;
static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
// static mut MUSIC_OFFSET: usize = 0x3451f30; // default = 8.1.0 offset
// static mut MUSIC_PARAM1: *mut u64 = 0 as *mut u64;
// static mut MUSIC_PARAM2: i64 = 0;
// static mut NUS3AUDIO_HASH: *mut u64 = 0 as *mut u64;
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut FIGHTER_MANAGER: usize = 0;
pub static mut ITEM_MANAGER: usize = 0;

static YU_AUDIO: [&'static str; 36] = ["appeal01", "appeal02", "attack01", "attack02", "attack03", "attack04", "attack05", "attack06", "attack07", "cliffcatch", "damage_twinkle", "damage01", "damage02", "damage03", "damagefly01", "damagefly02", "final", "furafura", "furasleep", "heavyget", "jump01", "missfoot01", "missfoot02", "ottotto", "passive", "special_h01", "special_l01", "special_l02", "special_n01", "swimup", "win01", "win02", "win03", "win_marth", "win_ike", "knockout"];
static YU_SEQ: [&'static str; 8] = ["attack", "special_n", "special_l", "special_h", "futtobi01", "futtobi02", "jump", "ottotto"];

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

// static INT64_SEARCH_CODE: &[u8] = &[
//     0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x15, 0x40, 0xf9,
// ];

static NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE: &[u8] = &[
    0xff, 0x03, 0x03, 0xd1,
    0xe8, 0x2b, 0x00, 0xfd,
    0xfc, 0x6f, 0x06, 0xa9,
    0xfa, 0x67, 0x07, 0xa9,
    0xf8, 0x5f, 0x08, 0xa9,
    0xf6, 0x57, 0x09, 0xa9,
    0xf4, 0x4f, 0x0a, 0xa9,
    0xfd, 0x7b, 0x0b, 0xa9,
    0xfd, 0xc3, 0x02, 0x91,
    0xfb, 0x03, 0x00, 0xaa
];

// static MUSIC_SEARCH_CODE: &[u8] = &[
//     0xfc, 0x6f, 0xba, 0xa9, 0xfa, 0x67, 0x01, 0xa9, 0xf8, 0x5f, 0x02, 0xa9, 0xf6, 0x57, 0x03, 0xa9,
//     0xf4, 0x4f, 0x04, 0xa9, 0xfd, 0x7b, 0x05, 0xa9, 0xfd, 0x43, 0x01, 0x91, 0xff, 0xc3, 0x1b, 0xd1,
//     0xe8, 0x63, 0x05, 0x91,
// ];

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

mod commonfuncs;
use crate::commonfuncs::*;
mod globals;
mod system;
use crate::system::{IS_FUNNY/*, IS_FGC*/, COUNTER_HIT_STATE, OPPONENT_BOMA, AIR_WHIFF};
mod daisy;
mod samusd;
mod lucina;
use crate::lucina::{AIR_ACTION, HEROIC_GRAB, shadow_id};
mod littlemac;
mod gaogaen;
use crate::gaogaen::REVENGE;
mod dedede;
mod lucas;
mod jack;
mod kirby;
use crate::kirby::SLIDE_BOUNCE;
mod cloud;
mod lucario;
use crate::lucario::IS_SPIRIT_BOMB;
mod bayonetta;
mod dolly;
// use crate::dolly::GO_SAUCE;
mod shulk;
use crate::shulk::SHULK_SPECIAL_LW;
mod pikachu;
mod robot;
mod snake;
mod palutena;
mod master;
mod ryu;
use crate::ryu::{SECRET_SENSATION, SEC_SEN_STATE, OPPONENT_X, OPPONENT_Y, CAMERA};
mod toonlink;
mod zelda;
mod buddy;
mod ridley;
use crate::ridley::FUNNY_RIDLEY;
mod koopajr;
mod gamewatch;
mod donkey;
mod richter;
use crate::richter::RICHTER_SPECIAL_HI;
mod eflame;
mod elight;
mod falco;
// mod brave;
mod purin;
// mod wiifit;
// use crate::wiifit::CAN_DRAGON_INSTALL;
mod ken;
use crate::ken::{QUICK_STEP_STATE, V_SHIFT, V_GAUGE, V_TRIGGER, SHORYUREPPA, TATSULOOPS};
mod metaknight;
mod ganon;
use crate::ganon::CAN_TELEPORT;
mod wario;
use crate::wario::FINISH_SIGN;
mod luigi;
mod reflet;
mod miiswordsman;
mod edge;
// use crate::edge::ONE_WING;
mod pickel;
mod pitb;
mod diddy;
mod sheik;
mod tantan;
mod yoshi;
mod mario;
mod duckhunt;
mod rockman;
mod kamui;
mod samus;
mod mariod;

// An unused experiment to make the Grab button work as a Smash Attack button.

// #[skyline::hook(replace = ControlModule::get_command_flag_cat )]
// pub unsafe fn get_command_flag_cat_replace(boma: &mut BattleObjectModuleAccessor, category: i32) -> i32 {
//     let mut flag = original!()(boma, category);
//     let entry_id = WorkModule::get_int(boma,*FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
//     let fighter_manager = *(FIGHTER_MANAGER as *mut *mut smash::app::FighterManager);
//     if smash::app::lua_bind::FighterInformation::is_operation_cpu(smash::app::lua_bind::FighterManager::get_fighter_information(fighter_manager, smash::app::FighterEntryID(entry_id as i32))) == false
//     && flag & *FIGHTER_PAD_CMD_CAT1_FLAG_CATCH != 0
//     && ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_CATCH) {
//         let stickx = ControlModule::get_stick_x(boma);
//         let sticky = ControlModule::get_stick_y(boma);
//         if stickx.abs() < 0.5 && sticky >= 0.5 {
//             flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4;
//         }
//         else if stickx.abs() < 0.5 && sticky <= -0.5 {
//             flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4;
//         }
//         else {
//             flag = *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4;
//         }
//     }
//     return flag;
// }

#[skyline::hook(offset = NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET)]
pub unsafe fn notify_log_event_collision_hit_replace(
fighter_manager: &mut FighterManager,
attacker_object_id: u32,
defender_object_id: u32, 
move_type: f32,
arg5: i32,
move_type_again: bool) -> u64 {
    let attacker_boma = sv_battle_object::module_accessor(attacker_object_id);
    let defender_boma = sv_battle_object::module_accessor(defender_object_id);
    let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let d_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

        // Used for Ken's meter building, as well as sets the opponent he will track towards with V-Trigger.

        if attacker_fighter_kind == *FIGHTER_KIND_KEN
        && a_entry_id < 8 {
            if d_entry_id < 8
            && utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                OPPONENT_BOMA[a_entry_id] = (&mut *defender_boma as *mut BattleObjectModuleAccessor) as u64;
            }
            else {
                OPPONENT_BOMA[a_entry_id] = 0;
            }
            if MotionModule::motion_kind(attacker_boma) != hash40("special_lw")
            && V_TRIGGER[a_entry_id] == false {
                if MotionModule::motion_kind(attacker_boma) == hash40("attack_s3_s_w")
                && StatusModule::status_kind(attacker_boma) == *FIGHTER_STATUS_KIND_SPECIAL_LW {
                    V_GAUGE[a_entry_id] += 100;
                }
                else if d_entry_id < 8 {
                    if COUNTER_HIT_STATE[d_entry_id] == 1 {
                        V_GAUGE[a_entry_id] += AttackModule::get_power(attacker_boma, 0, false, 1.0, false) as i32 * 6;
                    }
                }
                else {
                    V_GAUGE[a_entry_id] += AttackModule::get_power(attacker_boma, 0, false, 1.0, false) as i32 * 4;
                }
                if V_GAUGE[a_entry_id] > 900 {
                    V_GAUGE[a_entry_id] = 900;
                }
            }
            else {
                OPPONENT_BOMA[a_entry_id] = 0;
            }
        }
    }
    if utility::get_category(&mut *defender_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

        // Used for Ultra Instinct's tracking.

        if defender_fighter_kind == *FIGHTER_KIND_RYU
        && d_entry_id < 8 {
            if SEC_SEN_STATE[d_entry_id] {
                if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
                || utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY {
                    OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut BattleObjectModuleAccessor) as u64;
                    OPPONENT_X[d_entry_id] = PostureModule::pos_x(attacker_boma);
                    OPPONENT_Y[d_entry_id] = PostureModule::pos_y(attacker_boma);
                    if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        JostleModule::set_status(&mut *attacker_boma, false);
                    }
                }
                else if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                    let oboma = sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
                    if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                        OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                        OPPONENT_BOMA[d_entry_id] = (&mut *defender_boma as *mut BattleObjectModuleAccessor) as u64;
                    }
                    else {
                        OPPONENT_X[d_entry_id] = PostureModule::pos_x(oboma);
                        OPPONENT_Y[d_entry_id] = PostureModule::pos_y(oboma);
                        OPPONENT_BOMA[d_entry_id] = (&mut *oboma as *mut BattleObjectModuleAccessor) as u64;
                        if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            JostleModule::set_status(&mut *oboma, false);
                        }
                    }
                }
                else {
                    OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                    OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                    OPPONENT_BOMA[d_entry_id] = (&mut *defender_boma as *mut BattleObjectModuleAccessor) as u64;
                }
                SECRET_SENSATION[d_entry_id] = true;
            }
        }

        // Used to detect if Ken was hit during V-Shift's startup.

        else if defender_fighter_kind == *FIGHTER_KIND_KEN
        && d_entry_id < 8 {
            if MotionModule::motion_kind(defender_boma) == hash40("special_lw_step_b")
            && MotionModule::frame(defender_boma) <= 8.75 {
                V_SHIFT[d_entry_id] = true;
            }
        }

        // Used to detect if Incineroar was hit during Revenge's counter frames, as well as turning Incineroar to the correct direction.

        else if defender_fighter_kind == *FIGHTER_KIND_GAOGAEN
        && d_entry_id < 8 {
            if (MotionModule::motion_kind(defender_boma) == hash40("special_lw_start")
            || MotionModule::motion_kind(defender_boma) == hash40("special_air_lw_start"))
            && MotionModule::frame(defender_boma) >= 8.0
            && MotionModule::frame(defender_boma) <= 27.0 {
                REVENGE[d_entry_id] = 2;
                if PostureModule::pos_x(defender_boma) < PostureModule::pos_x(attacker_boma)
                && PostureModule::lr(defender_boma) == 1.0 {
                    PostureModule::reverse_lr(defender_boma);
                }
                else if PostureModule::pos_x(defender_boma) > PostureModule::pos_x(attacker_boma)
                && PostureModule::lr(defender_boma) == -1.0 {
                    PostureModule::reverse_lr(defender_boma);
                }
                StatusModule::change_status_request_from_script(defender_boma, *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_S_LARIAT, true);
            }
        }

        // Used to make sure Shulk is facing the right direction for Vision Burst.

        else if defender_fighter_kind == *FIGHTER_KIND_SHULK
        && d_entry_id < 8 {
            if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
            || utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY {
                OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut BattleObjectModuleAccessor) as u64;
            }
            else if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let oboma = sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
                if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    OPPONENT_BOMA[d_entry_id] = (&mut *oboma as *mut BattleObjectModuleAccessor) as u64;
                }
            }
            else {
                OPPONENT_BOMA[d_entry_id] = 0;
            }
        }
    }
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

// #[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
// unsafe fn attack_replace(lua_state: u64) {
//     let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
//     let fighter_kind = smash::app::utility::get_kind(boma);
//     let mut l2c_agent = L2CAgent::new(lua_state);
//     let mut hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
//     l2c_agent.clear_lua_stack();
//     for i in 0..36 {
//         if i == 21 && hitbox_params[i].get_f32() >= 0.0 {
//             l2c_agent.push_lua_stack(&mut L2CValue::new_num(1.0));
//         } else {
//             l2c_agent.push_lua_stack(&mut hitbox_params[i]);
//         }
//     }
//     original!()(lua_state);
// }

#[skyline::hook(replace = WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(boma: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = utility::get_kind(boma);
    let ret = original!()(boma,term);
    
    // Global Edits

    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

        // Fighter-Specific Param Edits
        
        if fighter_kind == *FIGHTER_KIND_ROCKMAN && entry_id(boma) < 8 {
            if IS_FUNNY[entry_id(boma)] {
                if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_RUN_BRAKE {
                    return false;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_LUCINA && entry_id(boma) < 8 {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                return false;
            }
            if HEROIC_GRAB[entry_id(boma)]
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_THROW_HI
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3
            && term != *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN {
                return false;
            }
            if AIR_ACTION[entry_id(boma)] && IS_FUNNY[entry_id(boma)] == false {
                if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S { // Disable Lion's Leap and Heroic Bravery if used once unless in Funny
                    return false;
                }
                else {
                    return ret;
                }
            }
            if MotionModule::motion_kind(boma) == hash40("attack_12") {
                if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_DASH
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON
                || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT {
                    return false;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_SHULK && entry_id(boma) < 8 {
            if SHULK_SPECIAL_LW[entry_id(boma)] && IS_FUNNY[entry_id(boma)] == false { // Disable Vision if used Burst and not in Funny
                if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                    return false;
                }
                else {
                    return ret;
                }
            }
            
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_RICHTER && entry_id(boma) < 8 {
            if RICHTER_SPECIAL_HI[entry_id(boma)] && IS_FUNNY[entry_id(boma)] == false { // Disable Multiple Up-Bs unless in Funny
                if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
                    return false;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_RYU && entry_id(boma) < 8 {
            if CAMERA[entry_id(boma)] {
                return false;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_KIRBY && entry_id(boma) < 8 {
            if SLIDE_BOUNCE[entry_id(boma)] {
                return false;
            }
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
                if CAN_TELEPORT[entry_id(boma)] == false {
                    return false;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_KEN && entry_id(boma) < 8 {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
                return (StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_LW) && (QUICK_STEP_STATE[entry_id(boma)] != 2);
            }
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
            || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK
            || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH
            || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN
            || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN
            || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH
            || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START
            || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT {
                if QUICK_STEP_STATE[entry_id(boma)] == 1 {
                    return false;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        // if fighter_kind == *FIGHTER_KIND_WIIFIT && entry_id(boma) < 8 {
        //     if CAN_DRAGON_INSTALL[entry_id(boma)] == false
        //     && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
        //         return false;
        //     }
        //     else {
        //         return ret;
        //     }
        // }
        if fighter_kind == *FIGHTER_KIND_GANON && entry_id(boma) < 8 {
            if CAN_TELEPORT[entry_id(boma)] == false
            && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
                return false;
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);
    
    // Fighter-Specific Param Edits

    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if param_hash == hash40("guard_off_cancel_frame") { // Shield Drop Cancel Frame
            if IS_FUNNY[entry_id(boma)] {
                return 5;
            }
            else {
                return ret;
            }
        }
    
        if param_hash == hash40("invalid_capture_frame") { // The Chain Grab Param
            if IS_FUNNY[entry_id(boma)] {
                return 1;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_RIDLEY && entry_id(boma) < 8 { // Funny Ridley
            if FUNNY_RIDLEY[entry_id(boma)] {
                if param_hash == hash40("max_charge_frame") {// Funny Plasma Breath
                    return 300;
                }
                if param_hash == hash40("max_fire_num") {
                    return 40;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_DAISY && entry_id(boma) < 8 {
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("uniq_float_float_frame") { // Give Daisy Float back if Funny
                    return 300;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_CLOUD && entry_id(boma) < 8 { // Limit Shenanigans
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("limit_break_clear_frame") {
                    return 6000;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        // if fighter_kind == *FIGHTER_KIND_WIIFIT && entry_id(boma) < 8 {
        //     if IS_FUNNY[entry_id(boma)] { // DRAGON INSTALL
        //         if param_hash == 0x202e02971b { // Unstaled Deep Breathing Timer hash
        //             return 420;
        //         }
        //         else {
        //             return ret;
        //         }
        //     }
        //     else {
        //         return ret;
        //     }
        // }
        if fighter_kind == *FIGHTER_KIND_REFLET {
            if param_hash == hash40("grimoire_el_window_revival_time") {
                return 480;
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_KEN && entry_id(boma) < 8 {
            if param_hash == hash40("loop_num_w")
            || param_hash == hash40("air_loop_num_w") {
                TATSULOOPS[entry_id(boma)][0] = 1;
                return 1;
            }
            if param_hash == hash40("loop_num_m")
            || param_hash == hash40("air_loop_num_m") {
                TATSULOOPS[entry_id(boma)][1] = 2;
                return 2;
            }
            if param_hash == hash40("loop_num_s")
            || param_hash == hash40("air_loop_num_s") {
                TATSULOOPS[entry_id(boma)][2] = 3;
                return 3;
            }
            if param_hash == hash40("fall_frame") {
                if SHORYUREPPA[entry_id(boma)] == 1 {
                    return 15;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_SHULK {
            if param_hash == hash40("circle_menu_release_after_interval_frame") {
                let status_kind = StatusModule::status_kind(boma);
                if (status_kind == *FIGHTER_STATUS_KIND_DAMAGE
                || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
                || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
                || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
                || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
                || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
                || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
                || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
                || status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL 
                || status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE) && WorkModule::is_enable_transition_term(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) == false {
                    let hitstun = (WorkModule::get_int(boma, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME) - WorkModule::get_int(boma, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME)) as f32;
                    if WorkModule::get_float(boma, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME) + hitstun < 40.0 {
                        return WorkModule::get_float(boma, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME) as i32;
                    }
                    else {
                        return (40.0 - hitstun) as i32;
                    }
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL { // Funny Mode Spirit Bomb Params
            if param_hash == hash40("life") {
                if IS_SPIRIT_BOMB[entry_id(boma)] {
                    return 6000;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *WEAPON_KIND_SAMUSD_CSHOT { // Phazon Orb Life
            if param_hash == hash40("life") {
                if IS_FUNNY[entry_id(boma)] {
                    return 6000;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *WEAPON_KIND_YOSHI_TAMAGO {
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("life") {
                    return 6000;
                }
                if param_hash == hash40("max_bound_num") {
                    return 100;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *WEAPON_KIND_ROCKMAN_ROCKBUSTER {
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("life") {
                    return 6000;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *WEAPON_KIND_ROCKMAN_CHARGESHOT {
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("life_min")
                || param_hash == hash40("life_max") {
                    return 6000;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *WEAPON_KIND_ROCKMAN_HARDKNUCKLE {
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("life") {
                    return 60;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *WEAPON_KIND_ROCKMAN_CRASHBOMB {
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("life") {
                    return 6000;
                }
                if param_hash == hash40("is_penetration") {
                    return 1;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
    let boma = &mut *(*((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(module_accessor, param_type, param_hash);
    let fighter_kind = utility::get_kind(boma);

    if (param_type == hash40("landing_attack_air_frame_n")
    || param_type == hash40("landing_attack_air_frame_f")
    || param_type == hash40("landing_attack_air_frame_b")
    || param_type == hash40("landing_attack_air_frame_hi")
    || param_type == hash40("landing_attack_air_frame_lw"))
    && entry_id(boma) < 8 {
        if AIR_WHIFF[entry_id(boma)] {
            return ret + 3.0;
        }
        else {
            return ret;
        }
    }
    
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

        if fighter_kind == *FIGHTER_KIND_CLOUD && entry_id(boma) < 8 { // Limit Shenanigans
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("lw2_spd_x_mul") {
                    return 0.6;
                }
                if param_hash == hash40("lw2_spd_y_mul") {
                    return 0.6;
                }
                if param_hash == hash40("lw2_accel_y") {
                    return 0.06;
                }
                if param_hash == hash40("lw2_speed_max_y") {
                    return 1.2;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_SHULK && entry_id(boma) < 8 {
            if IS_FUNNY[entry_id(boma)] {
                if param_hash == hash40("active_time_jump") {
                    return 100.0;
                }
                if param_hash == hash40("active_time_speed") {
                    return 100.0;
                }
                if param_hash == hash40("active_time_shield") {
                    return 100.0;
                }
                if param_hash == hash40("active_time_buster") {
                    return 100.0;
                }
                if param_hash == hash40("active_time_smash") {
                    return 100.0;
                }
                if param_hash == hash40("unavailable_time_jump") {
                    return 0.1;
                }
                if param_hash == hash40("unavailable_time_speed") {
                    return 0.1;
                }
                if param_hash == hash40("unavailable_time_shield") {
                    return 0.1;
                }
                if param_hash == hash40("unavailable_time_buster") {
                    return 0.1;
                }
                if param_hash == hash40("unavailable_time_smash") {
                    return 0.1;
                }
                if param_hash == hash40("shield_endure_mul") {
                    return 100.0;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_KEN && entry_id(boma) < 8 { // Shoryureppa
            if param_hash == hash40("endurance_min")
            || param_hash == hash40("endurance_max") {
                return 0.0;
            }
            if param_hash == hash40("defense_mul") {
                return 1.0;
            }
            if param_hash == hash40("speed_x_mul_s")
            || param_hash == hash40("hit_stop_frame_mul") {
                if SHORYUREPPA[entry_id(boma)] == 1 {
                    return 0.15;
                }
                else {
                    return ret;
                }
            }
            if param_hash == hash40("speed_y_mul_s") {
                if V_TRIGGER[entry_id(boma)]
                && SHORYUREPPA[entry_id(boma)] == 1 {
                    return 0.1;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *FIGHTER_KIND_WARIO && entry_id(boma) < 8 {
            if param_hash == hash40("gass_middle_time") {
                if FINISH_SIGN[entry_id(boma)] > 0 {
                    return 1.0;
                }
                else {
                    return ret;
                }
            }
            if param_hash == hash40("gass_large_time") {
                if FINISH_SIGN[entry_id(boma)] > 8 {
                    return 1.0;
                }
                else {
                    return ret;
                }
            }
            if param_hash == hash40("gass_max_time") {
                if FINISH_SIGN[entry_id(boma)] >= 15 {
                    return 1.0;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        // if fighter_kind == *FIGHTER_KIND_DOLLY && entry_id(boma) < 8 {
        //     if param_hash == hash40("super_special_damage") {
        //         if GO_SAUCE[entry_id(boma)] < 50.0 {
        //             return 999.0;
        //         }
        //         else {
        //             return 0.0;
        //         }
        //     }
        //     if param_hash == hash40("super_special_hp_rate") {
        //         if GO_SAUCE[entry_id(boma)] < 50.0 {
        //             return 0.0;
        //         }
        //         else {
        //             return 100.0;
        //         }
        //     }
        //     if param_hash == hash40("super_special_hp_min") {
        //         return 0.0;
        //     }
        //     if param_hash == hash40("super_special_hp_max") {
        //         if GO_SAUCE[entry_id(boma)] < 50.0 {
        //             return 0.0;
        //         }
        //         else {
        //             return 999.0;
        //         }
        //     }
        //     else {
        //         return ret;
        //     }
        // }
        // if fighter_kind == *FIGHTER_KIND_EDGE && entry_id(boma) < 8 {
        //     if ONE_WING[entry_id(boma)] == 900.0 {
        //         if param_hash == hash40("activate_point_init")
        //         || param_hash == hash40("activate_point_init_hp") {
        //             return 0.0;
        //         }
        //         else {
        //             return 999.0;
        //         }
        //     }
        //     else {
        //         return ret;
        //     }
        // }
        else {
            return ret;
        }
    }
    else if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
        if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL { // Funny Mode Spirit Bomb Params
            if param_hash == hash40("max_speed") {
                if IS_SPIRIT_BOMB[entry_id(boma)] {
                    return 0.4;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *WEAPON_KIND_ROCKMAN_ROCKBUSTER {
            if param_hash == hash40("speed") {
                if IS_FUNNY[entry_id(boma)] {
                    return 5.0;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        if fighter_kind == *WEAPON_KIND_ROCKMAN_CHARGESHOT {
            if param_hash == hash40("speed_min") {
                if IS_FUNNY[entry_id(boma)] {
                    return 5.5;
                }
                else {
                    return ret;
                }
            }
            if param_hash == hash40("speed_max") {
                if IS_FUNNY[entry_id(boma)] {
                    return 5.5;
                }
                else {
                    return ret;
                }
            }
            else {
                return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

// #[skyline::hook(offset = MUSIC_OFFSET)]
// pub fn music_function_replace(
//     param_1: *mut u64,
//     param_2: i64,
//     nus3bank_hash: u64,
//     nus3audio_hash: *mut u64,
//     nus3audio_index: usize,
// ) {
//     unsafe {
//         MUSIC_PARAM1 = param_1;
//         MUSIC_PARAM2 = param_2;
//         NUS3AUDIO_HASH = nus3audio_hash;
//     }
//     // if nus3bank_hash != hash40("bgm_crs2_03_shuuten")
//     // && nus3audio_hash != hash40("bgm_crs2_03_shuuten") {
//     //     for x in 0..7 {
//     //         if DRAGON_INSTALL[x] {
//     //             nus3bank_hash = hash40("bgm_crs2_03_shuuten");
//     //             nus3audio_hash = hash40("bgm_crs2_03_shuuten");
//     //             break;
//     //         }
//     //     }
//     // }
//     original!()(
//         param_1,
//         param_2,
//         nus3bank_hash,
//         nus3audio_hash,
//         nus3audio_index,
//     );
// }

#[skyline::hook(replace = WorkModule::get_int64 )]
pub unsafe fn get_int64_replace(boma: &mut BattleObjectModuleAccessor, term: i32) -> u64 {
    let ret = original!()(boma,term);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if entry_id(boma) < 8 {
            if utility::get_kind(boma) == *FIGHTER_KIND_LUCINA
            && term == *FIGHTER_STATUS_CATCH_WAIT_WORK_INT_MOTION_KIND {
                if HEROIC_GRAB[entry_id(boma)] {
                    return 0x8a0abc72cu64;
                }
            }
        }
    }
    return ret;
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::hook(replace = GroundModule::correct)]
pub unsafe fn correct_hook(boma: &mut BattleObjectModuleAccessor, mut param_2: u64) -> u64{

    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        let status_kind = StatusModule::status_kind(boma);
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_DASH].contains(&status_kind) {
            param_2 = *GROUND_CORRECT_KIND_GROUND as u64;
        }
    }
    original!()(boma, param_2)
}

// #[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
// pub unsafe fn init_settings_replace(
// boma: &mut app::BattleObjectModuleAccessor,
// situation_kind: i32,
// arg3: i32, 
// arg4: u64,
// ground_cliff_check_kind: u64,
// arg6: bool,
// arg7: i32,
// arg8: i32,
// arg9: i32,
// arg10: i32) -> u64 {
// 	let status_kind = StatusModule::status_kind(boma);
// 	let fighter_kind = app::utility::get_kind(boma);
//     let ret = original!()(boma, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
// 	if status_kind != *FIGHTER_STATUS_KIND_APPEAL
// 	&& status_kind != *FIGHTER_STATUS_KIND_DASH
// 	&& status_kind != *FIGHTER_STATUS_KIND_TURN
// 	&& status_kind != *FIGHTER_STATUS_KIND_TURN_DASH
// 	&& status_kind != *FIGHTER_STATUS_KIND_LANDING
// 	&& status_kind != *FIGHTER_STATUS_KIND_LANDING_LIGHT
// 	&& status_kind != *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
// 	&& status_kind != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
// 	&& status_kind != *FIGHTER_STATUS_KIND_ESCAPE_AIR
// 	&& fighter_kind != *FIGHTER_KIND_BUDDY {
// 		return ret;
// 	}
// 	else if status_kind == *FIGHTER_STATUS_KIND_APPEAL
// 	|| status_kind == *FIGHTER_STATUS_KIND_DASH
// 	|| status_kind == *FIGHTER_STATUS_KIND_TURN
// 	|| status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
// 	|| status_kind == *FIGHTER_STATUS_KIND_LANDING
// 	|| status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT
// 	|| status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
// 	|| status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
// 		return original!()(boma, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
// 	}
// 	else if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
// 		if ControlModule::get_stick_y(boma) >= 0.66 {
// 			return original!()(boma, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
// 		}
// 		else {
// 			return ret;
// 		}
// 	}
// 	else if fighter_kind == FIGHTER_KIND_BUDDY {
// 		if (status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S) && situation_kind == SITUATION_KIND_GROUND {
// 			return original!()(boma, situation_kind, arg3, 7 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10);
// 		}
// 		else {
// 			return ret;
// 		}
// 	}  
// 	else {
// 		return ret;
// 	}
// }

#[skyline::hook(replace = sv_animcmd::PLAY_SE)]
unsafe fn play_se_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1); //sound effect
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::PLAY_SEQUENCE)]
pub unsafe fn play_sequence_replace(lua_state: u64) -> u64 {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let seq = l2c_agent.pop_lua_stack(1); //sound effect
        l2c_agent.clear_lua_stack();
        let mut new_seq = seq.get_int();
        for i in 0..8 {
            if seq.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                new_seq = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq));
    }
    original!()(lua_state)
}

#[skyline::hook(replace = sv_animcmd::PLAY_FLY_VOICE)]
pub unsafe fn play_fly_voice_replace(lua_state: u64) -> u64 {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let seq = l2c_agent.pop_lua_stack(1); //sound effect
        let seq2 = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_seq = seq.get_int();
        let mut new_seq2 = seq2.get_int();
        for i in 0..8 {
            if seq.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                new_seq = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
            }
            if seq2.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                new_seq2 = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq2));
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq));
    }
    original!()(lua_state)
}

#[skyline::hook(replace = sv_animcmd::PLAY_STATUS)]
unsafe fn play_status_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
    && fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1); //sound effect
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::PLAY_DOWN_SE)]
unsafe fn play_down_se_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1); //sound effect
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::PLAY_SE_REMAIN)]
unsafe fn play_se_remain_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1); //sound effect
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::hook(replace = sv_animcmd::PLAY_SE_NO_3D)]
unsafe fn play_se_no_3d_replace(lua_state: u64) {
    let boma = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(boma);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(boma) {
        let mut l2c_agent = L2CAgent::new(lua_state);
        let se = l2c_agent.pop_lua_stack(1); //sound effect
        l2c_agent.clear_lua_stack();
        let mut new_se = se.get_int();
        for i in 0..36 {
            if se.get_int() == hash40(&("vc_lucina_".to_owned() + YU_AUDIO[i])) {
                new_se = hash40(&("vc_shadow_".to_owned() + YU_AUDIO[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
    }
    original!()(lua_state);
}

#[skyline::main(name = "the_wubor_patch")]
pub fn main() {
    unsafe{
        skyline::nn::ro::LookupSymbol(&mut FIGHTER_CUTIN_MANAGER_ADDR, c_str!("_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"));
        skyline::nn::ro::LookupSymbol(&mut ITEM_MANAGER, c_str!("_ZN3lib9SingletonIN3app11ItemManagerEE9instance_E"));
        skyline::nn::ro::LookupSymbol(&mut FIGHTER_MANAGER, c_str!("_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"));
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, NOTIFY_LOG_EVENT_COLLISION_HIT_SEARCH_CODE) {
            NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET = offset;
        }
        // if let Some(offset) = find_subsequence(text, INT64_SEARCH_CODE) {
        //     INT64_OFFSET = offset;
        // }
        // if let Some(offset) = find_subsequence(text, MUSIC_SEARCH_CODE) {
        //     MUSIC_OFFSET = offset;
        // }
    }
    
    system::install();
    daisy::install();
    samusd::install();
    lucina::install();
    littlemac::install();
    gaogaen::install();
    dedede::install();
    lucas::install();
    jack::install();
    kirby::install();
    cloud::install();
    lucario::install();
    bayonetta::install();
    dolly::install();
    shulk::install();
    pikachu::install();
    robot::install();
    snake::install();
    palutena::install();
    master::install();
    ryu::install();
    toonlink::install();
    zelda::install();
    buddy::install();
    ridley::install();
    koopajr::install();
    gamewatch::install();
    donkey::install();
    richter::install();
    eflame::install();
    elight::install();
    falco::install();
    // brave::install();
    purin::install();
    // wiifit::install();
    ken::install();
    metaknight::install();
    ganon::install();
    wario::install();
    luigi::install();
    reflet::install();
    miiswordsman::install();
    edge::install();
    pickel::install();
    pitb::install();
    diddy::install();
    sheik::install();
    tantan::install();
    yoshi::install();
    mario::install();
    duckhunt::install();
    rockman::install();
    kamui::install();
    samus::install();
    mariod::install();
    // skyline::install_hook!(get_command_flag_cat_replace);
    skyline::install_hook!(notify_log_event_collision_hit_replace);
    // skyline::install_hook!(attack_replace);
    skyline::install_hook!(is_enable_transition_term_replace);
    skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(get_param_int_replace);
    // skyline::install_hook!(music_function_replace);
    skyline::install_hook!(correct_hook);
    skyline::install_hook!(get_int64_replace);
    skyline::install_hook!(play_se_replace);
    skyline::install_hook!(play_fly_voice_replace);
    skyline::install_hook!(play_sequence_replace);
    skyline::install_hook!(play_status_replace);
    skyline::install_hook!(play_down_se_replace);
    skyline::install_hook!(play_se_remain_replace);
    skyline::install_hook!(play_se_no_3d_replace);
    // skyline::install_hook!(init_settings_replace);
}