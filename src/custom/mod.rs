//use smash::app::lua_bind::*;
//use smash::app::utility::get_kind;
//use smash::lib::lua_const::*;
//use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon/*, L2CFighterBase*/};
use acmd;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
//use smash::app::{self, lua_bind::*, *};

pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(_fighter : &mut L2CFighterCommon) {
    unsafe {
        LookupSymbol(
            &mut FIGHTER_CUTIN_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
                .as_bytes()
                .as_ptr(),
        );
    }
}

// Use this for general per-frame weapon-level hooks
//pub fn once_per_weapon_frame(fighter_base : &mut L2CFighterBase) {
//    unsafe {
//        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
//        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;
//
//        if frame % 10 == 0 {
//            println!("[Weapon Hook] Frame : {}", frame);
//        }
//    }
//}

pub fn install() {
    acmd::add_custom_hooks!(once_per_fighter_frame);
//    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}