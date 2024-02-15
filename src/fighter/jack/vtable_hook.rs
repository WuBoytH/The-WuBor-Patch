#![allow(non_snake_case)]

use crate::imports::status_imports::*;
use wubor_utils::app::*;

#[skyline::hook(offset = 0xb36ee0)]
pub unsafe extern "C" fn jack_damage_callback(_vtable: u64, _fighter: &mut Fighter, _event: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0xb36ea0)]
pub unsafe extern "C" fn jack_damage_callback2(_vtable: u64, _fighter: &mut Fighter, _event: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0x21b2960)]
pub unsafe extern "C" fn jack_damage_callback3(_stack: u64) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0xb34450)]
pub unsafe extern "C" fn jack_handle_gun_dodge_staling(_vtable: u64, _fighter: &mut Fighter) {
    // stub gaining rebel's gauge from getting hit?
}

#[skyline::hook(offset = 0x21b2b10)]
pub unsafe extern "C" fn jack_call_summon_dispatch(_stack: u64) {
    // stub gaining rebel's gauge from getting hit?
}

unsafe extern "C" fn jack_on_attack(vtable: u64, fighter: &mut Fighter, log: u64, damage: f32) {
    let module_accessor = fighter.battle_object.module_accessor;
    let collision_log = log as *mut CollisionLogScuffed;
    let collision_kind = (*collision_log).collision_kind;
    if [1, 2].contains(&collision_kind)
    && !WorkModule::is_flag(module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE_EXIST) {
        let mul = if collision_kind == 2 {
            0.1
        }
        else {
            1.0
        };
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID);
        FighterSpecializer_Jack::add_rebel_gauge(module_accessor, FighterEntryID(entry_id), damage * mul);
    }
    jack_on_attack_inner(vtable, fighter, log)
}

#[skyline::from_offset(0xb33d30)]
unsafe extern "C" fn jack_on_attack_inner(vtable: u64, fighter: &mut Fighter, log: u64);

pub fn install() {
    // Disables passive meter gain
    skyline::patching::Patch::in_text(0xb31620).data(0x17FFFF6Eu32);
    // Disables automatically summoning Arsene
    skyline::patching::Patch::in_text(0xb3153c).data(0x14000035u32);
    skyline::patching::Patch::in_text(0xb30dd4).data(0x14000031u32);
    skyline::install_hooks!(
        jack_damage_callback,
        jack_damage_callback2,
        jack_damage_callback3,
        jack_handle_gun_dodge_staling,
        jack_call_summon_dispatch
    );
    MiscModule::patch_vtable_function(0x4fc71b8, jack_on_attack as u64);
}