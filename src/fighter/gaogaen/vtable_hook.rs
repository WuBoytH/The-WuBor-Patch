#![allow(non_snake_case)]

use {
    smash::{
        app::{lua_bind::*, *},
        lib::lua_const::*
    },
    custom_var::*,
    wubor_utils::{wua_bind::*, vars::*}
};

#[skyline::hook(offset = 0xab9950)]
pub unsafe extern "C" fn gaogaen_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) {
    let object = &mut fighter.battle_object;
    let module_accessor = (*object).module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let status = StatusModule::status_kind(module_accessor);
    if [
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_HIT,
        *FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_LW_TURN
    ].contains(&status)
    && (*collision_log).collision_kind == 1
    && VarModule::is_flag(object, gaogaen::status::flag::REVENGE_CRITICAL) {
        MiscModule::call_critical(module_accessor, log, 0x47, 0x18dfb2f4 | 0x1000000000, 1, 0, 0, 0);
        VarModule::off_flag(object, gaogaen::status::flag::REVENGE_CRITICAL);
        return;
    }
    original!()(vtable, fighter, log)
}

#[repr(C)]
pub struct CollisionLogScuffed {
    x00: *const u64,
    x08: *const u64,
    location: smash_rs::cpp::simd::Vector3,
    x20: u8,
    x21: u8,
    x22: u8,
    x23: u8,
    opponent_object_id: u32,
    x28: u8,
    x29: u8,
    x2A: u8,
    x2B: u8,
    x2C: u8,
    x2D: u8,
    x2E: u8,
    collision_kind: u8,
    receiver_part_id: u8,
    collider_part_id: u8,
    receiver_id: u8,
    collider_id: u8,
}

pub fn install() {
    skyline::install_hooks!(
        gaogaen_on_attack
    );
    // Skips to the end of the Revenge check after changing statuses
    skyline::patching::Patch::in_text(0xab9fd0).data(0x140000D1u32);
}