#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

// use smash::phx::*;
use smash::hash40;
use smash::lib::L2CValue;
use smash::lib::L2CAgent;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::app::*;
use smash::app::FighterManager;
use skyline::hooks::{getRegionAddress, Region};

pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut IS_FUNNY : [bool; 8] = [false; 8];
pub static mut IS_FGC : [bool; 8] = [false; 8];
static mut INT_OFFSET : usize = 0x4E19D0;
// static mut INT64_OFFSET : usize = 0x4E19F0;
static mut FLOAT_OFFSET : usize = 0x4E19D0;
static mut NOTIFY_LOG_EVENT_COLLISION_HIT_OFFSET : usize = 0x675A20;
static mut MUSIC_OFFSET: usize = 0x3451f30; // default = 8.1.0 offset
static mut MUSIC_PARAM1: *mut u64 = 0 as *mut u64;
static mut MUSIC_PARAM2: i64 = 0;
static mut NUS3AUDIO_HASH: *mut u64 = 0 as *mut u64;

static YU_AUDIO: [&'static str; 36] = ["appeal01", "appeal02", "attack01", "attack02", "attack03", "attack04", "attack05", "attack06", "attack07", "cliffcatch", "damage_twinkle", "damage01", "damage02", "damage03", "damagefly01", "damagefly02", "final", "furafura", "furasleep", "heavyget", "jump01", "missfoot01", "missfoot02", "ottotto", "passive", "special_h01", "special_l01", "special_l02", "special_n01", "swimup", "win01", "win02", "win03", "win_marth", "win_ike", "knockout"];
static YU_SEQ: [&'static str; 8] = ["attack", "special_n", "special_l", "special_h", "futtobi01", "futtobi02", "jump", "ottotto"];

static MUSIC_SEARCH_CODE: &[u8] = &[
    0xfc, 0x6f, 0xba, 0xa9, 0xfa, 0x67, 0x01, 0xa9, 0xf8, 0x5f, 0x02, 0xa9, 0xf6, 0x57, 0x03, 0xa9,
    0xf4, 0x4f, 0x04, 0xa9, 0xfd, 0x7b, 0x05, 0xa9, 0xfd, 0x43, 0x01, 0x91, 0xff, 0xc3, 0x1b, 0xd1,
    0xe8, 0x63, 0x05, 0x91,
];
static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];

// static INT64_SEARCH_CODE: &[u8] = &[
//     0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x15, 0x40, 0xf9,
// ];

macro_rules! c_str {
    ($l:tt) => {
        [$l.as_bytes(), "\u{0}".as_bytes()].concat().as_ptr();
    };
}

mod commonfuncs;
use crate::commonfuncs::*;
mod custom;
mod daisy;
mod samusd;
mod lucina;
use crate::lucina::{LUCINA_SPECIAL_AIR_S, shadow_id};
mod littlemac;
mod gaogaen;
mod dedede;
mod lucas;
mod jack;
mod kirby;
mod cloud;
mod lucario;
use crate::lucario::IS_SPIRIT_BOMB;
// mod bayonetta;
// mod dolly;
mod shulk;
use crate::shulk::SHULK_SPECIAL_LW;
mod pikachu;
mod robot;
mod snake;
mod palutena;
mod master;
mod ryu;
use crate::ryu::{SECRET_SENSATION, OPPONENT_X, OPPONENT_Y, OPPONENT_BOMA, CAMERA};
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
mod wiifit;
use crate::wiifit::CAN_DRAGON_INSTALL;
mod ken;
use crate::ken::QUICK_STEP_STATE;

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
    // let attacker_fighter_kind = sv_battle_object::kind(attacker_object_id);
    let defender_fighter_kind = sv_battle_object::kind(defender_object_id);
    // let a_entry_id = WorkModule::get_int(attacker_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let d_entry_id = WorkModule::get_int(defender_boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    // if IS_FUNNY[d_entry_id] {
    if defender_fighter_kind == *FIGHTER_KIND_RYU {
        if (MotionModule::motion_kind(defender_boma) == smash::hash40("appeal_hi_r")
        || MotionModule::motion_kind(defender_boma) == smash::hash40("appeal_hi_l"))
        && MotionModule::frame(defender_boma) <= 30.0
        && MotionModule::frame(defender_boma) >= 4.0 {
            if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
            || utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY {
                OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut smash::app::BattleObjectModuleAccessor) as u64;
                OPPONENT_X[d_entry_id] = PostureModule::pos_x(attacker_boma);
                OPPONENT_Y[d_entry_id] = PostureModule::pos_y(attacker_boma);
                if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    JostleModule::set_status(&mut *attacker_boma, false);
                }
            }
            else if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
                if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                    OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                    OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                    OPPONENT_BOMA[d_entry_id] = (&mut *defender_boma as *mut smash::app::BattleObjectModuleAccessor) as u64;
                }
                else {
                    OPPONENT_X[d_entry_id] = PostureModule::pos_x(oboma);
                    OPPONENT_Y[d_entry_id] = PostureModule::pos_y(oboma);
                    OPPONENT_BOMA[d_entry_id] = (&mut *oboma as *mut smash::app::BattleObjectModuleAccessor) as u64;
                    if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        JostleModule::set_status(&mut *oboma, false);
                    }
                }
            }
            else {
                OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                OPPONENT_BOMA[d_entry_id] = (&mut *defender_boma as *mut smash::app::BattleObjectModuleAccessor) as u64;
            }
            SECRET_SENSATION[d_entry_id] = true;
        }
    }
    // }
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

// #[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
// unsafe fn attack_replace(lua_state: u64) {
//     let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
//     let mut l2c_agent = L2CAgent::new(lua_state);
//     let fighter_kind = smash::app::utility::get_kind(module_accessor);
//     if fighter_kind == *FIGHTER_KIND_GAMEWATCH {
//         let hitbox_params: Vec<L2CValue> = (0..36).map(|i| l2c_agent.pop_lua_stack(i + 1)).collect();
//         l2c_agent.clear_lua_stack();
//         for i in 0..36 {
//             let mut x = hitbox_params[i];
//             if i == 15 {
                
//             }
//             if i == 16 {
//                 l2c_agent.push_lua_stack(&mut L2CValue::new_num(0.2));
//             }
//             else {
//                 l2c_agent.push_lua_stack(&mut x);
//             }
//         }
//     }
//     original!()(lua_state);
// }

#[skyline::hook(replace = smash::app::sv_animcmd::PLAY_SE)]
unsafe fn play_se_replace(lua_state: u64) {
    let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(module_accessor) {
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

#[skyline::hook(replace = smash::app::sv_animcmd::PLAY_SEQUENCE)]
pub unsafe fn play_sequence_replace(lua_state: u64) -> u64 {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(module_accessor) {
        let seq = l2c_agent.pop_lua_stack(1); //sound effect
        l2c_agent.clear_lua_stack();
        let mut new_seq = seq.get_int();
        println!("Hash to compare: {}", new_seq);
        for i in 0..8 {
            println!("Comparing hash to: {}", hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])));
            if seq.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                println!("Sequence is seq_lucina_rnd_{}", YU_SEQ[i]);
                new_seq = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
                break;
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq));
    }
    original!()(lua_state)
}

#[skyline::hook(replace = smash::app::sv_animcmd::PLAY_FLY_VOICE)]
pub unsafe fn play_fly_voice_replace(lua_state: u64) -> u64 {
    let mut l2c_agent = L2CAgent::new(lua_state);
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    let fighter_kind = utility::get_kind(module_accessor);
    if fighter_kind == *FIGHTER_KIND_LUCINA
    && shadow_id(module_accessor) {
        let seq = l2c_agent.pop_lua_stack(1); //sound effect
        let seq2 = l2c_agent.pop_lua_stack(1);
        l2c_agent.clear_lua_stack();
        let mut new_seq = seq.get_int();
        let mut new_seq2 = seq2.get_int();
        println!("Hash to compare: {}", new_seq);
        for i in 0..8 {
            println!("Comparing hash to: {}", hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])));
            if seq.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                println!("Sequence is seq_lucina_rnd_{}", YU_SEQ[i]);
                new_seq = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
            }
            if seq2.get_int() == hash40(&("seq_lucina_rnd_".to_owned() + YU_SEQ[i])) {
                println!("Sequence is seq_lucina_rnd_{}", YU_SEQ[i]);
                new_seq2 = hash40(&("seq_shadow_rnd_".to_owned() + YU_SEQ[i]));
            }
        }
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq2));
        l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq));
    }
    original!()(lua_state)
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,term);
    
    // Fighter-Specific Param Edits
    
    if fighter_kind == *FIGHTER_KIND_LUCINA && get_player_number(module_accessor) < 8 {
        if LUCINA_SPECIAL_AIR_S[get_player_number(module_accessor)] && IS_FUNNY[get_player_number(module_accessor)] == false {
            if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S { // Disable Lion's Leap if used once unless in Funny
                return false;
            }
            else {
                return ret;
            }
        }
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW { // No One More! Spam
            return false;
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_SHULK && get_player_number(module_accessor) < 8 {
        if SHULK_SPECIAL_LW[get_player_number(module_accessor)] && IS_FUNNY[get_player_number(module_accessor)] == false { // Disable Vision if used Burst and not in Funny
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
    if fighter_kind == *FIGHTER_KIND_RICHTER && get_player_number(module_accessor) < 8 {
        if RICHTER_SPECIAL_HI[get_player_number(module_accessor)] && IS_FUNNY[get_player_number(module_accessor)] == false { // Disable Multiple Up-Bs unless in Funny
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
    if fighter_kind == *FIGHTER_KIND_RYU && get_player_number(module_accessor) < 8 {
        if CAMERA[get_player_number(module_accessor)] {
            return false;
        }
        else {
            return ret;
        }
    }
    if fighter_kind == *FIGHTER_KIND_KEN && get_player_number(module_accessor) < 8 {
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
            return false;
        }
        if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_WALK
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_DASH
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_RUN
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_TURN_DASH
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_JUMP_START
        || term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT {
            if QUICK_STEP_STATE[get_player_number(module_accessor)] == 1 {
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
    if fighter_kind == *FIGHTER_KIND_WIIFIT && get_player_number(module_accessor) < 8 {
        if CAN_DRAGON_INSTALL[get_player_number(module_accessor)] == false
        && term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
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

#[skyline::hook(offset = INT_OFFSET)]
pub unsafe fn get_param_int_replace(boma: u64, param_type: u64, param_hash: u64) -> i32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);

    // Global Param Edits
    
    if param_hash == smash::hash40("guard_off_cancel_frame") { // Shield Drop Cancel Frame
        if IS_FUNNY[get_player_number(module_accessor)] {
            return 5;
        }
        else {
            return ret;
        }
    }

    if param_hash == smash::hash40("invalid_capture_frame") { // The Chain Grab Param
        if IS_FUNNY[get_player_number(module_accessor)] {
            return 1;
        }
        else {
            return ret;
        }
    }
    
    // Fighter-Specific Param Edits

    if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL { // Funny Mode Spirit Bomb Params
        let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let o_entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if param_hash == smash::hash40("life") {
            if IS_SPIRIT_BOMB[o_entry_id] {
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
        let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let o_entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if param_hash == smash::hash40("life") {
            if IS_FUNNY[o_entry_id] {
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
    if fighter_kind == *FIGHTER_KIND_RIDLEY && get_player_number(module_accessor) < 8 { // Funny Ridley
        if FUNNY_RIDLEY[get_player_number(module_accessor)] {
            if param_hash == smash::hash40("max_charge_frame") {// Funny Plasma Breath
                return 300;
            }
            if param_hash == smash::hash40("max_fire_num") {
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
    if fighter_kind == *FIGHTER_KIND_DAISY && get_player_number(module_accessor) < 8 {
        if IS_FUNNY[get_player_number(module_accessor)] {
            if param_hash == smash::hash40("uniq_float_float_frame") { // Give Daisy Float back if Funny
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
    if fighter_kind == *FIGHTER_KIND_CLOUD && get_player_number(module_accessor) < 8 { // Limit Shenanigans
        if IS_FUNNY[get_player_number(module_accessor)] {
            if param_hash == smash::hash40("limit_break_clear_frame") {
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
    if fighter_kind == *FIGHTER_KIND_WIIFIT && get_player_number(module_accessor) < 8 {
        if IS_FUNNY[get_player_number(module_accessor)] { // DRAGON INSTALL
            if param_hash == 0x202e02971b { // Unstaled Deep Breathing Timer hash
                return 420;
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
pub unsafe fn get_param_float_replace(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);

    // Universal Param Edits
    
    // Fighter-Specific Param Edits
    
    if fighter_kind == *WEAPON_KIND_LUCARIO_AURABALL { // Funny Mode Spirit Bomb Params
        let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
        let o_entry_id = WorkModule::get_int(oboma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if param_hash == smash::hash40("max_speed") {
            if IS_SPIRIT_BOMB[o_entry_id] {
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
    if fighter_kind == *FIGHTER_KIND_CLOUD && get_player_number(module_accessor) < 8 { // Limit Shenanigans
        if IS_FUNNY[get_player_number(module_accessor)] {
            if param_hash == smash::hash40("lw2_spd_x_mul") {
                return 0.6;
            }
            if param_hash == smash::hash40("lw2_spd_y_mul") {
                return 0.6;
            }
            if param_hash == smash::hash40("lw2_accel_y") {
                return 0.06;
            }
            if param_hash == smash::hash40("lw2_speed_max_y") {
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
    if fighter_kind == *FIGHTER_KIND_SHULK && get_player_number(module_accessor) < 8 {
        if IS_FUNNY[get_player_number(module_accessor)] {
            if param_hash == smash::hash40("active_time_jump") {
                return 100.0;
            }
            if param_hash == smash::hash40("active_time_speed") {
                return 100.0;
            }
            if param_hash == smash::hash40("active_time_shield") {
                return 100.0;
            }
            if param_hash == smash::hash40("active_time_buster") {
                return 100.0;
            }
            if param_hash == smash::hash40("active_time_smash") {
                return 100.0;
            }
            if param_hash == smash::hash40("unavailable_time_jump") {
                return 0.1;
            }
            if param_hash == smash::hash40("unavailable_time_speed") {
                return 0.1;
            }
            if param_hash == smash::hash40("unavailable_time_shield") {
                return 0.1;
            }
            if param_hash == smash::hash40("unavailable_time_buster") {
                return 0.1;
            }
            if param_hash == smash::hash40("unavailable_time_smash") {
                return 0.1;
            }
            if param_hash == smash::hash40("shield_endure_mul") {
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
    // if fighter_kind == *FIGHTER_KIND_KEN && get_player_number(module_accessor) < 8 { // FADC but Secret Sensation
    //     if QUICK_STEP_STATE[get_player_number(module_accessor)] == 1 {
    //         if param_hash == smash::hash40("run_speed_max") {
    //             return 2.0;
    //         }
    //         else {
    //             return ret;
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

#[skyline::hook(offset = MUSIC_OFFSET)]
pub fn music_function_replace(
    param_1: *mut u64,
    param_2: i64,
    nus3bank_hash: u64,
    nus3audio_hash: *mut u64,
    nus3audio_index: usize,
) {
    unsafe {
        println!("Param 1: {:#x}", &*param_1);
        println!("Param 2: {}", param_2);
        println!("Nus3bank Hash: {}", nus3bank_hash);
        println!("Nus3audio Raw: {:?}", nus3audio_hash);
        println!("Nus3audio Hash: {:#x}", &*nus3audio_hash);
        MUSIC_PARAM1 = param_1;
        MUSIC_PARAM2 = param_2;
        NUS3AUDIO_HASH = nus3audio_hash;
    }
    // if nus3bank_hash != smash::hash40("bgm_crs2_03_shuuten")
    // && nus3audio_hash != smash::hash40("bgm_crs2_03_shuuten") {
    //     for x in 0..7 {
    //         if DRAGON_INSTALL[x] {
    //             nus3bank_hash = smash::hash40("bgm_crs2_03_shuuten");
    //             nus3audio_hash = smash::hash40("bgm_crs2_03_shuuten");
    //             break;
    //         }
    //     }
    // }
    original!()(
        param_1,
        param_2,
        nus3bank_hash,
        nus3audio_hash,
        nus3audio_index,
    );
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::main(name = "the_bor_patch")]
pub fn main() {
    unsafe{
        skyline::nn::ro::LookupSymbol(&mut FIGHTER_CUTIN_MANAGER_ADDR, c_str!("_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"));
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }
        // if let Some(offset) = find_subsequence(text, INT64_SEARCH_CODE) {
        //     INT64_OFFSET = offset;
        // }
        if let Some(offset) = find_subsequence(text, MUSIC_SEARCH_CODE) {
            MUSIC_OFFSET = offset;
        }
    }
    custom::install();
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
    // bayonetta::install();
    // dolly::install();
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
    wiifit::install();
    ken::install();
    skyline::install_hook!(notify_log_event_collision_hit_replace);
    // skyline::install_hook!(attack_replace);
    skyline::install_hook!(play_se_replace);
    skyline::install_hook!(play_fly_voice_replace);
    skyline::install_hook!(play_sequence_replace);
    skyline::install_hook!(is_enable_transition_term_replace);
    skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(get_param_int_replace);
    skyline::install_hook!(music_function_replace);
}