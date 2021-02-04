use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use acmd;

// Use this for general per-frame fighter-level hooks
pub fn once_per_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        if fighter_kind == *FIGHTER_KIND_GAOGAEN {
            if (Frame>=20) {
                if *FIGHTER_GAOGAEN_STATUS_SPECIAL_N_FLAG_ROTATION && (AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
                    if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                        StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                      }
                }
            }
        }
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