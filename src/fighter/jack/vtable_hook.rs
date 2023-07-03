#![allow(non_snake_case)]

use crate::imports::status_imports::*;

#[skyline::hook(offset = 0xb36ec0)]
pub unsafe extern "C" fn jack_damage_callback(_vtable: u64, _fighter: &mut Fighter, _event: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0xb36e80)]
pub unsafe extern "C" fn jack_damage_callback2(_vtable: u64, _fighter: &mut Fighter, _event: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0xb33d10)]
pub unsafe extern "C" fn jack_on_attack(vtable: u64, fighter: &mut Fighter, log: u64) {
    let module_accessor = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    if !WorkModule::is_flag(module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        FighterSpecializer_Jack::add_rebel_gauge(module_accessor, FighterEntryID(entry_id), (*collision_log).damage);
    }
    original!()(vtable, fighter, log)
}

#[repr(C)]
pub struct CollisionLogScuffed {
    x00: *const u64,
    x08: *const u64,
    location: smash_rs::cpp::simd::Vector3,
    damage: f32,
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
    // Disables passive meter gain
    skyline::patching::Patch::in_text(0xb31600).data(0x17FFFF6Eu32);
    // Disables automatically summoning Arsene
    skyline::patching::Patch::in_text(0xb3151c).data(0x14000035u32);
    skyline::patching::Patch::in_text(0xb30db4).data(0x14000031u32);
    skyline::install_hooks!(
        jack_damage_callback,
        jack_damage_callback2,
        jack_on_attack
    );
}