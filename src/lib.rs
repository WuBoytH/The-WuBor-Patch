#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

// use smash::lib::L2CValue;
// use smash::lib::L2CAgent;
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

mod custom;
mod daisy;
mod samusd;
mod lucina;
use crate::lucina::LUCINA_SPECIAL_AIR_S;
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
use crate::ryu::{SECRET_SENSATION, OPPONENT_X, OPPONENT_Y, OPPONENT_BOMA};
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
            println!("Is Ryu!");
            if (MotionModule::motion_kind(defender_boma) == smash::hash40("appeal_hi_r")
            || MotionModule::motion_kind(defender_boma) == smash::hash40("appeal_hi_l"))
            && MotionModule::frame(defender_boma) <= 30.0 {
                println!("Is 6Hing!");
                if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER
                || utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_ENEMY
                || utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_ITEM {
                    println!("Getting Fighter Pos!");
                    OPPONENT_X[d_entry_id] = PostureModule::pos_x(attacker_boma);
                    OPPONENT_Y[d_entry_id] = PostureModule::pos_y(attacker_boma);
                    if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        println!("Setting jostle for Fighter!");
                        JostleModule::set_status(&mut *attacker_boma, false);
                    }
                    OPPONENT_BOMA[d_entry_id] = (&mut *attacker_boma as *mut smash::app::BattleObjectModuleAccessor) as u64;
                }
                else if utility::get_category(&mut *attacker_boma) == *BATTLE_OBJECT_CATEGORY_WEAPON {
                    println!("Is weapon! Getting Weapon Owner Boma!");
                    let oboma = smash::app::sv_battle_object::module_accessor((WorkModule::get_int(attacker_boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
                    println!("Checking for if Fighter!");
                    if utility::get_category(&mut *oboma) != *BATTLE_OBJECT_CATEGORY_FIGHTER {
                        println!("Isn't fighter! Setting Ryu Pos...");
                        OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                        OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                    }
                    else {
                        println!("Is fighter! Getting Pos...");
                        OPPONENT_X[d_entry_id] = PostureModule::pos_x(oboma);
                        OPPONENT_Y[d_entry_id] = PostureModule::pos_y(oboma);
                        OPPONENT_BOMA[d_entry_id] = (&mut *oboma as *mut smash::app::BattleObjectModuleAccessor) as u64;
                        if utility::get_category(&mut *oboma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
                            println!("Setting jostle for Fighter!");
                            JostleModule::set_status(&mut *attacker_boma, false);
                        }
                    }
                }
                else {
                    println!("Isn't Fighter or Weapon! Getting Pos...");
                    OPPONENT_X[d_entry_id] = PostureModule::pos_x(defender_boma);
                    OPPONENT_Y[d_entry_id] = PostureModule::pos_y(defender_boma);
                }
                println!("Secret Sensation is true! We're done!");
                SECRET_SENSATION[d_entry_id] = true;
            }
        }
    // }
    original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

// #[skyline::hook(replace = smash::app::sv_animcmd::ATTACK)]
// unsafe fn attack_replace(lua_state: u64) {
//     let module_accessor = smash::app::sv_system::battle_object_module_accessor(lua_state);
//     let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
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

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term )]
pub unsafe fn is_enable_transition_term_replace(module_accessor: &mut BattleObjectModuleAccessor, term: i32) -> bool {
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let ret = original!()(module_accessor,term);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    // Fighter-Specific Param Edits
    
    if fighter_kind == *FIGHTER_KIND_LUCINA && entry_id < 8 {
        if LUCINA_SPECIAL_AIR_S[entry_id] && IS_FUNNY[entry_id] == false {
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
    if fighter_kind == *FIGHTER_KIND_SHULK && entry_id < 8 {
        if SHULK_SPECIAL_LW[entry_id] && IS_FUNNY[entry_id] == false { // Disable Vision if used Burst and not in Funny
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
    if fighter_kind == *FIGHTER_KIND_RICHTER && entry_id < 8 {
        if RICHTER_SPECIAL_HI[entry_id] && IS_FUNNY[entry_id] == false { // Disable Multiple Up-Bs unless in Funny
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
    if fighter_kind == *FIGHTER_KIND_RYU && entry_id < 8 {
        if SECRET_SENSATION[entry_id] {
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
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = smash::app::utility::get_kind(module_accessor);

    // Global Param Edits
    
    if param_hash == smash::hash40("guard_off_cancel_frame") { // Shield Drop Cancel Frame
        if IS_FUNNY[entry_id] {
            return 5;
        }
        else {
            return ret;
        }
    }

    if param_hash == smash::hash40("invalid_capture_frame") { // The Chain Grab Param
        if IS_FUNNY[entry_id] {
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
    if fighter_kind == *FIGHTER_KIND_RIDLEY && entry_id < 8 { // Funny Ridley
        if FUNNY_RIDLEY[entry_id] {
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
    if fighter_kind == *FIGHTER_KIND_DAISY && entry_id < 8 {
        if IS_FUNNY[entry_id] {
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
    if fighter_kind == *FIGHTER_KIND_CLOUD && entry_id < 8 { // Limit Shenanigans
        if IS_FUNNY[entry_id] {
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
    else {
        return ret;
    }
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    
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
    if fighter_kind == *FIGHTER_KIND_CLOUD && entry_id < 8 { // Limit Shenanigans
        if IS_FUNNY[entry_id] {
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
    if fighter_kind == *FIGHTER_KIND_SHULK && entry_id < 8 {
        if IS_FUNNY[entry_id] {
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
    else {
        return ret;
    }
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
    skyline::install_hook!(notify_log_event_collision_hit_replace);
    // skyline::install_hook!(attack_replace);
    skyline::install_hook!(is_enable_transition_term_replace);
    skyline::install_hook!(get_param_float_replace);
    skyline::install_hook!(get_param_int_replace);
}