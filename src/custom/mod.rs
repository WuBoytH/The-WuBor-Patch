//use smash::app::utility::get_kind;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
// use smash::lua2cpp::L2CAgentBase;
use smash::lua2cpp::{L2CFighterCommon/*, L2CFighterBase*/};
// use smash_script::*;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use crate::{IS_FUNNY, IS_FGC, COUNTER_HIT_STATE, COUNTER_HIT_HELPER};
use crate::commonfuncs::*;
use skyline::nn::ro::LookupSymbol;
use smash::app::{self, /***/};

pub static mut TIME_SLOW_EFFECT_VECTOR: smash::phx::Vector3f = smash::phx::Vector3f {x:-3.0,y:3.0,z:0.0};
//pub const TIME_SLOW_EFFECT_HASH: u64 = smash::hash40("sys_sp_flash");

// Use this for general per-frame fighter-level hooks
unsafe fn global_fighter_frame(_fighter : &mut L2CFighterCommon) {
    let lua_state = _fighter.lua_state_agent;
    let boma = smash::app::sv_system::battle_object_module_accessor(lua_state);
    let status_kind = StatusModule::status_kind(boma);

    LookupSymbol(
        &mut FIGHTER_CUTIN_MANAGER_ADDR,
        "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\u{0}"
            .as_bytes()
            .as_ptr(),
    );
    if ItemModule::is_attach_item(boma, app::ItemKind(*ITEM_KIND_USAGIHAT))
    && IS_FUNNY[get_player_number(boma)] == false {
        IS_FUNNY[get_player_number(boma)] = true;
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP) {
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_RABBIT_CAP);
    }
    // if !ItemModule::is_attach_item(boma, app::ItemKind(*ITEM_KIND_USAGIHAT))
    if (StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD || smash::app::sv_information::is_ready_go() == false)
    && IS_FUNNY[get_player_number(boma)] {
        IS_FUNNY[get_player_number(boma)] = false;
    }
    if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR)
    && IS_FGC[get_player_number(boma)] == false {
        IS_FGC[get_player_number(boma)] = true;
        println!("FGC is on!");
    }
    if IS_FGC[get_player_number(boma)] {
        if WorkModule::is_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR) {
            WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_HIT_REFLECTOR);
            println!("Disabled Badge Reflector!");
        }
    }
    if !ItemModule::is_attach_item(boma, app::ItemKind(*ITEM_KIND_BADGE))
    && IS_FGC[get_player_number(boma)] {
        IS_FGC[get_player_number(boma)] = false;
        println!("FGC is off!");
    }
    if IS_FUNNY[get_player_number(boma)] {
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL_SPECIAL {
            StatusModule::change_status_request_from_script(boma,*FIGHTER_STATUS_KIND_FALL_AERIAL,true);
        }
    }

    if status_kind == *FIGHTER_STATUS_KIND_ATTACK
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_START
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_START
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_START
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH
    || status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
        if MotionModule::frame(boma) <= 1.0 {
            COUNTER_HIT_HELPER[get_player_number(boma)] = 0.0;
            COUNTER_HIT_STATE[get_player_number(boma)] = 1;
        }
        if AttackModule::is_attack(boma, 0, false)
        && COUNTER_HIT_HELPER[get_player_number(boma)] == 0.0 {
            COUNTER_HIT_HELPER[get_player_number(boma)] = MotionModule::frame(boma);
        }
        if !AttackModule::is_attack(boma, 0, false)
        && COUNTER_HIT_HELPER[get_player_number(boma)] != 0.0 {
            COUNTER_HIT_STATE[get_player_number(boma)] = 0;
        }
    }
    else {
        COUNTER_HIT_STATE[get_player_number(boma)] = 0;
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
    smash_script::add_fighter_frame_callbacks!(global_fighter_frame);
//    acmd::add_custom_weapon_hooks!(once_per_weapon_frame);
}